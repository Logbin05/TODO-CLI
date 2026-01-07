use crate::services::clear_terminal::clear_terminal;

pub fn home_page() {
    clear_terminal();
    print!("--- Welcome to • TODO: CLI • ---");
    let menu = r#"
  ----------------------------
           • Action •

    [1]: Watch list todo;
    [1]: Add todo;
    [3]: Edit name todo;
    [4]: Delete todo;

  ----------------------------
           • Others •

    [c]: clear terminal;
    [q]: Exit/close;
  ----------------------------

  • Made with ❤️ by logbin05👨‍💻 •
"#;

    println!("{menu}");
}
