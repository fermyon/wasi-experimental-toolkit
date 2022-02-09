pub mod wasi_mongo {
  #[allow(unused_imports)]
  use wit_bindgen_wasmtime::{wasmtime, anyhow};
  /// General purpose error.
  #[repr(u8)]
  #[derive(Clone, Copy, PartialEq, Eq)]
  pub enum Error{
    Success,
    Error,
  }
  impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Error::Success => {
          f.debug_tuple("Error::Success").finish()
        }
        Error::Error => {
          f.debug_tuple("Error::Error").finish()
        }
      }
    }
  }
  pub type DocumentParam<'a,> = &'a [u8];
  pub type DocumentResult = Vec<u8>;
  pub trait WasiMongo: Sized {
    /// Create operations
    /// Insert a document into a collection
    fn insert_one(&mut self,db: & str,collection: & str,doc: DocumentParam<'_,>,) -> Result<(),Error>;
    
    /// Insert many documents into a collection
    fn insert_many(&mut self,db: & str,collection: & str,docs: Vec<DocumentParam<'_,>>,) -> Result<(),Error>;
    
    /// Read operations
    /// Get a list of documents from the collection
    /// TODO (nitishm): Add support for query params
    fn find(&mut self,db: & str,collection: & str,filter: DocumentParam<'_,>,) -> Result<Vec<DocumentResult>,Error>;
    
    /// Update operations
    /// Update one document in the collection
    fn update_one(&mut self,db: & str,collection: & str,filter: DocumentParam<'_,>,doc: DocumentParam<'_,>,) -> Result<(),Error>;
    
    /// Update many documents in the collection
    fn update_many(&mut self,db: & str,collection: & str,filter: DocumentParam<'_,>,docs: Vec<DocumentParam<'_,>>,) -> Result<(),Error>;
    
    /// Replace one document in the collection
    fn replace_one(&mut self,db: & str,collection: & str,filter: DocumentParam<'_,>,doc: DocumentParam<'_,>,) -> Result<(),Error>;
    
    /// Delete operations
    /// Delete a document from the collection
    fn delete_one(&mut self,db: & str,collection: & str,filter: DocumentParam<'_,>,) -> Result<(),Error>;
    
    /// Delete many documents from the collection
    fn delete_many(&mut self,db: & str,collection: & str,filter: DocumentParam<'_,>,) -> Result<(),Error>;
    
    /// Drop operations
    /// Drop the database
    fn drop_database(&mut self,db: & str,) -> Result<(),Error>;
    
  }
  
  pub fn add_to_linker<T, U>(linker: &mut wasmtime::Linker<T>, get: impl Fn(&mut T) -> &mut U+ Send + Sync + Copy + 'static) -> anyhow::Result<()> 
  where U: WasiMongo
  {
    use wit_bindgen_wasmtime::rt::get_memory;
    use wit_bindgen_wasmtime::rt::get_func;
    linker.func_wrap("wasi-mongo", "insert-one", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32,arg2:i32,arg3:i32,arg4:i32,arg5:i32,arg6:i32| {
      let memory = &get_memory(&mut caller, "memory")?;
      let (mem, data) = memory.data_and_store_mut(&mut caller);
      let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
      let host = get(data);
      let ptr0 = arg0;
      let len0 = arg1;
      let ptr1 = arg2;
      let len1 = arg3;
      let ptr2 = arg4;
      let len2 = arg5;
      let param0 = _bc.slice_str(ptr0, len0)?;
      let param1 = _bc.slice_str(ptr1, len1)?;
      let param2 = _bc.slice(ptr2, len2)?;
      let result3 = host.insert_one(param0, param1, param2, );
      let (result4_0,result4_1,) = match result3{
        Ok(()) => { (0i32, 0i32)}
        Err(e) => { (1i32, e as i32)}
      };
      let caller_memory = memory.data_mut(&mut caller);
      caller_memory.store(arg6 + 8, wit_bindgen_wasmtime::rt::as_i32(result4_1))?;
      caller_memory.store(arg6 + 0, wit_bindgen_wasmtime::rt::as_i32(result4_0))?;
      Ok(())
    })?;
    linker.func_wrap("wasi-mongo", "insert-many", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32,arg2:i32,arg3:i32,arg4:i32,arg5:i32,arg6:i32| {
      let memory = &get_memory(&mut caller, "memory")?;
      let (mem, data) = memory.data_and_store_mut(&mut caller);
      let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
      let host = get(data);
      let ptr0 = arg0;
      let len0 = arg1;
      let ptr1 = arg2;
      let len1 = arg3;
      let len5 = arg5;
      let base5 = arg4;
      let mut result5 = Vec::with_capacity(len5 as usize);
      for i in 0..len5 {
        let base = base5 + i *8;
        result5.push({
          let load2 = _bc.load::<i32>(base + 0)?;
          let load3 = _bc.load::<i32>(base + 4)?;
          let ptr4 = load2;
          let len4 = load3;
          _bc.slice(ptr4, len4)?
        });
      }
      let param0 = _bc.slice_str(ptr0, len0)?;
      let param1 = _bc.slice_str(ptr1, len1)?;
      let param2 = result5;
      let result6 = host.insert_many(param0, param1, param2, );
      let (result7_0,result7_1,) = match result6{
        Ok(()) => { (0i32, 0i32)}
        Err(e) => { (1i32, e as i32)}
      };
      let caller_memory = memory.data_mut(&mut caller);
      caller_memory.store(arg6 + 8, wit_bindgen_wasmtime::rt::as_i32(result7_1))?;
      caller_memory.store(arg6 + 0, wit_bindgen_wasmtime::rt::as_i32(result7_0))?;
      Ok(())
    })?;
    linker.func_wrap("wasi-mongo", "find", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32,arg2:i32,arg3:i32,arg4:i32,arg5:i32,arg6:i32| {
      
      let func = get_func(&mut caller, "canonical_abi_realloc")?;
      let func_canonical_abi_realloc = func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
      let memory = &get_memory(&mut caller, "memory")?;
      let (mem, data) = memory.data_and_store_mut(&mut caller);
      let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
      let host = get(data);
      let ptr0 = arg0;
      let len0 = arg1;
      let ptr1 = arg2;
      let len1 = arg3;
      let ptr2 = arg4;
      let len2 = arg5;
      let param0 = _bc.slice_str(ptr0, len0)?;
      let param1 = _bc.slice_str(ptr1, len1)?;
      let param2 = _bc.slice(ptr2, len2)?;
      let result3 = host.find(param0, param1, param2, );
      let (result6_0,result6_1,result6_2,) = match result3{
        Ok(e) => { {
          let vec5 = e;
          let len5 = vec5.len() as i32;
          let result5 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 4, len5 * 8))?;
          for (i, e) in vec5.into_iter().enumerate() {
            let base = result5 + (i as i32) * 8;
            {
              let vec4 = e;
              let ptr4 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec4.len() as i32) * 1))?;
              let caller_memory = memory.data_mut(&mut caller);
              caller_memory.store_many(ptr4, vec4.as_ref())?;
              caller_memory.store(base + 4, wit_bindgen_wasmtime::rt::as_i32(vec4.len() as i32))?;
              caller_memory.store(base + 0, wit_bindgen_wasmtime::rt::as_i32(ptr4))?;
            }}(0i32, result5, len5)
          }}
          Err(e) => { (1i32, e as i32, 0i32)}
        };
        let caller_memory = memory.data_mut(&mut caller);
        caller_memory.store(arg6 + 16, wit_bindgen_wasmtime::rt::as_i32(result6_2))?;
        caller_memory.store(arg6 + 8, wit_bindgen_wasmtime::rt::as_i32(result6_1))?;
        caller_memory.store(arg6 + 0, wit_bindgen_wasmtime::rt::as_i32(result6_0))?;
        Ok(())
      })?;
      linker.func_wrap("wasi-mongo", "update-one", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32,arg2:i32,arg3:i32,arg4:i32,arg5:i32,arg6:i32,arg7:i32,arg8:i32| {
        let memory = &get_memory(&mut caller, "memory")?;
        let (mem, data) = memory.data_and_store_mut(&mut caller);
        let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
        let host = get(data);
        let ptr0 = arg0;
        let len0 = arg1;
        let ptr1 = arg2;
        let len1 = arg3;
        let ptr2 = arg4;
        let len2 = arg5;
        let ptr3 = arg6;
        let len3 = arg7;
        let param0 = _bc.slice_str(ptr0, len0)?;
        let param1 = _bc.slice_str(ptr1, len1)?;
        let param2 = _bc.slice(ptr2, len2)?;
        let param3 = _bc.slice(ptr3, len3)?;
        let result4 = host.update_one(param0, param1, param2, param3, );
        let (result5_0,result5_1,) = match result4{
          Ok(()) => { (0i32, 0i32)}
          Err(e) => { (1i32, e as i32)}
        };
        let caller_memory = memory.data_mut(&mut caller);
        caller_memory.store(arg8 + 8, wit_bindgen_wasmtime::rt::as_i32(result5_1))?;
        caller_memory.store(arg8 + 0, wit_bindgen_wasmtime::rt::as_i32(result5_0))?;
        Ok(())
      })?;
      linker.func_wrap("wasi-mongo", "update-many", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32,arg2:i32,arg3:i32,arg4:i32,arg5:i32,arg6:i32,arg7:i32,arg8:i32| {
        let memory = &get_memory(&mut caller, "memory")?;
        let (mem, data) = memory.data_and_store_mut(&mut caller);
        let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
        let host = get(data);
        let ptr0 = arg0;
        let len0 = arg1;
        let ptr1 = arg2;
        let len1 = arg3;
        let ptr2 = arg4;
        let len2 = arg5;
        let len6 = arg7;
        let base6 = arg6;
        let mut result6 = Vec::with_capacity(len6 as usize);
        for i in 0..len6 {
          let base = base6 + i *8;
          result6.push({
            let load3 = _bc.load::<i32>(base + 0)?;
            let load4 = _bc.load::<i32>(base + 4)?;
            let ptr5 = load3;
            let len5 = load4;
            _bc.slice(ptr5, len5)?
          });
        }
        let param0 = _bc.slice_str(ptr0, len0)?;
        let param1 = _bc.slice_str(ptr1, len1)?;
        let param2 = _bc.slice(ptr2, len2)?;
        let param3 = result6;
        let result7 = host.update_many(param0, param1, param2, param3, );
        let (result8_0,result8_1,) = match result7{
          Ok(()) => { (0i32, 0i32)}
          Err(e) => { (1i32, e as i32)}
        };
        let caller_memory = memory.data_mut(&mut caller);
        caller_memory.store(arg8 + 8, wit_bindgen_wasmtime::rt::as_i32(result8_1))?;
        caller_memory.store(arg8 + 0, wit_bindgen_wasmtime::rt::as_i32(result8_0))?;
        Ok(())
      })?;
      linker.func_wrap("wasi-mongo", "replace-one", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32,arg2:i32,arg3:i32,arg4:i32,arg5:i32,arg6:i32,arg7:i32,arg8:i32| {
        let memory = &get_memory(&mut caller, "memory")?;
        let (mem, data) = memory.data_and_store_mut(&mut caller);
        let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
        let host = get(data);
        let ptr0 = arg0;
        let len0 = arg1;
        let ptr1 = arg2;
        let len1 = arg3;
        let ptr2 = arg4;
        let len2 = arg5;
        let ptr3 = arg6;
        let len3 = arg7;
        let param0 = _bc.slice_str(ptr0, len0)?;
        let param1 = _bc.slice_str(ptr1, len1)?;
        let param2 = _bc.slice(ptr2, len2)?;
        let param3 = _bc.slice(ptr3, len3)?;
        let result4 = host.replace_one(param0, param1, param2, param3, );
        let (result5_0,result5_1,) = match result4{
          Ok(()) => { (0i32, 0i32)}
          Err(e) => { (1i32, e as i32)}
        };
        let caller_memory = memory.data_mut(&mut caller);
        caller_memory.store(arg8 + 8, wit_bindgen_wasmtime::rt::as_i32(result5_1))?;
        caller_memory.store(arg8 + 0, wit_bindgen_wasmtime::rt::as_i32(result5_0))?;
        Ok(())
      })?;
      linker.func_wrap("wasi-mongo", "delete-one", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32,arg2:i32,arg3:i32,arg4:i32,arg5:i32,arg6:i32| {
        let memory = &get_memory(&mut caller, "memory")?;
        let (mem, data) = memory.data_and_store_mut(&mut caller);
        let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
        let host = get(data);
        let ptr0 = arg0;
        let len0 = arg1;
        let ptr1 = arg2;
        let len1 = arg3;
        let ptr2 = arg4;
        let len2 = arg5;
        let param0 = _bc.slice_str(ptr0, len0)?;
        let param1 = _bc.slice_str(ptr1, len1)?;
        let param2 = _bc.slice(ptr2, len2)?;
        let result3 = host.delete_one(param0, param1, param2, );
        let (result4_0,result4_1,) = match result3{
          Ok(()) => { (0i32, 0i32)}
          Err(e) => { (1i32, e as i32)}
        };
        let caller_memory = memory.data_mut(&mut caller);
        caller_memory.store(arg6 + 8, wit_bindgen_wasmtime::rt::as_i32(result4_1))?;
        caller_memory.store(arg6 + 0, wit_bindgen_wasmtime::rt::as_i32(result4_0))?;
        Ok(())
      })?;
      linker.func_wrap("wasi-mongo", "delete-many", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32,arg2:i32,arg3:i32,arg4:i32,arg5:i32,arg6:i32| {
        let memory = &get_memory(&mut caller, "memory")?;
        let (mem, data) = memory.data_and_store_mut(&mut caller);
        let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
        let host = get(data);
        let ptr0 = arg0;
        let len0 = arg1;
        let ptr1 = arg2;
        let len1 = arg3;
        let ptr2 = arg4;
        let len2 = arg5;
        let param0 = _bc.slice_str(ptr0, len0)?;
        let param1 = _bc.slice_str(ptr1, len1)?;
        let param2 = _bc.slice(ptr2, len2)?;
        let result3 = host.delete_many(param0, param1, param2, );
        let (result4_0,result4_1,) = match result3{
          Ok(()) => { (0i32, 0i32)}
          Err(e) => { (1i32, e as i32)}
        };
        let caller_memory = memory.data_mut(&mut caller);
        caller_memory.store(arg6 + 8, wit_bindgen_wasmtime::rt::as_i32(result4_1))?;
        caller_memory.store(arg6 + 0, wit_bindgen_wasmtime::rt::as_i32(result4_0))?;
        Ok(())
      })?;
      linker.func_wrap("wasi-mongo", "drop-database", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32,arg2:i32| {
        let memory = &get_memory(&mut caller, "memory")?;
        let (mem, data) = memory.data_and_store_mut(&mut caller);
        let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
        let host = get(data);
        let ptr0 = arg0;
        let len0 = arg1;
        let param0 = _bc.slice_str(ptr0, len0)?;
        let result1 = host.drop_database(param0, );
        let (result2_0,result2_1,) = match result1{
          Ok(()) => { (0i32, 0i32)}
          Err(e) => { (1i32, e as i32)}
        };
        let caller_memory = memory.data_mut(&mut caller);
        caller_memory.store(arg2 + 8, wit_bindgen_wasmtime::rt::as_i32(result2_1))?;
        caller_memory.store(arg2 + 0, wit_bindgen_wasmtime::rt::as_i32(result2_0))?;
        Ok(())
      })?;
      Ok(())
    }
    use wit_bindgen_wasmtime::rt::RawMem;
  }
  