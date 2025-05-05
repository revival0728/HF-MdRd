---
title: HF-MdRd README
description: An brief introduction to HF-MdRd
---

# HF-MdRd

A markdown renderer written in Rust was compiled to wasm.

It can be used in both frontend and backend.

## Usage

### Node.js
1. Install `hf-mdrd`
```bash
npm install hf-mdrd
```

2. Import the module
```js
import { md_to_html } from "hf-mdrd";

const html = md_to_html(markdown_content);
```

3. Add CSS to the frontend
```js
import "hf-mdrd/css/hf-mdrd.css";
```

### Browser
```html
<html>
  <head>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/hf-mdrd@0.1.10/css/hf-mdrd.min.css" crossorigin="anonymous">
  </head>
  <body>
    <script type="module" type="text/javascript">
      import init, { md_to_html } from 'https://cdn.jsdelivr.net/npm/hf-mdrd@0.1.10/browser/hf-mdrd.min.mjs';
      await init();
      const html = md_to_html(markdown_text);
    </script>
  </body>
</html>
```

## Basic Syntax
using [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark/) to handle basic syntax

`Code span`

**Bold**

*Italic*

~~Strikethrough~~

AutoLink: <https://hf-mdrd.hsingfu.tw>

Link: [link](https://hf-mdrd.hsingfu.tw)

![image](https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/icons/markdown.svg)

- List
  - List
- List

1. Ordered
2. List

> Don't believe everything you
> read on the Internet.
>
> — Abraham Lincoln

---
Break Here


## CodeBlock
```rust
fn main() {
  println!("hello, world!");
}
```

### Highlight
using [highlight.js](https://highlightjs.org/) to handle syntax highlight

#### Supported Language
- python
- rust
- typescript
- xml
- html
- fortran
- go
- java
- javascript
- json
- kotlin
- latex
- lua
- markdown
- bash
- c
- cpp
- css

### Adding Line Number
```rust :setNumber
fn main() {
  println!("hello, world!");
}
```

```rust=
fn main() {
  println!("hello, world!");
}
```

## Math
using [katex](https://katex.org/) to handle math expression

Euler Formula: $e^{ix} = cos(x) + isin(x)$

Euler Formula:
$$e^{ix} = cos(x) + isin(x)$$

## Spoiler
:::spoiler
The
:::

:::spoiler spoiler
Content
:::

:::spoiler `spoiler`
Inside
:::

:::spoiler $spoiler$
Spoiler
:::

## Table
| header | row |
| ------ | --- |
| body   | row |

## API
```ts
function md_to_html(markdown_content: string): string
```