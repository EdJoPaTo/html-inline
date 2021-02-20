use std::fs;

use regex::{Captures, Regex};

mod cli;

fn main() {
    let matches = cli::build().get_matches();

    let html_path = matches
        .value_of("html path")
        .expect("could not read html file path from command line");

    let output_path = matches.value_of("output path");

    let html_content = fs::read_to_string(html_path).expect("failed to read html file");

    // TODO: xpath is probably a better idea than regex
    let regex = Regex::new(r#"<img src="([^"]+)""#).expect("failed to parse regex");

    let new_content = regex.replace_all(&html_content, |caps: &Captures| {
        let path = &caps[1];
        let img_data = fs::read(path).expect("failed to read image");
        let base = image_base64_wasm::vec_to_base64(img_data);
        format!(r#"<img src="{}""#, base)
    });

    if let Some(output_path) = output_path {
        fs::write(output_path, new_content.to_string())
            .expect("failed to write output html to file");
    } else {
        println!("{}", new_content);
    }
}
