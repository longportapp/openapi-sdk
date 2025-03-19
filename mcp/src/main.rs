mod server;

use std::sync::Arc;

use clap::Parser;
use longport::{Config, QuoteContext, TradeContext};
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route, Server};
use poem_mcpserver::{sse::sse_endpoint, stdio::stdio, McpServer};
use server::Longport;

#[derive(Parser)]
struct Cli {
    /// Use SSE transport
    #[clap(long)]
    sse: bool,
    /// Bind address for the SSE server.
    #[clap(long, default_value = "127.0.0.1:8000")]
    bind: String,
    /// Use verbose output
    #[clap(short, long, default_value = "false")]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    if cli.verbose {
        tracing_subscriber::fmt()
            .with_writer(std::io::stderr)
            .init();
    }

    let config = Arc::new(Config::from_env()?.dont_print_quote_packages());
    let (quote_context, _) = QuoteContext::try_new(config.clone()).await?;
    let (trade_context, _) = TradeContext::try_new(config.clone()).await?;

    if !cli.sse {
        let server = McpServer::new().tools(Longport::new(quote_context, trade_context));
        stdio(server).await?;
    } else {
        let listener = TcpListener::bind(&cli.bind);
        let app = Route::new()
            .at(
                "/sse",
                sse_endpoint(move || {
                    let tools = Longport::new(quote_context.clone(), trade_context.clone());
                    McpServer::new().tools(tools)
                }),
            )
            .with(Cors::new());
        Server::new(listener).run(app).await?;
    }

    Ok(())
}
