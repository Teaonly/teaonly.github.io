extern crate tera;
extern crate yaml_rust;
extern crate chrono;
extern crate pulldown_cmark;

mod help;
mod cli;
mod blog;

use std::fs;
use std::collections::HashSet;
use std::path::PathBuf;
use chrono::{NaiveDate};
use glob::glob;
use tera::Tera;

fn create_blog_index(tera: Tera, target_dir: &PathBuf,  blogs: &mut Vec<blog::Blog>) -> Result<(), String> {
    let mut strbuf = String::new();

    blogs.sort_by(|a,b|b.date.cmp(&a.date));
    for ref blog in blogs {
        if blog.todo {
            continue;
        }
        let href = format!("/blog/{}", blog.code);
        let date = NaiveDate::parse_from_str( &blog.date, "%Y-%m-%d").unwrap().format("%-d %B, %C%y").to_string();
        strbuf.push_str(&format!("<li><a href={}> {} </a>\n", href, blog.title));
        strbuf.push_str(&format!("<figcaption><b>{}</b></figcaption>\n", date));
        strbuf.push_str(&format!("<figcaption>{}</figcaption>\n", blog.desc));
        strbuf.push_str("</li>\n");
    }

    let mut context = tera::Context::new();
    context.insert("articles_list", &strbuf);
    let result = tera.render("index.html", &context);
    if result.is_err() {
        return Err(format!("render error: {}", result.err().unwrap().to_string()));
    }

    fs::write(target_dir.join("index.html"), &result.unwrap()).unwrap();

    return Ok(());
}

fn main() {
    let command = cli::build_cli().get_matches();

    /* ====== step.1 preparing tera engine */
    // build a absolute template path
    let root_value = PathBuf::from(command.value_of("root").unwrap());
    let root_path = fs::canonicalize(&root_value).unwrap();
    let root_dir = root_path.to_str().unwrap();

    let target_dir = PathBuf::from(command.value_of("target").unwrap());
    fs::create_dir( target_dir.join("blog") ).unwrap();

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

    /* ====== step.2 parsing all blog/gits file*/
    // all blogs
    let blog_pattern = root_dir.to_string() + "/content/blog/**/*.md";
    let blog_glob = glob(&blog_pattern).expect("Invalid glob");

    let all_blog_path: Vec<_> = blog_glob.filter_map(|e| e.ok()).collect();

    let mut all_code = HashSet::<String>::new();
    let mut all_blog: Vec<blog::Blog> = vec![];
    for ref blog_path in all_blog_path {

        println!("Parsing markdown file: {:?}", blog_path);

        if let Ok(blog) = blog::parse(blog_path) {
            if all_code.contains( &blog.code ) {
                panic!( format!("Repeated blog's code from {}!", blog_path.to_str().unwrap()));
            }
            all_code.insert(blog.code.clone());

            // collect basic info from full blog
            let mut blog_short : blog::Blog = Default::default();
            blog_short.title = blog.title.clone();
            blog_short.desc = blog.desc.clone();
            blog_short.date = blog.date.clone();
            blog_short.code = blog.code.clone();
            blog_short.todo = blog.todo;

            // do convert and render
            let html = blog::convert(&blog.raw);
            let full_html = blog::render(&mut tera, &html, &blog).unwrap();

            // create target fold and index.html
            fs::create_dir( target_dir.join("blog").join(&blog.code) ).unwrap();
            fs::write(target_dir.join("blog").join(&blog.code).join("index.html"), &full_html).unwrap();

            // copy resource files from root to target
            let resource_pattern = root_dir.to_string() + "/content/blog/" + &blog.code + "/*";
            let resource_glob = glob(&resource_pattern).expect("Invalid glob");
            let all_resource: Vec<_> = resource_glob.filter_map(|e| e.ok()).collect();
            for ref res in all_resource {
                let dst = target_dir.join("blog").join(&blog.code).join( res.file_name().unwrap().to_str().unwrap() );
                fs::copy(res, dst).unwrap();
            }

            // copy sevlet files from source to target
            if blog.sveltes.len() > 0 {
                let src_pattern = root_dir.to_string() + "/svelte/" + &blog.code + "/public/build/*";
                let src_glob = glob(&src_pattern).expect("Invalid glob");
                let all_src: Vec<_> = src_glob.filter_map(|e| e.ok()).collect();
                if all_src.len() == 0 {
                    panic!("Frontmarker has svelte, but can'f find svelte's target!");
                }
                for ref src in all_src {
                    let dst = target_dir.join("blog").join(&blog.code).join( src.file_name().unwrap().to_str().unwrap() );
                    fs::copy(src, dst).unwrap();
                }
            }

            all_blog.push(blog_short);
        } else {
            panic!( format!("Parse blog {} error!", blog_path.to_str().unwrap()));
        }
    }

    // load all gits from local database
    // TODO

    /* ====== step.2 create blog/gist index file*/
    create_blog_index(tera, &target_dir, &mut all_blog).unwrap();

    /* ====== step.3 create other pages*/
    // TODO

    println!("Building site done!");
}
