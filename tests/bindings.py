from abc import abstractmethod
import ctypes
from dataclasses import dataclass
from enum import Enum
from typing import Any, Generic, List, Optional, Tuple, TypeVar, Union, cast
import wasmtime

try:
    from typing import Protocol
except ImportError:
    class Protocol: # type: ignore
        pass

T = TypeVar('T')

def _load(ty: Any, mem: wasmtime.Memory, store: wasmtime.Storelike, base: int, offset: int) -> Any:
    ptr = (base & 0xffffffff) + offset
    if ptr + ctypes.sizeof(ty) > mem.data_len(store):
        raise IndexError('out-of-bounds store')
    raw_base = mem.data_ptr(store)
    c_ptr = ctypes.POINTER(ty)(
        ty.from_address(ctypes.addressof(raw_base.contents) + ptr)
    )
    return c_ptr[0]

@dataclass
class Ok(Generic[T]):
    value: T
E = TypeVar('E')
@dataclass
class Err(Generic[E]):
    value: E

Expected = Union[Ok[T], Err[E]]

def _decode_utf8(mem: wasmtime.Memory, store: wasmtime.Storelike, ptr: int, len: int) -> str:
    ptr = ptr & 0xffffffff
    len = len & 0xffffffff
    if ptr + len > mem.data_len(store):
        raise IndexError('string out of bounds')
    base = mem.data_ptr(store)
    base = ctypes.POINTER(ctypes.c_ubyte)(
        ctypes.c_ubyte.from_address(ctypes.addressof(base.contents) + ptr)
    )
    return ctypes.string_at(base, len).decode('utf-8')

def _encode_utf8(val: str, realloc: wasmtime.Func, mem: wasmtime.Memory, store: wasmtime.Storelike) -> Tuple[int, int]:
    bytes = val.encode('utf8')
    ptr = realloc(store, 0, 0, 1, len(bytes))
    assert(isinstance(ptr, int))
    ptr = ptr & 0xffffffff
    if ptr + len(bytes) > mem.data_len(store):
        raise IndexError('string out of bounds')
    base = mem.data_ptr(store)
    base = ctypes.POINTER(ctypes.c_ubyte)(
        ctypes.c_ubyte.from_address(ctypes.addressof(base.contents) + ptr)
    )
    ctypes.memmove(base, bytes, len(bytes))
    return (ptr, len(bytes))

def _list_canon_lift(ptr: int, len: int, size: int, ty: Any, mem: wasmtime.Memory ,store: wasmtime.Storelike) -> Any:
    ptr = ptr & 0xffffffff
    len = len & 0xffffffff
    if ptr + len * size > mem.data_len(store):
        raise IndexError('list out of bounds')
    raw_base = mem.data_ptr(store)
    base = ctypes.POINTER(ty)(
        ty.from_address(ctypes.addressof(raw_base.contents) + ptr)
    )
    if ty == ctypes.c_uint8:
        return ctypes.string_at(base, len)
    return base[:len]

def _list_canon_lower(list: Any, ty: Any, size: int, align: int, realloc: wasmtime.Func, mem: wasmtime.Memory, store: wasmtime.Storelike) -> Tuple[int, int]:
    total_size = size * len(list)
    ptr = realloc(store, 0, 0, align, total_size)
    assert(isinstance(ptr, int))
    ptr = ptr & 0xffffffff
    if ptr + total_size > mem.data_len(store):
        raise IndexError('list realloc return of bounds')
    raw_base = mem.data_ptr(store)
    base = ctypes.POINTER(ty)(
        ty.from_address(ctypes.addressof(raw_base.contents) + ptr)
    )
    for i, val in enumerate(list):
        base[i] = val
    return (ptr, len(list))

@dataclass
class SlabEntry(Generic[T]):
    next: int
    val: Optional[T]

class Slab(Generic[T]):
    head: int
    list: List[SlabEntry[T]]

    def __init__(self) -> None:
        self.list = []
        self.head = 0

    def insert(self, val: T) -> int:
        if self.head >= len(self.list):
            self.list.append(SlabEntry(next = len(self.list) + 1, val = None))
        ret = self.head
        slot = self.list[ret]
        self.head = slot.next
        slot.next = -1
        slot.val = val
        return ret

    def get(self, idx: int) -> T:
        if idx >= len(self.list):
            raise IndexError('handle index not valid')
        slot = self.list[idx]
        if slot.next == -1:
            assert(slot.val is not None)
            return slot.val
        raise IndexError('handle index not valid')

    def remove(self, idx: int) -> T:
        ret = self.get(idx)
        slot = self.list[idx]
        slot.val = None
        slot.next = self.head
        self.head = idx
        return ret

Data = bytes

class Cloudevent:
    
    _wasm_val: int
    _refcnt: int
    _obj: 'WasiCe'
    _destroyed: bool
    
    def __init__(self, val: int, obj: 'WasiCe') -> None:
        self._wasm_val = val
        self._refcnt = 1
        self._obj = obj
        self._destroyed = False
    
    def clone(self) -> 'Cloudevent':
        self._refcnt += 1
        return self
    
    def drop(self, store: wasmtime.Storelike) -> None:
        self._refcnt -= 1;
        if self._refcnt != 0:
            return
        assert(not self._destroyed)
        self._destroyed = True
        self._obj._canonical_abi_drop_cloudevent(store, self._wasm_val)
    
    def __del__(self) -> None:
        if not self._destroyed:
            raise RuntimeError('wasm object not dropped')
    @classmethod
    def create(cls, caller: wasmtime.Store, obj: 'WasiCe') -> 'Cloudevent':
        ret = obj._cloudevent_create(caller)
        assert(isinstance(ret, int))
        return obj._resource0_slab.remove(ret)
    def set_id(self, caller: wasmtime.Store, id: str) -> None:
        memory = self._obj._memory;
        realloc = self._obj._canonical_abi_realloc
        obj = self
        ptr, len0 = _encode_utf8(id, realloc, memory, caller)
        self._obj._cloudevent_set_id(caller, self._obj._resource0_slab.insert(obj.clone()), ptr, len0)
    def get_id(self, caller: wasmtime.Store) -> str:
        memory = self._obj._memory;
        free = self._obj._canonical_abi_free
        obj = self
        ret = self._obj._cloudevent_get_id(caller, self._obj._resource0_slab.insert(obj.clone()))
        assert(isinstance(ret, int))
        load = _load(ctypes.c_int32, memory, caller, ret, 0)
        load0 = _load(ctypes.c_int32, memory, caller, ret, 8)
        ptr = load
        len1 = load0
        list = _decode_utf8(memory, caller, ptr, len1)
        free(caller, ptr, len1, 1)
        return list
    def set_source(self, caller: wasmtime.Store, source: str) -> None:
        memory = self._obj._memory;
        realloc = self._obj._canonical_abi_realloc
        obj = self
        ptr, len0 = _encode_utf8(source, realloc, memory, caller)
        self._obj._cloudevent_set_source(caller, self._obj._resource0_slab.insert(obj.clone()), ptr, len0)
    def get_source(self, caller: wasmtime.Store) -> str:
        memory = self._obj._memory;
        free = self._obj._canonical_abi_free
        obj = self
        ret = self._obj._cloudevent_get_source(caller, self._obj._resource0_slab.insert(obj.clone()))
        assert(isinstance(ret, int))
        load = _load(ctypes.c_int32, memory, caller, ret, 0)
        load0 = _load(ctypes.c_int32, memory, caller, ret, 8)
        ptr = load
        len1 = load0
        list = _decode_utf8(memory, caller, ptr, len1)
        free(caller, ptr, len1, 1)
        return list
    def set_specversion(self, caller: wasmtime.Store, specversion: str) -> None:
        memory = self._obj._memory;
        realloc = self._obj._canonical_abi_realloc
        obj = self
        ptr, len0 = _encode_utf8(specversion, realloc, memory, caller)
        self._obj._cloudevent_set_specversion(caller, self._obj._resource0_slab.insert(obj.clone()), ptr, len0)
    def get_specversion(self, caller: wasmtime.Store) -> str:
        memory = self._obj._memory;
        free = self._obj._canonical_abi_free
        obj = self
        ret = self._obj._cloudevent_get_specversion(caller, self._obj._resource0_slab.insert(obj.clone()))
        assert(isinstance(ret, int))
        load = _load(ctypes.c_int32, memory, caller, ret, 0)
        load0 = _load(ctypes.c_int32, memory, caller, ret, 8)
        ptr = load
        len1 = load0
        list = _decode_utf8(memory, caller, ptr, len1)
        free(caller, ptr, len1, 1)
        return list
    def set_type(self, caller: wasmtime.Store, type: str) -> None:
        memory = self._obj._memory;
        realloc = self._obj._canonical_abi_realloc
        obj = self
        ptr, len0 = _encode_utf8(type, realloc, memory, caller)
        self._obj._cloudevent_set_type(caller, self._obj._resource0_slab.insert(obj.clone()), ptr, len0)
    def get_type(self, caller: wasmtime.Store) -> str:
        memory = self._obj._memory;
        free = self._obj._canonical_abi_free
        obj = self
        ret = self._obj._cloudevent_get_type(caller, self._obj._resource0_slab.insert(obj.clone()))
        assert(isinstance(ret, int))
        load = _load(ctypes.c_int32, memory, caller, ret, 0)
        load0 = _load(ctypes.c_int32, memory, caller, ret, 8)
        ptr = load
        len1 = load0
        list = _decode_utf8(memory, caller, ptr, len1)
        free(caller, ptr, len1, 1)
        return list
    def set_data(self, caller: wasmtime.Store, data: Data) -> None:
        memory = self._obj._memory;
        realloc = self._obj._canonical_abi_realloc
        obj = self
        ptr, len0 = _list_canon_lower(data, ctypes.c_uint8, 1, 1, realloc, memory, caller)
        self._obj._cloudevent_set_data(caller, self._obj._resource0_slab.insert(obj.clone()), ptr, len0)
    def get_data(self, caller: wasmtime.Store) -> Data:
        memory = self._obj._memory;
        free = self._obj._canonical_abi_free
        obj = self
        ret = self._obj._cloudevent_get_data(caller, self._obj._resource0_slab.insert(obj.clone()))
        assert(isinstance(ret, int))
        load = _load(ctypes.c_int32, memory, caller, ret, 0)
        load0 = _load(ctypes.c_int32, memory, caller, ret, 8)
        ptr = load
        len1 = load0
        list = cast(bytes, _list_canon_lift(ptr, len1, 1, ctypes.c_uint8, memory, caller))
        free(caller, ptr, len1, 1)
        return list
    def set_datacontenttype(self, caller: wasmtime.Store, datacontenttype: str) -> None:
        memory = self._obj._memory;
        realloc = self._obj._canonical_abi_realloc
        obj = self
        ptr, len0 = _encode_utf8(datacontenttype, realloc, memory, caller)
        self._obj._cloudevent_set_datacontenttype(caller, self._obj._resource0_slab.insert(obj.clone()), ptr, len0)
    def get_datacontenttype(self, caller: wasmtime.Store) -> str:
        memory = self._obj._memory;
        free = self._obj._canonical_abi_free
        obj = self
        ret = self._obj._cloudevent_get_datacontenttype(caller, self._obj._resource0_slab.insert(obj.clone()))
        assert(isinstance(ret, int))
        load = _load(ctypes.c_int32, memory, caller, ret, 0)
        load0 = _load(ctypes.c_int32, memory, caller, ret, 8)
        ptr = load
        len1 = load0
        list = _decode_utf8(memory, caller, ptr, len1)
        free(caller, ptr, len1, 1)
        return list
    def set_dataschema(self, caller: wasmtime.Store, dataschema: str) -> None:
        memory = self._obj._memory;
        realloc = self._obj._canonical_abi_realloc
        obj = self
        ptr, len0 = _encode_utf8(dataschema, realloc, memory, caller)
        self._obj._cloudevent_set_dataschema(caller, self._obj._resource0_slab.insert(obj.clone()), ptr, len0)
    def get_dataschema(self, caller: wasmtime.Store) -> str:
        memory = self._obj._memory;
        free = self._obj._canonical_abi_free
        obj = self
        ret = self._obj._cloudevent_get_dataschema(caller, self._obj._resource0_slab.insert(obj.clone()))
        assert(isinstance(ret, int))
        load = _load(ctypes.c_int32, memory, caller, ret, 0)
        load0 = _load(ctypes.c_int32, memory, caller, ret, 8)
        ptr = load
        len1 = load0
        list = _decode_utf8(memory, caller, ptr, len1)
        free(caller, ptr, len1, 1)
        return list
    def set_subject(self, caller: wasmtime.Store, subject: str) -> None:
        memory = self._obj._memory;
        realloc = self._obj._canonical_abi_realloc
        obj = self
        ptr, len0 = _encode_utf8(subject, realloc, memory, caller)
        self._obj._cloudevent_set_subject(caller, self._obj._resource0_slab.insert(obj.clone()), ptr, len0)
    def get_subject(self, caller: wasmtime.Store) -> str:
        memory = self._obj._memory;
        free = self._obj._canonical_abi_free
        obj = self
        ret = self._obj._cloudevent_get_subject(caller, self._obj._resource0_slab.insert(obj.clone()))
        assert(isinstance(ret, int))
        load = _load(ctypes.c_int32, memory, caller, ret, 0)
        load0 = _load(ctypes.c_int32, memory, caller, ret, 8)
        ptr = load
        len1 = load0
        list = _decode_utf8(memory, caller, ptr, len1)
        free(caller, ptr, len1, 1)
        return list
    def set_time(self, caller: wasmtime.Store, time: str) -> None:
        memory = self._obj._memory;
        realloc = self._obj._canonical_abi_realloc
        obj = self
        ptr, len0 = _encode_utf8(time, realloc, memory, caller)
        self._obj._cloudevent_set_time(caller, self._obj._resource0_slab.insert(obj.clone()), ptr, len0)
    def get_time(self, caller: wasmtime.Store) -> str:
        memory = self._obj._memory;
        free = self._obj._canonical_abi_free
        obj = self
        ret = self._obj._cloudevent_get_time(caller, self._obj._resource0_slab.insert(obj.clone()))
        assert(isinstance(ret, int))
        load = _load(ctypes.c_int32, memory, caller, ret, 0)
        load0 = _load(ctypes.c_int32, memory, caller, ret, 8)
        ptr = load
        len1 = load0
        list = _decode_utf8(memory, caller, ptr, len1)
        free(caller, ptr, len1, 1)
        return list
# General purpose error.
class Error(Enum):
    SUCCESS = 0
    ERROR = 1

Payload = bytes
# CloudEvent
# record cloudevent {
# "id": string,
# "source": string,
# "specversion": string,
# "type": string,
# }
# record cloudevent-options {
# "datacontenttype": option<string>,
# "dataschema": option<string>,
# "subject": option<string>,
# "time": option<string>,
# "data": option<data>,
# }
class WasiCe:
    instance: wasmtime.Instance
    _canonical_abi_free: wasmtime.Func
    _canonical_abi_realloc: wasmtime.Func
    _ce_handler: wasmtime.Func
    _cloudevent_create: wasmtime.Func
    _cloudevent_get_data: wasmtime.Func
    _cloudevent_get_datacontenttype: wasmtime.Func
    _cloudevent_get_dataschema: wasmtime.Func
    _cloudevent_get_id: wasmtime.Func
    _cloudevent_get_source: wasmtime.Func
    _cloudevent_get_specversion: wasmtime.Func
    _cloudevent_get_subject: wasmtime.Func
    _cloudevent_get_time: wasmtime.Func
    _cloudevent_get_type: wasmtime.Func
    _cloudevent_set_data: wasmtime.Func
    _cloudevent_set_datacontenttype: wasmtime.Func
    _cloudevent_set_dataschema: wasmtime.Func
    _cloudevent_set_id: wasmtime.Func
    _cloudevent_set_source: wasmtime.Func
    _cloudevent_set_specversion: wasmtime.Func
    _cloudevent_set_subject: wasmtime.Func
    _cloudevent_set_time: wasmtime.Func
    _cloudevent_set_type: wasmtime.Func
    _memory: wasmtime.Memory
    _resource0_slab: Slab[Cloudevent]
    _canonical_abi_drop_cloudevent: wasmtime.Func
    def __init__(self, store: wasmtime.Store, linker: wasmtime.Linker, module: wasmtime.Module):
        
        ty1 = wasmtime.FuncType([wasmtime.ValType.i32()], [])
        ty2 = wasmtime.FuncType([wasmtime.ValType.i32()], [wasmtime.ValType.i32()])
        def drop_cloudevent(caller: wasmtime.Caller, idx: int) -> None:
             self._resource0_slab.remove(idx).drop(caller);
        linker.define('canonical_abi', 'resource_drop_cloudevent', wasmtime.Func(store, ty1, drop_cloudevent, access_caller = True))
        
        def clone_cloudevent(idx: int) -> int:
             obj = self._resource0_slab.get(idx)
             return self._resource0_slab.insert(obj.clone())
        linker.define('canonical_abi', 'resource_clone_cloudevent', wasmtime.Func(store, ty2, clone_cloudevent))
        
        def get_cloudevent(idx: int) -> int:
             obj = self._resource0_slab.get(idx)
             return obj._wasm_val
        linker.define('canonical_abi', 'resource_get_cloudevent', wasmtime.Func(store, ty2, get_cloudevent))
        
        def new_cloudevent(val: int) -> int:
             return self._resource0_slab.insert(Cloudevent(val, self))
        linker.define('canonical_abi', 'resource_new_cloudevent', wasmtime.Func(store, ty2, new_cloudevent))
        self.instance = linker.instantiate(store, module)
        exports = self.instance.exports(store)
        
        canonical_abi_free = exports['canonical_abi_free']
        assert(isinstance(canonical_abi_free, wasmtime.Func))
        self._canonical_abi_free = canonical_abi_free
        
        canonical_abi_realloc = exports['canonical_abi_realloc']
        assert(isinstance(canonical_abi_realloc, wasmtime.Func))
        self._canonical_abi_realloc = canonical_abi_realloc
        
        ce_handler = exports['ce-handler']
        assert(isinstance(ce_handler, wasmtime.Func))
        self._ce_handler = ce_handler
        
        cloudevent_create = exports['cloudevent::create']
        assert(isinstance(cloudevent_create, wasmtime.Func))
        self._cloudevent_create = cloudevent_create
        
        cloudevent_get_data = exports['cloudevent::get-data']
        assert(isinstance(cloudevent_get_data, wasmtime.Func))
        self._cloudevent_get_data = cloudevent_get_data
        
        cloudevent_get_datacontenttype = exports['cloudevent::get-datacontenttype']
        assert(isinstance(cloudevent_get_datacontenttype, wasmtime.Func))
        self._cloudevent_get_datacontenttype = cloudevent_get_datacontenttype
        
        cloudevent_get_dataschema = exports['cloudevent::get-dataschema']
        assert(isinstance(cloudevent_get_dataschema, wasmtime.Func))
        self._cloudevent_get_dataschema = cloudevent_get_dataschema
        
        cloudevent_get_id = exports['cloudevent::get-id']
        assert(isinstance(cloudevent_get_id, wasmtime.Func))
        self._cloudevent_get_id = cloudevent_get_id
        
        cloudevent_get_source = exports['cloudevent::get-source']
        assert(isinstance(cloudevent_get_source, wasmtime.Func))
        self._cloudevent_get_source = cloudevent_get_source
        
        cloudevent_get_specversion = exports['cloudevent::get-specversion']
        assert(isinstance(cloudevent_get_specversion, wasmtime.Func))
        self._cloudevent_get_specversion = cloudevent_get_specversion
        
        cloudevent_get_subject = exports['cloudevent::get-subject']
        assert(isinstance(cloudevent_get_subject, wasmtime.Func))
        self._cloudevent_get_subject = cloudevent_get_subject
        
        cloudevent_get_time = exports['cloudevent::get-time']
        assert(isinstance(cloudevent_get_time, wasmtime.Func))
        self._cloudevent_get_time = cloudevent_get_time
        
        cloudevent_get_type = exports['cloudevent::get-type']
        assert(isinstance(cloudevent_get_type, wasmtime.Func))
        self._cloudevent_get_type = cloudevent_get_type
        
        cloudevent_set_data = exports['cloudevent::set-data']
        assert(isinstance(cloudevent_set_data, wasmtime.Func))
        self._cloudevent_set_data = cloudevent_set_data
        
        cloudevent_set_datacontenttype = exports['cloudevent::set-datacontenttype']
        assert(isinstance(cloudevent_set_datacontenttype, wasmtime.Func))
        self._cloudevent_set_datacontenttype = cloudevent_set_datacontenttype
        
        cloudevent_set_dataschema = exports['cloudevent::set-dataschema']
        assert(isinstance(cloudevent_set_dataschema, wasmtime.Func))
        self._cloudevent_set_dataschema = cloudevent_set_dataschema
        
        cloudevent_set_id = exports['cloudevent::set-id']
        assert(isinstance(cloudevent_set_id, wasmtime.Func))
        self._cloudevent_set_id = cloudevent_set_id
        
        cloudevent_set_source = exports['cloudevent::set-source']
        assert(isinstance(cloudevent_set_source, wasmtime.Func))
        self._cloudevent_set_source = cloudevent_set_source
        
        cloudevent_set_specversion = exports['cloudevent::set-specversion']
        assert(isinstance(cloudevent_set_specversion, wasmtime.Func))
        self._cloudevent_set_specversion = cloudevent_set_specversion
        
        cloudevent_set_subject = exports['cloudevent::set-subject']
        assert(isinstance(cloudevent_set_subject, wasmtime.Func))
        self._cloudevent_set_subject = cloudevent_set_subject
        
        cloudevent_set_time = exports['cloudevent::set-time']
        assert(isinstance(cloudevent_set_time, wasmtime.Func))
        self._cloudevent_set_time = cloudevent_set_time
        
        cloudevent_set_type = exports['cloudevent::set-type']
        assert(isinstance(cloudevent_set_type, wasmtime.Func))
        self._cloudevent_set_type = cloudevent_set_type
        
        memory = exports['memory']
        assert(isinstance(memory, wasmtime.Memory))
        self._memory = memory
        
        self._resource0_slab = Slab()
        canon_drop_cloudevent = exports['canonical_abi_drop_cloudevent']
        assert(isinstance(canon_drop_cloudevent, wasmtime.Func))
        self._canonical_abi_drop_cloudevent = canon_drop_cloudevent
    def ce_handler(self, caller: wasmtime.Store, event: 'Cloudevent') -> Expected['Cloudevent', Error]:
        memory = self._memory;
        obj = event
        ret = self._ce_handler(caller, self._resource0_slab.insert(obj.clone()))
        assert(isinstance(ret, int))
        load = _load(ctypes.c_int32, memory, caller, ret, 0)
        load0 = _load(ctypes.c_int32, memory, caller, ret, 8)
        variant: Expected['Cloudevent', Error]
        if load == 0:
            variant = Ok(self._resource0_slab.remove(load0))
        elif load == 1:
            variant = Err(Error(load0))
        else:
            raise TypeError("invalid variant discriminant for expected")
        return variant
