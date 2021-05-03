use std::fs;
use std::io::Read;

use html5ever::tendril::TendrilSink;
use html5ever::{local_name, namespace_url, ns, QualName};
use kuchiki::NodeRef;

fn load_vec(src: &str) -> anyhow::Result<Vec<u8>> {
    let data = if src.starts_with("http://") || src.starts_with("https://") {
        let mut data = Vec::new();
        ureq::get(src)
            .call()?
            .into_reader()
            .read_to_end(&mut data)?;
        data
    } else {
        fs::read(src)?
    };
    Ok(data)
}

fn load_string(src: &str) -> anyhow::Result<String> {
    let body = if src.starts_with("http://") || src.starts_with("https://") {
        ureq::get(src).call()?.into_string()?
    } else {
        fs::read_to_string(src)?
    };
    Ok(body)
}

/// Return the html String with external resources inlined
/// External resources that could not be loaded are listed in the second element in the returned pair.
///
/// # Errors
///
/// Will return `Err` when the input is not valid html.
pub fn html_inline(html: &str) -> anyhow::Result<(String, Vec<String>)> {
    let doc = kuchiki::parse_html().one(html);

    let mut failed_sources = Vec::new();

    for img_ref in doc
        .select("img[src]")
        .expect("image selector could not be parsed")
    {
        let mut img_node = img_ref.attributes.borrow_mut();
        let src = img_node.get_mut("src").expect("img needs src to work");
        if !src.starts_with("data:image/") {
            match load_vec(src) {
                Ok(img_data) => {
                    let base64 = image_base64_wasm::vec_to_base64(img_data);
                    *src = base64;
                }
                Err(err) => failed_sources.push(format!("image {}: {}", src, err)),
            }
        }

        img_node.remove("srcset");
    }

    for link_ref in doc
        .select("link[rel='stylesheet'][href]")
        .expect("link stylesheet selector could not be parsed")
    {
        let link_node = link_ref.attributes.borrow();
        let href = link_node
            .get("href")
            .expect("link stylesheet needs a href to work");

        match load_string(href) {
            Ok(style_text) => {
                let qual_name = QualName::new(None, ns!(html), local_name!("style"));
                let style = NodeRef::new_element(qual_name, vec![]);
                style.append(NodeRef::new_text(style_text.trim()));

                link_ref.as_node().insert_after(style);
                link_ref.as_node().detach();
            }
            Err(err) => failed_sources.push(format!("style {}: {}", href, err)),
        }
    }

    let mut buf = Vec::new();
    doc.serialize(&mut buf)?;
    let result = String::from_utf8(buf)?;
    Ok((result, failed_sources))
}

#[test]
fn inline_img_start_file_works() {
    let html = r#"<html><head></head><body><div><img class="something" src="testdata/white-pixel.png"></img></div></body></html>"#;
    let (result, errors) = html_inline(html).unwrap();
    println!("result {}", result);
    assert!(errors.is_empty());
    assert!(result.starts_with(
        r#"<html><head></head><body><div><img class="something" src="data:image/png;base64,"#
    ));
    assert!(result.ends_with(r#"="></div></body></html>"#));
}

#[test]
fn inline_img_empty_file_works() {
    let html = r#"<html><head></head><body><div><img class="something" src="testdata/white-pixel.png" /></div></body></html>"#;
    let (result, errors) = html_inline(html).unwrap();
    println!("result {}", result);
    assert!(errors.is_empty());
    assert!(result.starts_with(
        r#"<html><head></head><body><div><img class="something" src="data:image/png;base64,"#
    ));
    assert!(result.ends_with(r#"="></div></body></html>"#));
}

#[test]
fn inline_img_unknown_stays_the_same() {
    let html = r#"<html><head></head><body><div><img class="something" src="testdata/non-existant.png"></div></body></html>"#;
    let (result, errors) = html_inline(html).unwrap();
    println!("result {}", result);
    assert_eq!(
        errors,
        &["image testdata/non-existant.png: No such file or directory (os error 2)"]
    );
    assert_eq!(
        result,
        r#"<html><head></head><body><div><img class="something" src="testdata/non-existant.png"></div></body></html>"#
    );
}

#[test]
fn inline_web_img_works() {
    let html = r#"<html><head></head><body><div><img class="something" src="https://via.placeholder.com/1x1"></div>"#;
    let (result, errors) = html_inline(html).unwrap();
    println!("result {}", result);
    assert!(errors.is_empty());
    assert!(result.starts_with(
        r#"<html><head></head><body><div><img class="something" src="data:image/png;base64,"#
    ));
    assert!(result.ends_with(r#"="></div></body></html>"#));
}

#[test]
fn inline_stylesheet_file_works() {
    let html = r#"<html><head><link rel="stylesheet" href="testdata/simple.css" /></head><body></body></html>"#;
    let (result, errors) = html_inline(html).unwrap();
    assert!(errors.is_empty());
    assert_eq!(
        result,
        r#"<html><head><style>h1 { color: blue; }</style></head><body></body></html>"#
    );
}

#[test]
fn inline_web_stylesheet_works() {
    let html = r#"<html><head><link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/emojify.js/1.1.0/css/basic/emojify.min.css" integrity="sha256-UOrvMOsSDSrW6szVLe8ZDZezBxh5IoIfgTwdNDgTjiU=" crossorigin="anonymous" /></head><body></body></html>"#;
    let (result, errors) = html_inline(html).unwrap();
    println!("result {}", result);
    assert!(errors.is_empty());
    assert!(result.starts_with("<html><head><style>"));
    assert!(result.ends_with("</style></head><body></body></html>"));
}
