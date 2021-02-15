# html2VD
HTML To VirtualDOM.

**Still in development. Do not use it in productional stage.**

## how to use
```shell
wasm-pack build --target nodejs
```
## sample
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
```js
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
        tag: null,
        node_type: "TextNode",
        inner_html: "title",
        attrs: [],
        children: [],
        parent: null,
      }],
      parent: null,
    }, {
      tag: "button",
      node_type: "ElementNode",
      inner_html: null,
      attrs: [{
        name: "onclick",
        value: "handleclick"
      }],
      children: [{
        tag: null,
        node_type: "TextNode",
        inner_html: "change title",
        attrs: [],
        children: [],
        parent: null,
      }],
      parent: null
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
          tag: null,
          node_type: "TextNode",
          inner_html: "Rom",
          attrs: [],
          children: [],
          parent: null
        }],
        parent: null
      }],
      parent: null,
    }],
    parent: null,
  }],
  parent: null,
}
```