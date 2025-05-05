mod test;

use pulldown_cmark::{TextMergeStream, CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use wasm_bindgen::prelude::*;

const SET_NUMBER_FLAG: &str = ":setNumber";
const PLAIN_TEXT: &str = "plaintext";
const SPIOLER: &str = ":::spoiler";
const CODE_BLOCK_SUPPORTED_LANG_HASH: [u64; 18] = [123185905036052,1913061632,8076364826803074327,7892345,1746666781,28833023648627314,26376,1779112988,8509275548630674707,1780024600,117666871801371,465485235300,7107960,7857689625060903943,1644368152,99,6516835,6517603];
const CODE_BLOCK_SUPPORTED_LANGUAGES: [&str; 18] = [
  "python",
  "rust",
  "typescript",
  "xml",
  "html",
  "fortran",
  "go",
  "java",
  "javascript",
  "json",
  "kotlin",
  "latex",
  "lua",
  "markdown",
  "bash",
  "c",
  "cpp",
  "css"
];

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

fn hash_lang_name(lang: &str) -> u64 {
  let mut hash = 0;
  for c in lang.chars() {
    hash ^= (hash << 8) + (c as u64);
  }
  hash
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
  fn highlightjs_simple_highlight(code: &str, language: &str) -> String;
}

fn highlight_code(code: &str, language: &str, set_line_number: bool) -> String {
  let hl_code = highlightjs_simple_highlight(code, language);
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

fn check_language(language: &str) -> bool {
  let hash = hash_lang_name(language);
  for (idx, &lang_hash ) in CODE_BLOCK_SUPPORTED_LANG_HASH.iter().enumerate() {
    if lang_hash == hash {
      return CODE_BLOCK_SUPPORTED_LANGUAGES[idx] == language;
    }
  }
  false
}

#[wasm_bindgen]
pub fn md_to_html(markdwon_input: &str) -> String {
  set_panic_hook();

  let mut events = Vec::new();
  let mut in_code_block = false;
  let mut language = PLAIN_TEXT.to_string();
  let mut set_line_number = false;
  let mut in_heading = false;
  let mut in_spoiler = false;
  let mut is_spoiler_summary = false;
  let mut heading_level: HeadingLevel = HeadingLevel::H1;
  let iterator = TextMergeStream::new(md_to_parser(markdwon_input));
  for event in iterator {
    match event {
      Event::InlineMath(text) => {
        if is_spoiler_summary {
          is_spoiler_summary = false;
          events.push(Event::InlineHtml(CowStr::from("<summary>")));
          events.push(Event::Html(CowStr::from(render_katex(&text, false))));
          events.push(Event::InlineHtml(CowStr::from("</summary>")));
        } else {
          events.push(Event::Html(CowStr::from(render_katex(&text, false))));
        }
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
                language = PLAIN_TEXT.to_string();
                set_line_number = false;

                let args = cow_str.split_whitespace().collect::<Vec<_>>();
                let args_len = args.len();
                if args_len > 0 {
                  language = args[0].to_string();
                  if language.ends_with("=") {
                    language = language[0..language.len()-1].to_string();
                    set_line_number |= true;
                  }
                  if !check_language(&language) {
                    language = PLAIN_TEXT.to_string();
                  }
                }
                if args_len > 1 {
                  set_line_number |= args[1] == SET_NUMBER_FLAG;
                }
                events.push(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::from(language.clone())))));
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
          events.push(Event::Html(CowStr::from(highlight_code(&text, &language, set_line_number))));
        } else if in_heading {
          events.push(Event::Text(text.clone()));
          let hc = text.to_string().replace(" ", "-");
          let heading_tag = format!(r##"<a id="{}" href="#{}"><i class="bi bi-link mdrd-hl"></i></a>"##, hc, hc);
          events.push(Event::Html(CowStr::from(heading_tag)));
        } else {
          // handle custom :::spoiler tag
          for line in text.lines() {
            if line.starts_with(SPIOLER) && !in_spoiler {
              events.push(Event::InlineHtml(CowStr::from("<details>")));
              in_spoiler = true;
              match line.trim_end().split_once(" ") {
                Some((_, summary)) => {
                  events.push(Event::Html(CowStr::from(format!("<summary>{}</summary>", summary))));
                }, 
                None => {
                  is_spoiler_summary = true;
                }
              };
            } else if line.starts_with(":::") && in_spoiler {
              events.push(Event::InlineHtml(CowStr::from("</details>")));
              in_spoiler = false;
              is_spoiler_summary = false;
            } else {
              events.push(Event::Text(CowStr::from(line.to_owned())));
              events.push(Event::Text(CowStr::from("\n")));
            }
          }
        }
      },
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
        if is_spoiler_summary {
          is_spoiler_summary = false;
          events.push(Event::InlineHtml(CowStr::from("<summary>")));
          events.push(event);
          events.push(Event::InlineHtml(CowStr::from("</summary>")));
        } else {
          events.push(event);
        }
      }
    }
  }

  let mut html_output = String::new();
  pulldown_cmark::html::push_html(&mut html_output, events.into_iter());

  html_output
}