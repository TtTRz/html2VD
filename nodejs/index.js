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

HtmlParser.parse(html.trim().replace(/[\n]+[\s]*/g,""))