#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct LooperEngine LooperEngine;

typedef struct LooperVoice LooperVoice;

typedef struct SharedPtr_LooperProcessorHandle SharedPtr_LooperProcessorHandle;

struct LooperEngine *looper_engine__new(void);

uintptr_t looper_engine__num_loopers(struct LooperEngine *engine);

void looper_engine__record(struct LooperEngine *engine, uintptr_t looper_id);

void looper_engine__play(struct LooperEngine *engine, uintptr_t looper_id);

void looper_engine__clear(struct LooperEngine *engine, uintptr_t looper_id);

struct LooperVoice *looper_engine__get_voice(struct LooperEngine *engine, uintptr_t looper_id);

struct SharedPtr_LooperProcessorHandle looper_voice__get_looper_handle(struct LooperVoice *voice);

void looper_handle__free(struct SharedPtr_LooperProcessorHandle handle);

bool looper_handle__is_recording(struct SharedPtr_LooperProcessorHandle handle);

bool looper_handle__is_playing_back(struct SharedPtr_LooperProcessorHandle handle);
