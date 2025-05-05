#[cfg(test)]
mod tests {
  use core::hash;
use std::fs;
  use std::path::Path;
  use wasm_bindgen_test::wasm_bindgen_test;
  use crate::{ md_to_html, md_to_parser};

  #[test]
  fn test_md_to_html() {
    // Read markdown from tests/main.md
    let md_path = "tests/main.md";
    let md_content = fs::read_to_string(md_path)
        .expect("Should be able to read the markdown file");
    
    // Initialize HTML string
    let mut html_content = String::new();
    
    // Convert markdown to HTML
    let parser = md_to_parser(&md_content);
    pulldown_cmark::html::push_html(&mut html_content, parser);
    
    // Write HTML to tests/main.html
    let html_path = "tests/main.html";
    fs::write(html_path, html_content)
        .expect("Should be able to write HTML to output file");
    
    // Verify file was written successfully
    assert!(Path::new(html_path).exists(), "Output HTML file was not created");
  }

  #[test]
  fn get_language_list_hash() {
    use crate::{hash_lang_name, CODE_BLOCK_SUPPORTED_LANGUAGES};
    let mut hashes = Vec::new();
    for lang in CODE_BLOCK_SUPPORTED_LANGUAGES.iter() {
      let hash = hash_lang_name(lang);
      hashes.push(hash);
      println!("Language: {}, Hash: {}", lang, hash);
    }
    print!("Hash list: [");
    for hash  in hashes.iter() {
      print!("{},", hash);
    }
    print!("]");
  }

  #[test]
  fn print_parser() {
    // Read markdown from tests/main.md
    let md_path = "tests/main.md";
    let md_content = fs::read_to_string(md_path)
        .expect("Should be able to read the markdown file");
    
    // Convert markdown to HTML
    let parser = md_to_parser(&md_content);

    for event in parser {
      println!("{:#?}", event);
    }
  }

  wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
  #[wasm_bindgen_test]
  fn main() {
    // Read markdown from tests/main.md
    let md_content = r#####"
    # markdown-render
This markdown render support `katex`, `emoji`, **spoiler container** and automatically add **heading link**. (based on markdown-it)

## Installation
```
npm install @revival0728/markdown-render
```

## Adding CSS
To add CSS to your web page.

use

```css
@import url('https://cdn.jsdelivr.net/npm/@revival0728/markdown-render@1.0.0/index.css');
```
in your global css

or

```js
import 'node_modules/@revival0728/markdown-render/index.css'
```
in your javascript file

## Usage
```js
const mdRender = require('@revival0728/markdown-render');

let renderResult = mdRender.renderMdtoHtml(markdownAsString)
document.getElementById('my-mdContent').innerHTML = renderResult.content
```

It also support adding data before markdown and return the data as key **data** in the return object.

If you render the following markdown text with the code above, you will get the result below.

```markdown
---
tag: 'example'
---
# H1
content
```

```js
{
    content: '<div class="markdown-body" id="markdown-body"><h1>H1 <a id="H1" href="#H1"><i class="bi bi-link" style="color: rgb(26, 83, 255);"></i></a></h1><p>content</p></div>',
    data: {tag: 'example'}
}
```

## About Render Function
The `renderMdtoHtml()` function has two arguments.

1. `mdString`: pass the markdown content as string
2. `options`: pass the object as the options for this function

### About the options object
Below are the supported options

1. `autoChangeLine` (type: boolean): automatically change line when rendering the markdown content
2. `withIndent` (type: boolean): automatically indent the markdown content

The `autoChangeLine` is useful when editing a handout or blog because you don't have to hit the enter key twice.

The `withIndent` is useful when the web page only shows the markdown content.

## Katex Support
Only support **$** when rendering the katex syntax

```markdown
$e^{i\pi} + 1 = 0$
```
$e^{i\pi} + 1 = 0$

```markdown
$$
x = \begin{cases}
   x \quad \text{if } x \geq 0 \\
   -x \quad \text{if } x \lt 0
\end{cases}
$$
```
$$
x = \begin{cases}
   x \quad \text{if } x \geq 0 \\
   -x \quad \text{if } x \lt 0
\end{cases}
$$

To show character **$** in your markdown content, use **\$** instead.

## Code Box Highlight and Line Numbers
If you want to highlight the code in a code box, add the language name after the code box syntax.

The `<codebox>` below represents **```**

```markdown
<codebox>python
print('hello, world')
<codebox>
```

If you want to add line numbers to the code box, add `:setNumber` after the language name.

```markdown
<codebox>python :setNumber
print('hello, world')
<codebox>
```

```python :setNumber
def main():
   print('hello, world')
```
    "#####;
    
    let html_content = md_to_html(&md_content);
    
    println!("{:#?}", html_content);
  }
}
