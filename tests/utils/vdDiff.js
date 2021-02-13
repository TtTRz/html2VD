class VDD {

  diff_result = false;
  is_equal = false;
  constructor() {
    this.diff_result = true;
    this.is_equal = true;
  }

  diff_vd = (prevVdObj, nextVdObj) => {
    for (let key in prevVdObj) {
      switch (key) {
        case "children":  
          this.diff_children(prevVdObj, nextVdObj)
          break
        case "attrs": 
          this.diff_attrs(prevVdObj, nextVdObj)
          break
        default: 
          this.is_equal = prevVdObj[key] === nextVdObj[key] ? true : false
          return
      }
    }
  }

  diff_children = (prevVdObj, nextVdObj) => {
    if (prevVdObj.children.length !== nextVdObj.children.length) {
      this.is_equal = false
      return 
    }
    for(let i in prevVdObj.children) {
      this.diff_vd(prevVdObj.children[i], nextVdObj.children[i])
    }
  }

  diff_attrs = (prevVdObj, nextVdObj) => {
    if (prevVdObj.attrs.length !== nextVdObj.attrs.length) {
      this.is_equal = false
      return 
    }
    let prevVdAttrsObj = {};
    let nextVdAttrsObj = {};
    for(let i in prevVdObj.attrs) {
      prevVdAttrsObj[i] = prevVdObj.attrs[i];
      nextVdAttrsObj[i] = nextVdObj.attrs[i];
    }
    for(let i in prevVdAttrsObj) {
      if (prevVdAttrsObj[i] !== nextVdAttrsObj[i]) {
        this.is_equal = false;
        return 
      }
    }
  }

}

module.exports = VDD;