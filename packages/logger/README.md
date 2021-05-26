# @wisual/logger
This is a logging module for the front-end. At the moment it only implements simple creation of contextual and
hierarchical loggers that carry over data through React.js context. It also implements some basic pretty-printing.

Another module should be added here to post logs to a service in production and hide them from users.

## Usage

```typescript jsx
import React from "react";
import { useLogger } from "@wisual/logger";

function MyComponent({ userId, data }: { userId: string; data: any }) {
  // Creatin our logger and properly setting its context
  const logger = useLogger("MyComponent", { userId });
  // Will log with proper context / names
  logger.info("rendering");
  // This will use effect to log
  logger.onInfo("Data has changed", { data });
  // This is wrapping the children with this logger's context provider
  return logger.wrap(<div />);
}
```
