#include <stdlib.h>
#include <wasi-cache.h>

__attribute__((weak, export_name("canonical_abi_realloc")))
void *canonical_abi_realloc(
void *ptr,
size_t orig_size,
size_t org_align,
size_t new_size
) {
  void *ret = realloc(ptr, new_size);
  if (!ret)
  abort();
  return ret;
}

__attribute__((weak, export_name("canonical_abi_free")))
void canonical_abi_free(
void *ptr,
size_t size,
size_t align
) {
  free(ptr);
}
#include <string.h>

void wasi_cache_string_set(wasi_cache_string_t *ret, const char *s) {
  ret->ptr = (char*) s;
  ret->len = strlen(s);
}

void wasi_cache_string_dup(wasi_cache_string_t *ret, const char *s) {
  ret->len = strlen(s);
  ret->ptr = canonical_abi_realloc(NULL, 0, 1, ret->len);
  memcpy(ret->ptr, s, ret->len);
}

void wasi_cache_string_free(wasi_cache_string_t *ret) {
  canonical_abi_free(ret->ptr, ret->len, 1);
  ret->ptr = NULL;
  ret->len = 0;
}
void wasi_cache_payload_free(wasi_cache_payload_t *ptr) {
  canonical_abi_free(ptr->ptr, ptr->len * 1, 1);
}
typedef struct {
  bool is_err;
  union {
    wasi_cache_error_t err;
  } val;
} wasi_cache_expected_unit_error_t;
typedef struct {
  bool is_err;
  union {
    wasi_cache_payload_t ok;
    wasi_cache_error_t err;
  } val;
} wasi_cache_expected_payload_error_t;

__attribute__((aligned(4)))
static uint8_t RET_AREA[12];
__attribute__((import_module("wasi-cache"), import_name("set")))
void __wasm_import_wasi_cache_set(int32_t, int32_t, int32_t, int32_t, int32_t, int32_t, int32_t);
wasi_cache_error_t wasi_cache_set(wasi_cache_string_t *key, wasi_cache_payload_t *value, wasi_cache_option_u32_t *ttl) {
  int32_t option;
  int32_t option1;
  
  if ((*ttl).is_some) {
    const uint32_t *payload0 = &(*ttl).val;
    option = 1;
    option1 = (int32_t) (*payload0);
    
  } else {
    option = 0;
    option1 = 0;
    
  }
  int32_t ptr = (int32_t) &RET_AREA;
  __wasm_import_wasi_cache_set((int32_t) (*key).ptr, (int32_t) (*key).len, (int32_t) (*value).ptr, (int32_t) (*value).len, option, option1, ptr);
  wasi_cache_expected_unit_error_t expected;
  switch ((int32_t) (*((uint8_t*) (ptr + 0)))) {
    case 0: {
      expected.is_err = false;
      
      
      break;
    }
    case 1: {
      expected.is_err = true;
      
      expected.val.err = (int32_t) (*((uint8_t*) (ptr + 1)));
      break;
    }
  }return expected.is_err ? expected.val.err : -1;
}
__attribute__((import_module("wasi-cache"), import_name("get")))
void __wasm_import_wasi_cache_get(int32_t, int32_t, int32_t);
wasi_cache_error_t wasi_cache_get(wasi_cache_string_t *key, wasi_cache_payload_t *ret0) {
  int32_t ptr = (int32_t) &RET_AREA;
  __wasm_import_wasi_cache_get((int32_t) (*key).ptr, (int32_t) (*key).len, ptr);
  wasi_cache_expected_payload_error_t expected;
  switch ((int32_t) (*((uint8_t*) (ptr + 0)))) {
    case 0: {
      expected.is_err = false;
      
      expected.val.ok = (wasi_cache_payload_t) { (uint8_t*)(*((int32_t*) (ptr + 4))), (size_t)(*((int32_t*) (ptr + 8))) };
      break;
    }
    case 1: {
      expected.is_err = true;
      
      expected.val.err = (int32_t) (*((uint8_t*) (ptr + 4)));
      break;
    }
  }*ret0 = expected.val.ok;
  return expected.is_err ? expected.val.err : -1;
}
__attribute__((import_module("wasi-cache"), import_name("delete")))
void __wasm_import_wasi_cache_delete(int32_t, int32_t, int32_t);
wasi_cache_error_t wasi_cache_delete(wasi_cache_string_t *key) {
  int32_t ptr = (int32_t) &RET_AREA;
  __wasm_import_wasi_cache_delete((int32_t) (*key).ptr, (int32_t) (*key).len, ptr);
  wasi_cache_expected_unit_error_t expected;
  switch ((int32_t) (*((uint8_t*) (ptr + 0)))) {
    case 0: {
      expected.is_err = false;
      
      
      break;
    }
    case 1: {
      expected.is_err = true;
      
      expected.val.err = (int32_t) (*((uint8_t*) (ptr + 1)));
      break;
    }
  }return expected.is_err ? expected.val.err : -1;
}
