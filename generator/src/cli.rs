use clap::{crate_authors, crate_description, crate_version, App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("gen")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("root")
                .short("r")
                .long("root")
                .takes_value(true)
                .default_value("site")
                .help("Directory to use as root of project")
        )
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .takes_value(true)
                .default_value("public")
                .help("Directory to use as target of project")
        )
}
