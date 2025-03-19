# LongPort MCP

`LongPort-MCP` is an [MCP](https://modelcontextprotocol.io/introduction) server implementation for LongPort OpenAPI

## Features

- Trading - Create, amend, cancel orders, query today’s/past orders and transaction details, etc.
- Quotes - Real-time quotes, acquisition of historical quotes, etc.
- Portfolio - Real-time query of the account assets, positions, funds

## Installation

### Download the binary

Download the latest binary from the [release page](https://github.com/longportapp/openapi/releases/tag/longport-mcp-0.1.0)

## Running on Cursor

To configure LongPort MCP in Cursor:

- Open Cursor Settings
- Go to Features > MCP Servers
- Click `+ Add New MCP Server`
- Enter the following:
    - Name: `longport-mcp` (or your preferred name)
    - Type: `command`
    - Command: `env LONGPORT_APP_KEY=your-app-key LONGPORT_APP_SECRET=your-app-secret LONGPORT_ACCESS_TOKEN=your-access-token longport-mcp`

If you are using Windows, replace command with `cmd /c "set LONGPORT_APP_KEY=your-app-key && set LONGPORT_APP_SECRET=your-app-secret && set LONGPORT_ACCESS_TOKEN=your-access-token && longport-mcp"`

## Running on Cherry Studio

To configure LongPort MCP in Cherry Studio:

- Go to Settings > MCP Servers
- Click `+ Add Server`
- Enter the following:
    - Name: `longport-mcp` (or your preferred name)
    - Type: `STDIO`
    - Command: `env LONGPORT_APP_KEY=your-app-key LONGPORT_APP_SECRET=your-app-secret LONGPORT_ACCESS_TOKEN=your-access-token longport-mcp`

If you are using Windows, replace command with `cmd /c "set LONGPORT_APP_KEY=your-app-key && set LONGPORT_APP_SECRET=your-app-secret && set LONGPORT_ACCESS_TOKEN=your-access-token && longport-mcp"`

## Running as a SSE server

```bash
env LONGPORT_APP_KEY=your-app-key LONGPORT_APP_SECRET=your-app-secret LONGPORT_ACCESS_TOKEN=your-access-token longport-mcp --sse
```

Default bind address is `127.0.0.1:8000`, you can change it by using the `--bind` flag:

```bash
longport-mcp --sse --bind 127.0.0.1:3000
```