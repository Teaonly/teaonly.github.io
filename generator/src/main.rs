#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;
mod cli;

use tera::{Context, Result, Tera};

fn main() {
    let command = cli::build_cli().get_matches();
    let root_dir: &str = command.value_of("root").unwrap();

    let all_tempate: String = root_dir.to_string() + "template/**/*.html";

    let mut tera = match Tera::new(&all_tempate) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera.autoescape_on(vec![]);

    let preface = format!("<p>Hello World</p>");
    let title = format!("TODO");

    let mut context = Context::new();
    context.insert("preface", &preface);
    context.insert("title", &title);

    match tera.render("index.html", &context) {
        Ok(result) =>   println!("{}", result),
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
