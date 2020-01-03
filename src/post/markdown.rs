use super::katex::{create_katex_block, create_katex_inline};
use super::plantuml::create_plantuml_svg;
use super::pygments::create_code_block;
use comrak::ComrakOptions;

lazy_static::lazy_static! {
    static ref COMRAK_OPTIONS: ComrakOptions = ComrakOptions {
        hardbreaks: false,
        smart: true,
        github_pre_lang: false,
        default_info_string: None,
        unsafe_: true,
        ext_strikethrough: true,
        ext_tagfilter: false,
        ext_table: true,
        ext_autolink: true,
        ext_tasklist: true,
        ext_superscript: true,
        ext_header_ids: Some("header".to_owned()),
        ext_footnotes: true,
        ext_description_lists: true,
        ..ComrakOptions::default()
    };
    static ref INLINE_MATH_REGEX: regex::Regex = regex::Regex::new(r#"(\$|\\\()(.*?)(\$|\\\))"#).expect("valid regex");
    //static ref INLINE_MATH_REGEX: regex::Regex = regex::Regex::new(r#"\$(.*?)\$"#).expect("valid regex");
}

pub struct FormatResponse {
    pub output: String,
    pub include_katex_css: bool,
}

fn format_code(lang: &str, src: &str) -> Result<FormatResponse, Box<dyn std::error::Error>> {
    // render plantuml code blocks into an inline svg
    if lang == "plantuml" {
        let svg = create_plantuml_svg(src)?;
        let svg = svg.replace(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#,
            "",
        );

        return Ok(FormatResponse {
            output: format!("<figure>{}</figure>", svg),
            include_katex_css: false,
        });
    }
    // render katex code blocks into an inline math
    if lang == "katex" {
        return Ok(FormatResponse {
            output: create_katex_block(src)?,
            include_katex_css: true,
        });
    }

    // otherwise, pass it to pygments
    let html = create_code_block(src, lang)?;

    Ok(FormatResponse {
        output: html,
        include_katex_css: false,
    })
}

fn wrap_image_in_figure(
    link: &comrak::nodes::NodeLink,
    alt: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let title = String::from_utf8_lossy(link.title.as_ref());
    let url = String::from_utf8_lossy(link.url.as_ref());
    if title.len() > 0 {
        Ok(format!(
            r#"<figure><img src="{}" alt="{}" title="{}"><figcaption>{}</figcaption></figure>"#,
            url, alt, title, title
        ))
    } else {
        Ok(format!(
            r#"<figure><img src="{}" alt="{}"></figure>"#,
            url, alt
        ))
    }
}

pub fn format_markdown(src: &str) -> Result<FormatResponse, Box<dyn std::error::Error>> {
    use comrak::nodes::{AstNode, NodeValue};
    use comrak::{format_html, parse_document, Arena};

    let arena = Arena::new();

    // parse math
    // TODO: move into markdown only when in paragraphs
    let mut found_inline_tex: bool = false;
    let src =
        INLINE_MATH_REGEX.replace_all(src, |caps: &regex::Captures| {
            match create_katex_inline(caps.get(2).expect("3 capture groups").as_str()) {
                Ok(s) => {
                    found_inline_tex = true;
                    s.trim().to_owned()
                }
                Err(e) => {
                    let s = caps.get(2).expect("3 capture groups").as_str().to_owned();
                    eprintln!("Failed to parse `{}` as inline KaTeX: {:?}", s, e);
                    s
                }
            }
        });

    let root = parse_document(&arena, src.as_ref(), &COMRAK_OPTIONS);

    fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &mut F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnMut(&'a AstNode<'a>) -> Result<(), Box<dyn std::error::Error>>,
    {
        f(node)?;
        for c in node.children() {
            iter_nodes(c, f)?;
        }
        Ok(())
    }

    let mut use_katex_css = found_inline_tex;
    iter_nodes(root, &mut |node| {
        let value = &mut node.data.borrow_mut().value;
        match value {
            NodeValue::CodeBlock(ref block) => {
                let lang = String::from_utf8_lossy(block.info.as_ref());
                let source = String::from_utf8_lossy(block.literal.as_ref());
                let FormatResponse {
                    output,
                    include_katex_css,
                } = format_code(&lang, &source)?;
                if include_katex_css {
                    use_katex_css = true;
                }
                let highlighted: Vec<u8> = Vec::from(output.into_bytes());
                *value = NodeValue::HtmlInline(highlighted);
            }
            NodeValue::Paragraph => {
                if node.children().count() == 1 {
                    let first_child = &node.first_child().unwrap();
                    let first_value = &first_child.data.borrow().value;
                    if let NodeValue::Image(link) = first_value {
                        if first_child.children().count() > 0 {
                            let mut alt: String = String::default();
                            for child in first_child.children() {
                                if let NodeValue::Text(t) = &child.data.borrow().value {
                                    alt.push_str(&String::from_utf8_lossy(&t));
                                }
                                child.detach();
                            }
                            first_child.detach();
                            let figure = wrap_image_in_figure(&link, &alt)?;
                            let figure: Vec<u8> = Vec::from(figure.into_bytes());
                            *value = NodeValue::HtmlInline(figure);
                        }
                    }
                }
            }
            // TODO: this shit breaks everything
            //NodeValue::Text(ref text) => {
            //    // convert inline math
            //    let text = std::str::from_utf8(text).expect("valid utf-8 text");
            //    let text = INLINE_MATH_REGEX.replace_all(text, |caps: &regex::Captures| {
            //        match create_katex_inline(caps.get(2).expect("3 capture groups").as_str()) {
            //            Ok(s) => s.trim().to_owned(),
            //            Err(e) => {
            //                let s = caps.get(2).expect("3 capture groups").as_str().to_owned();
            //                eprintln!("Failed to parse `{}` as inline KaTeX: {:?}", s, e);
            //                s
            //            }
            //        }
            //    });
            //    let text: String = text.as_ref().to_owned();
            //    *value = NodeValue::Text(text.into_bytes());
            //}
            _ => {}
        }
        Ok(())
    })?;

    let mut output: Vec<u8> = Vec::with_capacity((src.len() as f64 * 1.2) as usize);
    format_html(root, &COMRAK_OPTIONS, &mut output).expect("can format HTML");
    let output = String::from_utf8(output).expect("valid utf-8 generated HTML");
    Ok(FormatResponse {
        output,
        include_katex_css: use_katex_css,
    })
}
