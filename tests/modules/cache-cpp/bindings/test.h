#ifndef __BINDINGS_TEST_H
#define __BINDINGS_TEST_H
#ifdef __cplusplus
extern "C"
{
  #endif
  
  #include <stdint.h>
  #include <stdbool.h>
  typedef uint8_t test_error_t;
  #define TEST_ERROR_SUCCESS 0
  #define TEST_ERROR_FAILURE 1
  test_error_t test_test(void);
  #ifdef __cplusplus
}
#endif
#endif
