#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct CompletedCallback {
  void *userdata;
  void (*callback)(void*, bool);
} CompletedCallback;

void run_loop(void *context);

void run_draw(void);

void async_operation(struct CompletedCallback callback);
