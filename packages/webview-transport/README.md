# @wisual/webview-transport

This is an abstraction over message-passing _transports_.

Two transports are supported:

* `webkit.messageHandlers` - When running in an embedded webkit webview
* `websockets` - When running anywhere

## Webkit message handlers

### Incoming messages
`WebkitMessageTransport` will receive messages from a global function.

It's expected that the webview host will call this function to pass messages in.
```typescript
window.__onMessage({data:'here'});
```

Data will not be parsed. The host should serialize a JSON string and interpolate
it onto the script.

### Outgoing messages
`WebkitMessageTransport` will post messages on a certain channel.

```typescript
import { WebkitMessageTransport } from "@wisual/webkit-transport";

const transport = new WebkitMessageTransport();
transport.setup();

transport.postMessage('editor', { something: 'here' }, 0);
```

Posting messages will submit to `webkit.messageHandlers.editor`:
```json
{
  "id": 0,
  "message": { "something":  "here" },
  "channel": "editor"
}
```

## WebSocket message handlers
Websockets will connect to `localhost` and forward messages.
The same channel/id/message wrapper is used. The frames should be JSON.