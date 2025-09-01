use std::{io::Result, process::Stdio, sync::Arc};

use axum::{extract::State, routing::get, Router};
use clap::Parser;
use execute::{shell, Execute};
use tokio::net::TcpListener;

/// Program to monitor daemon process
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Args {
    /// Name of the process to monitor
    #[arg(short, long)]
    process: String,
}

struct AppState {
    process: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let process_name = args.process;
    println!("going to monitor the process: {}", process_name);

    let shared_state = Arc::new(AppState {
        process: process_name,
    });

    let router = Router::new()
        .route("/health", get(health_check))
        .with_state(shared_state);

    let listener = TcpListener::bind("0.0.0.0:4444").await?;
    axum::serve(listener, router).await?;

    Ok(())
}

async fn health_check(State(state): State<Arc<AppState>>) -> String {
    let health_report = String::from("good");
    let process = state.process.as_str();
    println!("going to check the health of process: {}", process);

    let mut command = shell(format!("ps aux | grep {}", process).as_str());
    command.stdout(Stdio::piped());
    let output = command.execute_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    println!("stdout:\n{}", stdout);

    health_report
}
