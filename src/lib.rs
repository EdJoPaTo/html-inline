use std::error::Error;
use std::fs;

use regex::{Captures, Regex};

/// Return the html String with exteernal resources inlined
///
/// # Errors
///
/// Will return `Err` when external resources could not be loaded or the input is not valid html.
pub fn html_inline(html: &str) -> Result<String, Box<dyn Error>> {
    // TODO: xpath is probably a better idea than regex
    let regex = Regex::new(r#"<img src="([^"]+)""#).expect("failed to parse regex");

    let new_content = regex.replace_all(&html, |caps: &Captures| {
        let path = &caps[1];
        let img_data = fs::read(path).expect("failed to read image");
        let base = image_base64_wasm::vec_to_base64(img_data);
        format!(r#"<img src="{}""#, base)
    });

    Ok(new_content.to_string())
}
