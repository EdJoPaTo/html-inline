use clap::{App, AppSettings, Arg};

pub fn build() -> App<'static, 'static> {
    App::new("HTML Image Inline base64")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .global_setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("html path")
                .value_name("FILE")
                .takes_value(true)
                .required(true)
                .help("Path to html file"),
        )
        .arg(
            Arg::with_name("output path")
                .long("output")
                .short("o")
                .value_name("FILE")
                .takes_value(true)
                .help("Path to output the final html"),
        )
}
