#[cfg(feature = "cli")]
use std::fs;

#[cfg(feature = "cli")]
mod cli;
#[cfg(feature = "cli")]
mod lib;

#[cfg(feature = "cli")]
fn main() {
    let matches = cli::build().get_matches();

    let html_path = matches
        .value_of("html path")
        .expect("could not read html file path from command line");

    let output_path = matches.value_of("output path");

    let html_content = fs::read_to_string(html_path).expect("failed to read html file");

    let (new_content, skipped_sources) =
        lib::html_inline(&html_content).expect("failed to inline external resources");

    for s in skipped_sources {
        eprintln!("SKIPPED {}", s);
    }

    if let Some(output_path) = output_path {
        fs::write(output_path, new_content).expect("failed to write output html to file");
    } else {
        println!("{}", new_content);
    }
}

#[cfg(not(feature = "cli"))]
fn main() {
    panic!("cli was disabled");
}
