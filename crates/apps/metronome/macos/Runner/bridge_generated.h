#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct WireSyncReturnStruct {
  uint8_t *ptr;
  int32_t len;
  bool success;
} WireSyncReturnStruct;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

void wire_initialize(int64_t port_);

void wire_deinitialize(int64_t port_);

void wire_set_is_playing(int64_t port_, bool value);

void wire_set_tempo(int64_t port_, float value);

void wire_set_volume(int64_t port_, float value);

void wire_set_beats_per_bar(int64_t port_, int32_t value);

void wire_get_playhead(int64_t port_);

void free_WireSyncReturnStruct(struct WireSyncReturnStruct val);

void store_dart_post_cobject(DartPostCObjectFnType ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_initialize);
    dummy_var ^= ((int64_t) (void*) wire_deinitialize);
    dummy_var ^= ((int64_t) (void*) wire_set_is_playing);
    dummy_var ^= ((int64_t) (void*) wire_set_tempo);
    dummy_var ^= ((int64_t) (void*) wire_set_volume);
    dummy_var ^= ((int64_t) (void*) wire_set_beats_per_bar);
    dummy_var ^= ((int64_t) (void*) wire_get_playhead);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturnStruct);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}