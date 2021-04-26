use std::fs;
use std::io::{Cursor, Read};

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;
use quick_xml::Writer;

/// Return the html String with exteernal resources inlined
///
/// # Errors
///
/// Will return `Err` when external resources could not be loaded or the input is not valid html.
pub fn html_inline(html: &str) -> anyhow::Result<String> {
    let mut reader = Reader::from_str(html);
    reader.trim_text(true);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf)? {
            Event::Eof => break,
            Event::Start(ref e) if e.name() == b"img" => {
                writer.write_event(Event::Start(replace_img(e)?))?;
            }
            Event::Empty(ref e) if e.name() == b"img" => {
                writer.write_event(Event::Empty(replace_img(e)?))?;
            }
            Event::Empty(ref e)
                if e.name() == b"link"
                    && e.attributes().any(|a| {
                        println!("i saw attributes {:?}", a);
                        a.map_or(false, |a| {
                            a.key == b"rel" && a.value.to_vec() == b"stylesheet"
                        })
                    }) =>
            {
                for attribute in e.attributes() {
                    let attribute = attribute?;
                    if let b"href" = attribute.key {
                        let body = if attribute.value.starts_with(b"http://")
                            || attribute.value.starts_with(b"https://")
                        {
                            let url = String::from_utf8(attribute.value.to_vec())?;
                            ureq::get(&url).call()?.into_string()?
                        } else {
                            let path = String::from_utf8(attribute.value.to_vec())?;
                            fs::read_to_string(path)?
                        };
                        writer.write_event(Event::Start(BytesStart::borrowed_name(b"style")))?;
                        writer.write_event(Event::Text(BytesText::from_plain_str(body.trim())))?;
                        writer.write_event(Event::End(BytesEnd::borrowed(b"style")))?;
                    }
                }
            }
            e => writer.write_event(&e)?,
        }
        buf.clear();
    }

    let result = writer.into_inner().into_inner();
    Ok(String::from_utf8(result)?)
}

fn replace_img(e: &BytesStart) -> anyhow::Result<quick_xml::events::BytesStart<'static>> {
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
                    let url = String::from_utf8(attribute.value.to_vec())?;
                    let mut img_data = Vec::new();
                    ureq::get(&url)
                        .call()?
                        .into_reader()
                        .read_to_end(&mut img_data)?;
                    let base64 = image_base64_wasm::vec_to_base64(img_data);
                    elem.push_attribute(("src", base64.as_ref()));
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

    Ok(elem)
}

#[test]
fn inline_img_start_file_works() {
    let html = r#"<div><img class="something" src="testdata/white-pixel.png"></img></div>"#;
    let result = html_inline(html).unwrap();
    assert!(result.starts_with(r#"<div><img class="something" src="data:image/png;base64,"#));
    assert!(result.ends_with(r#"="></img></div>"#));
}

#[test]
fn inline_img_empty_file_works() {
    let html = r#"<div><img class="something" src="testdata/white-pixel.png" /></div>"#;
    let result = html_inline(html).unwrap();
    assert!(result.starts_with(r#"<div><img class="something" src="data:image/png;base64,"#));

    let end = &result[result.len() - 20..];
    println!("end {}", end);
    assert!(result.ends_with(r#"="/></div>"#));
}

#[test]
fn inline_web_img_works() {
    let html = r#"<div><img class="something" src="https://via.placeholder.com/1x1"></img></div>"#;
    let result = html_inline(html).unwrap();
    assert!(result.starts_with(r#"<div><img class="something" src="data:image/png;base64,"#));
    assert!(result.ends_with(r#"="></img></div>"#));
}

#[test]
fn inline_stylesheet_file_works() {
    let html = r#"<head><link rel="stylesheet" href="testdata/simple.css" /></head>"#;
    let result = html_inline(html).unwrap();
    assert_eq!(result, r#"<head><style>h1 { color: blue; }</style></head>"#);
}

#[test]
fn inline_web_stylesheet_works() {
    let html = r#"<head><link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/emojify.js/1.1.0/css/basic/emojify.min.css" integrity="sha256-UOrvMOsSDSrW6szVLe8ZDZezBxh5IoIfgTwdNDgTjiU=" crossorigin="anonymous" /></head>"#;
    let result = html_inline(html).unwrap();
    assert!(result.starts_with("<head><style>"));
    assert!(result.ends_with("</style></head>"));
}
