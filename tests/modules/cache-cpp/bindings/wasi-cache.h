#ifndef __BINDINGS_WASI_CACHE_H
#define __BINDINGS_WASI_CACHE_H
#ifdef __cplusplus
extern "C"
{
  #endif
  
  #include <stdint.h>
  #include <stdbool.h>
  
  typedef struct {
    char *ptr;
    size_t len;
  } wasi_cache_string_t;
  
  void wasi_cache_string_set(wasi_cache_string_t *ret, const char *s);
  void wasi_cache_string_dup(wasi_cache_string_t *ret, const char *s);
  void wasi_cache_string_free(wasi_cache_string_t *ret);
  typedef uint8_t wasi_cache_error_t;
  #define WASI_CACHE_ERROR_SUCCESS 0
  #define WASI_CACHE_ERROR_ERROR 1
  typedef struct {
    uint8_t *ptr;
    size_t len;
  } wasi_cache_payload_t;
  void wasi_cache_payload_free(wasi_cache_payload_t *ptr);
  typedef struct {
    bool is_some;
    uint32_t val;
  } wasi_cache_option_u32_t;
  wasi_cache_error_t wasi_cache_set(wasi_cache_string_t *key, wasi_cache_payload_t *value, wasi_cache_option_u32_t *ttl);
  wasi_cache_error_t wasi_cache_get(wasi_cache_string_t *key, wasi_cache_payload_t *ret0);
  wasi_cache_error_t wasi_cache_delete(wasi_cache_string_t *key);
  #ifdef __cplusplus
}
#endif
#endif
