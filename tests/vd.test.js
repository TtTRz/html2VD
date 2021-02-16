const { TestScheduler } = require('jest')
const {html_2_vd} = require('./pkg/html2VD.js')
const VDD = require('./utils/vdDiff.js');

describe("parse simple html", () => {
  test("single tag", () => {
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
            inner_html: "Rom",
            node_type: 'TextNode',
          }
        ]
      }
    ]
  }
    let vdd = new VDD();
    let vdObj = html_2_vd(html)
    expect(vdObj).toEqual(targetObj)
    vdd.diff_vd(vdObj, targetObj);
    let diff_result = vdd.diff_result;
    expect(diff_result).toBe(true)
  })

  test("multiple tag", () => {
    const html = `
      <div>
        Rom
      </div>
      <span>
        Chung
      </span>
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
              inner_html: "Rom",
              node_type: 'TextNode',
            }
          ]
        },
        {
          tag: 'span',
          inner_html: null,
          attrs: [],
          node_type: 'ElementNode',
          children: [
            {
              inner_html: "Chung",
              node_type: 'TextNode',
            }
          ]
        }
      ]
    }
    let vdd = new VDD();
    let vdObj = html_2_vd(html)
    expect(vdObj).toEqual(targetObj)
    vdd.diff_vd(vdObj, targetObj);
    let diff_result = vdd.diff_result;
    expect(diff_result).toBe(true)
  })
})

describe("parse simple html with attrs", () => {
  test("single tag with attrs", () => {
    const html = `
    <div class="title" id="name">
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
        attrs: [{
          name: "class",
          value: "title"
        }, {
          name: "id",
          value: "name"
        }],
        node_type: 'ElementNode',
        children: [
          {
            inner_html: "Rom",
            node_type: 'TextNode',
          }
        ]
      }
    ]
  }
    let vdd = new VDD();
    let vdObj = html_2_vd(html)
    expect(vdObj).toEqual(targetObj)
    vdd.diff_vd(vdObj, targetObj);
    let diff_result = vdd.diff_result;
    expect(diff_result).toBe(true)
  })

  test("multiple tag with attrs", () => {
    const html = `
      <div class="title" style="width: 100px">
        Rom
      </div>
      <span id="rom">
        Chung
      </span>
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
          attrs: [{
            name: "class",
            value: "title"
          }, {
            name: "style",
            value: "width: 100px",
          }],
          node_type: 'ElementNode',
          children: [
            {
              inner_html: "Rom",
              node_type: 'TextNode',
            }
          ]
        },
        {
          tag: 'span',
          inner_html: null,
          attrs: [{
            name: "id",
            value: "rom"
          }],
          node_type: 'ElementNode',
          children: [
            {
              inner_html: "Chung",
              node_type: 'TextNode',
            }
          ]
        }
      ]
    }
    let vdd = new VDD();
    let vdObj = html_2_vd(html)
    expect(vdObj).toEqual(targetObj)
    vdd.diff_vd(vdObj, targetObj);
    let diff_result = vdd.diff_result;
    expect(diff_result).toBe(true)
  })
})

describe("parse complex html", () => {
  test("single tag", () => {
    const html = `
      <div>
        Rom
        <span>
          Chung
        </span>
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
              inner_html: "Rom",
              node_type: 'TextNode',
            },
            {
              tag: 'span',
              inner_html: null,
              attrs: [],
              node_type: 'ElementNode',
              children: [
                {
                  inner_html: "Chung",
                  node_type: 'TextNode',
                }
              ]
            }
          ]
        }
      ]
    }
    let vdd = new VDD();
    let vdObj = html_2_vd(html)
    expect(vdObj).toEqual(targetObj)
    vdd.diff_vd(vdObj, targetObj);
    let diff_result = vdd.diff_result;
    expect(diff_result).toBe(true)
  })
})