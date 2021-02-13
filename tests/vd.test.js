const { TestScheduler } = require('jest')
const {html_2_vd} = require('./pkg/html2VD.js')
const VDD = require('./utils/vdDiff.js');

describe("vd", () => {
  const html = `
    <div>
      Rom
    </div>
  `
  const targetObj = {
    tag: null,
    inner_html: null,
    attrs: [],
    node_type: 'FragmentNode',
    children: [
      {
        tag: 'div',
        inner_html: null,
        attrs: [],
        node_type: 'ElementNode',
        children: [
          {
            tag: null,
            inner_html: "Rom",
            attrs: [],
            node_type: 'TextNode',
            children: []
          }
        ]
      }
    ]
  }
  test("test parse html", () => {
    let vdd = new VDD();
    let vdObj = html_2_vd(html)
    expect(vdObj).toEqual(targetObj)
    vdd.diff_vd(vdObj, targetObj);
    let diff_result = vdd.diff_result;
    expect(diff_result).toBe(true)
  })
})