#include <stdlib.h>
#include <wasi_cache.h>

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
  // 0 if `val` is `ok`, 1 otherwise
  uint8_t tag;
  union {
    wasi_cache_error_t err;
  } val;
} wasi_cache_expected_void_error_t;
typedef struct {
  // 0 if `val` is `ok`, 1 otherwise
  uint8_t tag;
  union {
    wasi_cache_payload_t ok;
    wasi_cache_error_t err;
  } val;
} wasi_cache_expected_payload_error_t;
static int64_t RET_AREA[3];
__attribute__((import_module("wasi_cache"), import_name("set")))
void __wasm_import_wasi_cache_set(int32_t, int32_t, int32_t, int32_t, int32_t, int32_t, int32_t);
wasi_cache_error_t wasi_cache_set(wasi_cache_string_t *key, wasi_cache_payload_t *value, wasi_cache_option_u32_t *ttl) {
  int32_t variant;
  int32_t variant1;
  switch ((int32_t) (*ttl).tag) {
    case 0: {
      variant = 0;
      variant1 = 0;
      break;
    }
    case 1: {
      const uint32_t *payload0 = &(*ttl).val;
      variant = 1;
      variant1 = (int32_t) (*payload0);
      break;
    }
  }
  int32_t ptr = (int32_t) &RET_AREA;
  __wasm_import_wasi_cache_set((int32_t) (*key).ptr, (int32_t) (*key).len, (int32_t) (*value).ptr, (int32_t) (*value).len, variant, variant1, ptr);
  wasi_cache_expected_void_error_t variant2;
  variant2.tag = *((int32_t*) (ptr + 0));
  switch ((int32_t) variant2.tag) {
    case 0: {
      break;
    }
    case 1: {
      variant2.val.err = *((int32_t*) (ptr + 8));
      break;
    }
  }
  return variant2.tag ? variant2.val.err : -1;
}
__attribute__((import_module("wasi_cache"), import_name("get")))
void __wasm_import_wasi_cache_get(int32_t, int32_t, int32_t);
wasi_cache_error_t wasi_cache_get(wasi_cache_string_t *key, wasi_cache_payload_t *ret0) {
  int32_t ptr = (int32_t) &RET_AREA;
  __wasm_import_wasi_cache_get((int32_t) (*key).ptr, (int32_t) (*key).len, ptr);
  wasi_cache_expected_payload_error_t variant;
  variant.tag = *((int32_t*) (ptr + 0));
  switch ((int32_t) variant.tag) {
    case 0: {
      variant.val.ok = (wasi_cache_payload_t) { (uint8_t*)(*((int32_t*) (ptr + 8))), (size_t)(*((int32_t*) (ptr + 16))) };
      break;
    }
    case 1: {
      variant.val.err = *((int32_t*) (ptr + 8));
      break;
    }
  }
  *ret0 = variant.val.ok;
  return variant.tag ? variant.val.err : -1;
}
__attribute__((import_module("wasi_cache"), import_name("delete")))
void __wasm_import_wasi_cache_delete(int32_t, int32_t, int32_t);
wasi_cache_error_t wasi_cache_delete(wasi_cache_string_t *key) {
  int32_t ptr = (int32_t) &RET_AREA;
  __wasm_import_wasi_cache_delete((int32_t) (*key).ptr, (int32_t) (*key).len, ptr);
  wasi_cache_expected_void_error_t variant;
  variant.tag = *((int32_t*) (ptr + 0));
  switch ((int32_t) variant.tag) {
    case 0: {
      break;
    }
    case 1: {
      variant.val.err = *((int32_t*) (ptr + 8));
      break;
    }
  }
  return variant.tag ? variant.val.err : -1;
}
