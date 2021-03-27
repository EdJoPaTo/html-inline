use std::error::Error;
use std::fs;
use std::io::Cursor;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;

/// Return the html String with exteernal resources inlined
///
/// # Errors
///
/// Will return `Err` when external resources could not be loaded or the input is not valid html.
pub fn html_inline(html: &str) -> Result<String, Box<dyn Error>> {
    let mut reader = Reader::from_str(html);
    reader.trim_text(true);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf)? {
            Event::Eof => break,
            Event::Start(ref e) if e.name() == b"img" => {
                let mut elem = BytesStart::owned_name("img");

                for attribute in e.attributes() {
                    let attribute = attribute?;
                    match attribute.key {
                        b"src" => {
                            if attribute.value.starts_with(b"data:image/") {
                                elem.push_attribute(attribute);
                            } else if attribute.value.starts_with(b"http://")
                                || attribute.value.starts_with(b"https://")
                            {
                                println!(
                                    "TODO: remote location download not yet implemented {:?}",
                                    String::from_utf8(attribute.value.to_vec())
                                );
                                elem.push_attribute(attribute);
                            } else {
                                let path = String::from_utf8(attribute.value.to_vec())?;
                                let img_data = fs::read(path)?;
                                let base64 = image_base64_wasm::vec_to_base64(img_data);
                                elem.push_attribute(("src", base64.as_ref()));
                            }
                        }
                        _ => elem.push_attribute(attribute),
                    }
                }

                writer.write_event(Event::Start(elem))?;
            }
            e => writer.write_event(&e)?,
        }
        buf.clear();
    }

    let result = writer.into_inner().into_inner();
    Ok(String::from_utf8(result)?)
}

#[test]
fn check_img_src_file() {
    let html = r#"<div><img class="something" src="testdata/white-pixel.png"></img></div>"#;
    let result = html_inline(html).unwrap();
    assert!(result.starts_with(r#"<div><img class="something" src="data:image/png;base64,"#));
    assert!(result.ends_with(r#"="></img></div>"#));
}
