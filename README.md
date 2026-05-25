# mcp-starter

Small Rust starter template for an MCP server running over STDIO.

## What This Template Includes

- JSON-RPC / MCP protocol structs
- A `Server` that handles MCP requests and notifications
- A `Runtime` facade with a working STDIO runtime
- A tool system with:
  - `Tool` trait
  - `ToolRegistry`
  - `ToolDescriptor`
  - `ToolResult`
- Two demo tools:
  - `echo`
  - `time`

## Project Layout

- `src/protocol/`
  MCP message structs like request, success, error, and notification.

- `src/server/`
  MCP server behavior and internal transport handling.

- `src/runtime/`
  Runtime hosting. Right now this includes STDIO only.

- `src/tool/`
  Shared internal tool abstractions and types.

- `src/tools/`
  Concrete tool implementations used by the server.

## Run

```powershell
cargo run
```

The default entrypoint uses:

```rust
Runtime::new().run(&Server::new(), RuntimeMode::Stdio).await
```

The runtime API is already structured for multiple modes, even though only STDIO is implemented right now.

## Test Manually

Send one JSON line at a time after `cargo run`.

Initialize:

```json
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"test-client","version":"0.1.0"}}}
```

Initialized notification:

```json
{"jsonrpc":"2.0","method":"notifications/initialized"}
```

List tools:

```json
{"jsonrpc":"2.0","id":2,"method":"tools/list"}
```

Call `echo`:

```json
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"echo","arguments":{"text":"hello"}}}
```

Call `time`:

```json
{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"time","arguments":{}}}
```

## Add A New Tool

1. Add a new file in `src/tools/`
2. Implement the `Tool` trait
3. Export it from `src/tools/mod.rs`
4. Register it in `src/server/transport.rs` inside `register_default_tools()`

The demo tools are good references:

- `src/tools/echo.rs`
- `src/tools/time.rs`

## Notes

- Notifications do not return responses.
- The current runtime is newline-delimited JSON for local testing.
- The runtime layer is separated so HTTP can be added later without changing server logic.
