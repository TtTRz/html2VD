const HtmlParser = require("../pkg/html2VD")


const html = `
  <div class='title'>
    <span class='span'>
      <span>hello</span>
    </span>
  </div>
  <div>
    title
  </div>
`

const filterHtml = (html) => {
  return html.trim().replace(/[\n]+[\s]*/g,"")
}

console.log(HtmlParser.html2VD(html));

