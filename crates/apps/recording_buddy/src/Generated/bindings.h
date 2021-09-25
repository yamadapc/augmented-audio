#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct CompletedCallback {
  void *userdata;
  void (*callback)(void*, bool);
} CompletedCallback;

void async_operation(struct CompletedCallback callback);

void chart_handler_on_chart_view(void *ns_view);
