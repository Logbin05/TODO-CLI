pub mod run;
pub mod types;
pub mod ui;
pub mod services;

#[tokio::main]
async fn main() {
  run::run_cli().await;
}
