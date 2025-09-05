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

    //todo add a scheduler to regularly call health_check
    health_check(State(shared_state.clone())).await;

    let router = Router::new()
        .route("/health", get(health_check))
        .with_state(shared_state);

    let listener = TcpListener::bind("0.0.0.0:4444").await?;
    axum::serve(listener, router).await?;

    Ok(())
}

async fn health_check(State(state): State<Arc<AppState>>) -> String {
    let mut health_report = String::from("healt report: ");
    let process = state.process.as_str();
    println!("going to check the health of process: {}", process);

    let mut command = shell(
        format!(
            "ps aux | grep {} | grep -v grep | grep -v process-monitor",
            process
        )
        .as_str(),
    );
    command.stdout(Stdio::piped());
    let output = command.execute_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stdout = stdout.trim();
    println!("stdout:\n{}", stdout);

    if !stdout.is_empty() {
        health_report.push_str("good");
    } else {
        health_report.push_str(format!("bad (process {} not found)", process).as_str());
    }

    health_report
}
