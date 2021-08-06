// Cannot extract h3 headings

document.addEventListener('DOMContentLoaded', function() {
  function createTocList (el, selectors) {
    if (typeof(selectors) == "undefined") {
      if (el.querySelectorAll('.section.level2').length < 1) {
        if (el.querySelectorAll('h2').length < 1) {
         return '' 
        }
        selectors =  ['h2', 'h3'] // 'h2, h3'
      } else {
        selectors = ['.section.level2', '.section.level3']
      }
    } //else {
      //console.log("This is the second time")
      //console.log(selectors[0])
      //console.log(el.querySelectorAll("h2, h3").length)
    //}
    
    if (!Array.isArray(selectors)) {
      selectors = [selectors]
    }
    
    var els = el.querySelectorAll(selectors[0])
    if (els.length < 1) {
      return ''
    }
    
    var toc = `<ul>`
    els.forEach(function(el) {
      if (selectors[0].substring(0, 1) == 'h') {
        toc += `<li><a href="#${el.id}">${el.textContent}</a>`
      } else {
        toc += `<li><a href="#${el.id}">${el.firstElementChild.textContent}</a>`
      }
      if (selectors.length > 1) {
        //console.log("Run again")
        toc += createTocList(el, selectors.slice(1))
      }
      toc += '</li>'
    })
    return toc + `</ul>`
  }
  
  const main = document.querySelector('main')
  const toc = document.createElement('nav')
  toc.classList = 'table-of-contents'
  toc.setAttribute('role', "navigation")
  toc.innerHTML = createTocList(main)
  main.insertBefore(toc, main.firstElementChild)
})
