// This file is an adaptation of
// https://github.com/deislabs/wasi-nn-onnx/blob/main/crates/wasi-nn-onnx-wasmtime/src/tract.rs
// to the new WIT format.

mod bytes;

use crate::{bytes::f32_vec_to_bytes, wasi_nn::TensorType};
use bytes::bytes_to_f32_vec;
use ndarray::Array;
use std::{
    collections::{btree_map::Keys, BTreeMap},
    io::Cursor,
    sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard},
};
use tract_onnx::{
    prelude::Graph as TractGraph, prelude::Tensor as TractTensor, prelude::*,
    tract_hir::infer::InferenceOp,
};
use wasi_nn::{ExecutionTarget, GraphBuilderArray, GraphEncoding, TensorParam, TensorResult};

wit_bindgen_wasmtime::export!("wit/ephemeral/wasi-nn.wit");

#[derive(Debug)]
pub struct TractSession {
    pub graph: TractGraph<InferenceFact, Box<dyn InferenceOp>>,
    pub input_tensors: Option<Vec<TractTensor>>,
    pub output_tensors: Option<Vec<Arc<TractTensor>>>,
}

impl TractSession {
    pub fn with_graph(graph: TractGraph<InferenceFact, Box<dyn InferenceOp>>) -> Self {
        Self {
            graph,
            input_tensors: None,
            output_tensors: None,
        }
    }
}

/// Main context struct for which we implement the WasiNn trait.
#[derive(Default)]
pub struct WasiNnTractCtx {
    pub state: Arc<RwLock<State>>,
}

// TODO (@radu-matei)
//
// Currently, the `State` object is only able to hold Tract ONNX related state.
// Ideally, it could be able to hold and compute inferences for multiple implementations.
#[derive(Default)]
pub struct State {
    pub executions: BTreeMap<GraphExecutionContext, TractSession>,
    pub models: BTreeMap<Graph, Vec<u8>>,
}

impl State {
    /// Helper function that returns the key that is supposed to be inserted next.
    pub fn key<K: Into<u32> + From<u32> + Copy, V>(&self, keys: Keys<K, V>) -> K {
        match keys.last() {
            Some(&k) => {
                let last: u32 = k.into();
                K::from(last + 1)
            }
            None => K::from(0),
        }
    }
}

type Graph = u32;
type GraphExecutionContext = u32;

impl wasi_nn::WasiNn for WasiNnTractCtx {
    type Graph = Graph;
    type GraphExecutionContext = GraphExecutionContext;

    fn load(
        &mut self,
        builder: GraphBuilderArray<'_>,
        encoding: GraphEncoding,
        target: ExecutionTarget,
    ) -> Result<Self::Graph, wasi_nn::Error> {
        log::info!("load: encoding: {:?}, target: {:?}", encoding, target);
        if encoding != GraphEncoding::Onnx {
            log::error!("load current implementation can only load ONNX models");
            return Err(wasi_nn::Error::InvalidEncoding);
        }

        let model_bytes = builder[0];
        let mut state = self.state.write()?;
        let graph = state.key(state.models.keys());
        log::info!(
            "load: inserting graph: {:#?} with size {:#?}",
            graph,
            model_bytes.len()
        );
        state.models.insert(graph, model_bytes.to_vec());
        log::info!("load: current number of models: {:#?}", state.models.len());
        Ok(graph)
    }

    fn init_execution_context(
        &mut self,
        graph: &Self::Graph,
    ) -> Result<Self::GraphExecutionContext, wasi_nn::Error> {
        log::info!("init_execution_context: graph: {:#?}", graph);
        let mut state = self.state.write()?;
        let mut model_bytes = match state.models.get(&graph) {
            Some(mb) => Cursor::new(mb),
            None => {
                log::error!(
                    "init_execution_context: cannot find model in state with graph {:#?}",
                    graph
                );
                return Err(wasi_nn::Error::RuntimeError);
            }
        };

        let model = tract_onnx::onnx().model_for_read(&mut model_bytes).unwrap();

        let gec = state.key(state.executions.keys());
        log::info!(
            "init_execution_context: inserting graph execution context: {:#?}",
            gec
        );

        state
            .executions
            .insert(gec, TractSession::with_graph(model));

        Ok(gec)
    }

    fn set_input(
        &mut self,
        ctx: &Self::GraphExecutionContext,
        index: u32,
        tensor: TensorParam<'_>,
    ) -> Result<(), wasi_nn::Error> {
        let mut state = self.state.write()?;
        let execution = match state.executions.get_mut(&ctx) {
            Some(s) => s,
            None => {
                log::error!(
                    "set_input: cannot find session in state with context {:#?}",
                    ctx
                );

                return Err(wasi_nn::Error::RuntimeError);
            }
        };

        let shape = tensor
            .dimensions
            .iter()
            .map(|d| d.get() as usize)
            .collect::<Vec<_>>();

        execution.graph.set_input_fact(
            index as usize,
            InferenceFact::dt_shape(f32::datum_type(), shape.clone()),
        )?;

        let data = bytes_to_f32_vec(tensor.data.to_vec())?;
        let input: TractTensor = Array::from_shape_vec(shape, data)?.into();

        match execution.input_tensors {
            Some(ref mut input_arrays) => {
                input_arrays.push(input);
                log::info!(
                    "set_input: input arrays now contains {} items",
                    input_arrays.len(),
                );
            }
            None => {
                execution.input_tensors = Some(vec![input]);
            }
        };

        Ok(())
    }

    fn compute(&mut self, ctx: &Self::GraphExecutionContext) -> Result<(), wasi_nn::Error> {
        let mut state = self.state.write()?;
        let mut execution = match state.executions.get_mut(&ctx) {
            Some(s) => s,
            None => {
                log::error!(
                    "compute: cannot find session in state with context {:#?}",
                    ctx
                );

                return Err(wasi_nn::Error::RuntimeError);
            }
        };
        // TODO
        //
        // There are two `.clone()` calls here that could prove
        // to be *very* inneficient, one in getting the input tensors,
        // the other in making the model runnable.
        let input_tensors: Vec<TractTensor> = execution
            .input_tensors
            .as_ref()
            .unwrap_or(&vec![])
            .clone()
            .into_iter()
            .collect();

        log::info!(
            "compute: input tensors contains {} elements",
            input_tensors.len()
        );

        // Some ONNX models don't specify their input tensor
        // shapes completely, so we can only call `.into_optimized()` after we
        // have set the input tensor shapes.
        let output_tensors = execution
            .graph
            .clone()
            .into_optimized()?
            .into_runnable()?
            .run(input_tensors.into())?;

        log::info!(
            "compute: output tensors contains {} elements",
            output_tensors.len()
        );

        match execution.output_tensors {
            Some(_) => {
                log::error!("compute: existing data in output_tensors, aborting");
                return Err(wasi_nn::Error::RuntimeError);
            }
            None => {
                execution.output_tensors = Some(output_tensors.into_iter().collect());
            }
        };

        Ok(())
    }

    fn get_output(
        &mut self,
        ctx: &Self::GraphExecutionContext,
        index: u32,
    ) -> Result<TensorResult, wasi_nn::Error> {
        let state = self.state.read()?;
        let execution = match state.executions.get(&ctx) {
            Some(s) => s,
            None => {
                log::error!(
                    "compute: cannot find session in state with context {:#?}",
                    ctx
                );

                return Err(wasi_nn::Error::RuntimeError);
            }
        };

        let output_tensors = match execution.output_tensors {
            Some(ref oa) => oa,
            None => {
                log::error!("get_output: output_tensors for session is none. Perhaps you haven't called compute yet?");
                return Err(wasi_nn::Error::RuntimeError);
            }
        };

        let tensor = match output_tensors.get(index as usize) {
            Some(a) => a,
            None => {
                log::error!(
                    "get_output: output_tensors does not contain index {}",
                    index
                );
                return Err(wasi_nn::Error::RuntimeError);
            }
        };

        let dimensions: Vec<_> = tensor.shape().iter().map(|s| *s as u32).collect();
        let tensor_type = TensorType::Fp32;
        let data = f32_vec_to_bytes(tensor.as_slice().unwrap().to_vec());

        let res = TensorResult {
            dimensions,
            tensor_type,
            data,
        };

        Ok(res)
    }
}

impl From<PoisonError<RwLockWriteGuard<'_, State>>> for wasi_nn::Error {
    fn from(_: PoisonError<RwLockWriteGuard<'_, State>>) -> Self {
        Self::RuntimeError
    }
}

impl From<PoisonError<RwLockReadGuard<'_, State>>> for wasi_nn::Error {
    fn from(_: PoisonError<RwLockReadGuard<'_, State>>) -> Self {
        Self::RuntimeError
    }
}

impl From<anyhow::Error> for wasi_nn::Error {
    fn from(_: anyhow::Error) -> Self {
        Self::RuntimeError
    }
}

impl From<tract_ndarray::ShapeError> for wasi_nn::Error {
    fn from(_: tract_ndarray::ShapeError) -> Self {
        Self::RuntimeError
    }
}
