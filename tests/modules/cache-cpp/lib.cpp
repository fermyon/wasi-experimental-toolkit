#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <iostream>

#include "bindings/wasi-cache.h"
#include "bindings/test.h"

// wit-bindgen doesn’t create a constant for the 
// “success” case of the `unexpected`, so define one ourselves:
const wasi_cache_error_t SUCCESS_VALUE = -1;

test_error_t test_test(void)
{
    const char* key = "abc";
    const char* value = "def";
    printf("Writing contents `%s` in storage `%s`\n", value, key);

    wasi_cache_string_t skey{
        .ptr = strdup(key),
        .len = strlen(key),
    };

    wasi_cache_payload_t svalue{
        .ptr = (uint8_t*)strdup(value),
        .len = strlen(value),
    };

    auto result = wasi_cache_set(&skey, &svalue, nullptr);
    if (result != SUCCESS_VALUE) {
        fprintf(stderr, "Failed to set value\n");
        return TEST_ERROR_FAILURE;
    }

    wasi_cache_payload_t ret{};
    result = wasi_cache_get(&skey, &ret);
    if (result != SUCCESS_VALUE) {
        fprintf(stderr, "Failed to get value\n");
        return TEST_ERROR_FAILURE;
    }

    if (ret.ptr == nullptr) {
        fprintf(stderr, "No value returned, failing test\n");
        return TEST_ERROR_FAILURE;
    }

    printf("Retrieved from `%s`: `%s`\n", key, (char*)ret.ptr);

    if (svalue.len != ret.len ||
        memcmp(svalue.ptr, ret.ptr, svalue.len) != 0) {
        fprintf(stderr, "Values don't match, failing test\n");
        return TEST_ERROR_FAILURE;
    }

    result = wasi_cache_delete(&skey);
    if (result != SUCCESS_VALUE) {
        fprintf(stderr, "Failed to delete value\n");
        return TEST_ERROR_FAILURE;
    }

    free(svalue.ptr);
    free(skey.ptr);
    free(ret.ptr);

    printf("Test was successful\n");
    return TEST_ERROR_SUCCESS;
}
