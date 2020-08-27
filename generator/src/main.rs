#[macro_use]
extern crate tera;

mod cli;
mod blog;

use std::fs;
use std::path::PathBuf;
use glob::glob;
use tera::{Context, Result, Tera};

fn main() {
    let command = cli::build_cli().get_matches();

    /* ====== step.0 preparing tera engine */
    // build a absolute template path
    let root_value = PathBuf::from(command.value_of("root").unwrap());
    let root_path = fs::canonicalize(&root_value).unwrap();
    let root_dir = root_path.to_str().unwrap();

    // init tera engine, don't apply html string escape
    let template_pattern: String = root_dir.to_string() + "/template/**/*.html";
    let mut tera = match Tera::new(&template_pattern) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera.autoescape_on(vec![]);

    /* ====== step.1 parsing all blog/gist/index file*/
    // scan all the blog files
    let blog_pattern = root_dir.to_string() + "/content/blog/**/*.md";
    let blog_glob = glob(&blog_pattern).expect("Invalid glob");

    let all_blog_path: Vec<_> = blog_glob.filter_map(|e| e.ok()).collect();
    let top_index = root_path.join(PathBuf::from("content/index.md"));

    let mut all_blog: Vec<blog::Blog> = vec![];

    for ref blog_path in all_blog_path {
        if let Ok(blog) = blog::parse(blog_path) {
            all_blog.push(blog);
        } else {
            panic!( format!("Parse blog {} error!", blog_path.to_str().unwrap()));
        }
    }

    /* ====== step.2 rendering file and build target fold */


    /* ====== step.3 copy resource file */
}
