use clap::{crate_authors, crate_description, crate_version, App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("gen")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("root")
                .short("r")
                .long("root")
                .takes_value(true)
                .default_value("site/")
                .help("Directory to use as root of project")
        )
        .subcommands(vec![
            SubCommand::with_name("build")
                .about("Deletes the output directory if there is one and builds the site")
        ])
}
