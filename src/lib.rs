mod test;

use pulldown_cmark::{TextMergeStream, CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use wasm_bindgen::prelude::*;

fn set_panic_hook() {
  // When the `console_error_panic_hook` feature is enabled, we can call the
  // `set_panic_hook` function at least once during initialization, and then
  // we will get better error messages if our code ever panics.
  //
  // For more details see
  // https://github.com/rustwasm/console_error_panic_hook#readme
  #[cfg(feature = "console_error_panic_hook")]
  console_error_panic_hook::set_once();
}

fn get_parser_options() -> Options {
  let mut options = Options::empty();
  options.insert(Options::ENABLE_STRIKETHROUGH);
  options.insert(Options::ENABLE_TABLES);
  options.insert(Options::ENABLE_MATH);
  options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
  options
}

pub fn md_to_parser(markdwon_input: &str) -> Parser<'_> {
  Parser::new_ext(markdwon_input, get_parser_options())
}


#[wasm_bindgen(module = "/katex.js")]
extern "C" {
  #[wasm_bindgen(js_name = "renderToString")]
  fn katex_render_to_string(html: &str) -> String;
}

#[wasm_bindgen(module = "/highlight.js")]
extern "C" {
  #[wasm_bindgen(js_name = "simpleHighlight")]
  fn highlightjs_simple_highlight(code: &str, langauge: &str) -> String;
}

fn highlight_code(code: &str, langauge: &str, set_line_number: bool) -> String {
  let hl_code = highlightjs_simple_highlight(code, langauge);
  if !set_line_number {
    return hl_code;
  }
  let mut lines = hl_code.lines().collect::<Vec<_>>();
  let mut result = String::new();
  for (i, line) in lines.iter_mut().enumerate() {
    let new_line = format!(r#"<span class="mdrd-ln">{}</span>{}{}"#, i + 1, line, "\n");
    result.push_str(&new_line);
  }

  result
}

fn render_katex(katex: &str, display: bool) -> String {
  if !display {
    return katex_render_to_string(katex);
  }
  let mut result = String::new();
  result.push_str(r#"<div class="math-display">"#);
  result.push_str(&katex_render_to_string(katex));
  result.push_str(r#"</div>"#);
  result
}

#[wasm_bindgen]
pub fn md_to_html(markdwon_input: &str) -> String {
  set_panic_hook();

  const SET_NUMBER_FLAG: &str = ":setNumber";
  const PLAIN_TEXT: &str = "plaintext";
  let mut events = Vec::new();
  let mut in_code_block = false;
  let mut langauge = PLAIN_TEXT.to_string();
  let mut set_line_number = false;
  let mut in_heading = false;
  let mut heading_level: HeadingLevel = HeadingLevel::H1;
  let iterator = TextMergeStream::new(md_to_parser(markdwon_input));
  for event in iterator {
    match event {
      Event::InlineMath(text) => {
        events.push(Event::Html(CowStr::from(render_katex(&text, false))));
      },
      Event::DisplayMath(text) => {
        events.push(Event::Html(CowStr::from(render_katex(&text, true))));
      },
      Event::Start(tag) => {
        match tag {
          Tag::Heading { level, id, classes, attrs } => {
            in_heading = true;
            heading_level = level;
            events.push(Event::Start(Tag::Heading{level, id, classes, attrs}));
          },
          Tag::CodeBlock(code_block_kind) => {
            match code_block_kind {
              CodeBlockKind::Fenced(cow_str) => {
                in_code_block = true;
                langauge = PLAIN_TEXT.to_string();
                set_line_number = false;

                let args = cow_str.split_whitespace().collect::<Vec<_>>();
                let args_len = args.len();
                if args_len > 0 {
                  langauge = args[0].to_string();
                }
                if args_len > 1 {
                  set_line_number = args[1] == SET_NUMBER_FLAG;
                }
                events.push(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::from(langauge.clone())))));
              },
              CodeBlockKind::Indented => {
                events.push(Event::Start(Tag::CodeBlock(CodeBlockKind::Indented)));
              }
            }
          },
          _ => {
            events.push(Event::Start(tag));
          }
        }
      },
      Event::Text(text) => {
        if in_code_block {
          events.push(Event::Html(CowStr::from(highlight_code(&text, &langauge, set_line_number))));
        } else if in_heading {
          events.push(Event::Text(text.clone()));
          let hc = text.to_string().replace(" ", "-");
          let heading_tag = format!(r##"<a id="{}" href="#{}"><i class="bi bi-link mdrd-hl"></i></a>"##, hc, hc);
          events.push(Event::Html(CowStr::from(heading_tag)));
        } else {
          events.push(Event::Text(text));
        }
      }
      Event::End(tag) => {
        match tag {
          TagEnd::CodeBlock => {
            in_code_block = false;
            events.push(Event::End(TagEnd::CodeBlock));
          },
          TagEnd::Heading(level) => {
            assert_eq!(level, heading_level, "Interel Error: Heading level mismatch");
            in_heading = false;
            events.push(Event::End(TagEnd::Heading(level)));
          }
          _ => { events.push(Event::End(tag)); }
        }
      },
      _ => {
        events.push(event);
      }
    }
  }

  let mut html_output = String::new();
  pulldown_cmark::html::push_html(&mut html_output, events.into_iter());

  html_output
}