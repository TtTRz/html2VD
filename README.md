# html2VD
HTML To VirtualDOM.

Based On Rust.

**Still in development. Do not use it in productional stage.**

## How To Use
```shell
wasm-pack build --target nodejs
```

## Sample
```html
<div>
  <h1>title</h1>
  <div>
    <button onclick="handleclick">change title</button>
  </div>
  <div class="footer">
    <span>Rom</span>
  </div>
</div>
```

```javascript
const HtmlParser = require("../pkg/html2VD.js")

let vd = HtmlParser.html_2_vd(html)

// vd obj
let vdObj = {
  tag: null,
  node_type: "FragmentNode",
  inner_html: null,
  attrs: [],
  children: [{
    tag: "div",
    node_type: "ElementNode",
    inner_html: null,
    attrs: [],
    children: [{
      tag: "h1",
      node_type: "ElementNode",
      inner_html: null,
      attrs: [],
      children: [{
        node_type: "TextNode",
        inner_html: "title",
      }],
    }, {
      tag: "button",
      node_type: "ElementNode",
      inner_html: null,
      attrs: [{
        name: "onclick",
        value: "handleclick"
      }],
      children: [{
        node_type: "TextNode",
        inner_html: "change title",
      }],
    }, {
      tag: "div",
      node_type: "ElementNode",
      inner_html: null,
      attrs: [{
        name: "class",
        value: "footer",
      }],
      children: [{
        tag: "span",
        node_type: "ElementNode",
        inner_html: null,
        attrs: [],
        children: [{
          node_type: "TextNode",
          inner_html: "Rom",
        }],
      }],
    }],
  }],
}
```