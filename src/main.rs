pub mod types;

fn main() {
  let menu = r#"
    --- TODO: CLI ---

    Action:
    [1] Add todo;
    [2] Edit name todo;
    [3] Delete todo;
    [q] Exit/close;
  "#;
  
    println!("{}", format!("{}", menu));
}
