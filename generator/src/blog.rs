use std::fs;
use std::path::PathBuf;

 #[derive(Debug,Default)]
pub struct Blog {
    title:      String,
    template:   String,
    target:     String,

    date:       String,
    tags:       Vec<String>,

    floder:     String,
    resource:   bool,

    raw:        String,
}

pub fn parse(path: &PathBuf) -> Result<Blog, String> {
    let blog : Blog = Default::default();

    return Ok(blog);
}


