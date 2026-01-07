pub mod run;
pub mod types;
pub mod ui;

#[tokio::main]
async fn main() {
  run::run_cli().await;
}
