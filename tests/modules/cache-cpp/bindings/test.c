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
  bool is_err;
  union {
    test_error_t err;
  } val;
} test_expected_unit_error_t;

__attribute__((aligned(1)))
static uint8_t RET_AREA[2];
__attribute__((export_name("test")))
int32_t __wasm_export_test_test(void) {
  test_error_t ret = test_test();
  
  test_expected_unit_error_t ret0;
  if (ret <= 2) {
    ret0.is_err = true;
    ret0.val.err = ret;
  } else {
    ret0.is_err = false;
    
  }
  int32_t ptr = (int32_t) &RET_AREA;
  
  if ((ret0).is_err) {
    const test_error_t *payload1 = &(ret0).val.err;
    *((int8_t*)(ptr + 0)) = 1;
    *((int8_t*)(ptr + 1)) = (int32_t) *payload1;
    
  } else {
    
    *((int8_t*)(ptr + 0)) = 0;
    
  }
  return ptr;
}
