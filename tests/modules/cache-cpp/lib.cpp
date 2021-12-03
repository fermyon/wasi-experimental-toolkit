#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <iostream>

#include "bindings/wasi_cache.h"
#include "bindings/test.h"

test_error_t test_test(void)
{
    char *key = "abc";
    char *value = "def";
    printf("Writing contents `%s` in storage `%s`", value, key);

    wasi_cache_string_t *skey;
    skey->len = strlen(key);
    skey->ptr = key;

    wasi_cache_payload_t *svalue;
    svalue->len = strlen(value);
    svalue->ptr = (uint8_t *)value;

    wasi_cache_set(skey, svalue, NULL);

    wasi_cache_payload_t *ret;
    wasi_cache_get(skey, ret);
    printf("Retrieved from `%s`: `%s`", key, (char *)ret->ptr);

    assert(svalue->len == ret->len);

    return 0;
}