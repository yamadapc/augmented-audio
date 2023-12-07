#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_InitializeOptions {
  struct wire_uint_8_list *assets_file_path;
} wire_InitializeOptions;

typedef struct DartCObject *WireSyncReturn;

void metronome_will_terminate(void);

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_initialize(int64_t port_, struct wire_InitializeOptions *options);

void wire_deinitialize(int64_t port_);

void wire_set_is_playing(int64_t port_, bool value);

void wire_set_tempo(int64_t port_, float value);

void wire_set_volume(int64_t port_, float value);

void wire_set_beats_per_bar(int64_t port_, int32_t value);

void wire_set_sound(int64_t port_, int32_t value);

void wire_get_playhead(int64_t port_);

void wire_stream_errors(int64_t port_);

struct wire_InitializeOptions *new_box_autoadd_initialize_options_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturn(WireSyncReturn ptr);

jint JNI_OnLoad(JavaVM vm, void *res);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_initialize);
    dummy_var ^= ((int64_t) (void*) wire_deinitialize);
    dummy_var ^= ((int64_t) (void*) wire_set_is_playing);
    dummy_var ^= ((int64_t) (void*) wire_set_tempo);
    dummy_var ^= ((int64_t) (void*) wire_set_volume);
    dummy_var ^= ((int64_t) (void*) wire_set_beats_per_bar);
    dummy_var ^= ((int64_t) (void*) wire_set_sound);
    dummy_var ^= ((int64_t) (void*) wire_get_playhead);
    dummy_var ^= ((int64_t) (void*) wire_stream_errors);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_initialize_options_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
