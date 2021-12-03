#include <stdlib.h>
#include <test.h>

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
typedef struct {
  // 0 if `val` is `ok`, 1 otherwise
  uint8_t tag;
  union {
    test_error_t err;
  } val;
} test_expected_void_error_t;
static int64_t RET_AREA[2];
__attribute__((export_name("test")))
int32_t __wasm_export_test_test(void) {
  test_error_t ret = test_test();
  
  test_expected_void_error_t ret0;
  if (ret <= 2) {
    ret0.tag = 1;
    ret0.val.err = ret;
  } else {
    ret0.tag = 0;
    
  }
  int32_t variant4;
  int32_t variant5;
  switch ((int32_t) (ret0).tag) {
    case 0: {
      variant4 = 0;
      variant5 = 0;
      break;
    }
    case 1: {
      const test_error_t *payload1 = &(ret0).val.err;
      int32_t variant;
      switch ((int32_t) *payload1) {
        case 0: {
          variant = 0;
          break;
        }
        case 1: {
          variant = 1;
          break;
        }
      }
      variant4 = 1;
      variant5 = variant;
      break;
    }
  }
  int32_t ptr = (int32_t) &RET_AREA;
  *((int32_t*)(ptr + 8)) = variant5;
  *((int32_t*)(ptr + 0)) = variant4;
  return ptr;
}
