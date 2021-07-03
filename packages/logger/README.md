# @wisual/logger

This is a logging module for the front-end. At the moment it only implements simple creation of contextual and
hierarchical loggers that carry over data through React.js context. It also implements some basic pretty-printing.

## Basic usage
A logger can be created with `LoggerFactory`. It'll be cached by name & thus getting the logger is very cheap.

```typescript jsx
import { LoggerFactory } from "@wisual/logger";

const logger = LoggerFactory.getLogger("MyLogger");

logger.info("Hello world");
logger.debug("segfault");
logger.warn("Coulnd't find X");
logger.error("Oh no");
```

## Structured data
You may pass additional data as a parameter & this will be available for the sinks to output in a structured format:
```typescript jsx
logger.info("Created user", { user, sessionId, tabId });
```

## Hierarchical logger
Loggers are hierarchical. When creating a child, you may set extra context data on to it. Every child will inherits it's
parents contexes.
```typescript jsx
import { LoggerFactory } from "@wisual/logger";

const logger = LoggerFactory.getLogger("root");

const usersServiceLogger = logger.child('UsersService', { dbBackend: 'psql', flushTimeout: 1000 });
const usersCacheLogger = usersServiceLogger.child('UsersCache', { cacheBackend: 'redis' });

usersCacheLogger.info('Cache hits in last 10 seconds', { cacheHits: 10 });
// Data available in the log:
// {
//   logger: "root>UsersService>UsersCache",
//   message: "Cache hits in last 10 seconds",
//   time: "01-03-2021T...",
//   variables: { cacheHits: 10 },
//   context: {
//     dbBackend: 'psql',
//     flushTimeout: 1000,
//     cacheBackend: 'redis',
//   }
// }
```

## React integration
There are hooks provided for react integration.

```typescript jsx
import React from "react";
import { useLogger } from "@wisual/logger";

function MyComponent({ userId, data }: { userId: string; data: any }) {
  // Creating our logger and properly setting its context
  const logger = useLogger("MyComponent", { userId });

  // Will log on render with proper context / names
  logger.info("rendering");

  // This will use effect to log & log everytime the arguments change
  logger.onInfo("Data has changed", { data });

  // This is wrapping the children with this logger's context provider
  return logger.wrap(<div />);
}
```

The hierarchy of loggers will be created based on the React tree. A logger may wrap a sub-tree with **wrapWithLogger**:
```tsx
function Router({ routeName }) {
    const routerLogger = useLogger("Router", { routeName });
    // ...
    return wrapWithLogger(
        routerLogger,
        <div>...</div>
    );
}
```

All calls to `useLogger` within the wrapped sub-tree will inherit the parent context.

## Logger sinks
Sinks are implementors of the `LoggerSink` interface.

Tree sinks are provided:

* `PrettyConsoleSink` - Pretty print messages on Node.js
* `PrettyBrowserSink` - Pretty print messages on browser consoles, which have support for CSS
* `DelegatingSink` - Send messages to multiple sinks

There's no filtering currently implemented.

In addition, you'll find a `tauri` sink which provides rust/js logging functionality in the `crates/plugin-host-gui`
package.

### Configuring the sink
Use `LoggerFactory.setSink`.