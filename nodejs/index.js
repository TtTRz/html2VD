const HtmlParser = require("../pkg/html2VD")
const {parse_template} = require("templ8m8s")


const times = new Array(100).fill(0);

const html = `
  <div>
    <span>
    hello
    </span>
    <span>rom</span>
       ${times.map(() => { return "<span></span>"}).join("")}

  </div>
`
//    ${times.map(() => { return "<span></span>"}).join("")}



const filterHtml = (html) => {
  return html.trim().replace(/[\n]+[\s]*/g,"")
}

console.time("wasm")
// console.log(filterHtml(html))
HtmlParser.html2VD(filterHtml(html))
console.timeEnd("wasm")
console.log(HtmlParser.html2VD(filterHtml(html))
)




console.time("js")
const vdom = parse_template(filterHtml(html))
// console.log(vdom)
console.timeEnd("js")