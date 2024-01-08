const search = async (prompt) => {
  const response = await fetch("/api/search", {
    method: 'POST',
    mode: 'cors',
    cache: 'no-cache',
    credentials: 'same-origin',
    headers: {
      'Content-Type': 'text/html',
      'Access-Control-Allow-Origin': 'http://localhost:8080'
    },
    redirect: 'follow',
    referrerPolicy: 'no-referrer',
    body: prompt,
  })
  const json = await response.json()
  const results = document.getElementById('results')
  results.innerHTML = ""
  for ([path, rank] of json) {
    let item = document.createElement('span')
    let anchor = document.createElement('a')
    anchor.href = path
    anchor.innerText = path
    anchor.target = '_blank'
    if (['md', 'ts', 'rs', 'log', 'info', 'sql', 'tsv', 'tab', 'go', 'toml'].includes(anchor.href.split('.').pop()) || 
      ['README', 'slack-desc'].includes(anchor.href.split('/').pop())) {
      let href = anchor.href
      anchor.setAttribute('href', `#${anchor.href}`)
      anchor.setAttribute('target', '_self')
      anchor.onclick = () => {
        let wn = window.open(href); 
        setTimeout(() => {
          let source = wn.document.getElementsByTagName('body')[0].innerHTML 
          console.log(source, wn.document.childNodes[0].innerHTML)
          source = source.replace(/</g, "&lt;").replace(/>/g, "&gt;")
          source = `<pre>${source}</pre>`
          wn.document.write(source)
          wn.document.close()
        }, 250)
        console.log('Done');
      }
    } 
    item.appendChild(anchor)
    item.appendChild(document.createElement('br'))
    results.appendChild(item)
  }
}

let query = document.getElementById('query')
let currentSearch = Promise.resolve()

query.addEventListener('keypress', (e) => {
  if (e.key == 'Enter') {
    currentSearch.then(() => search(query.value))
  }
})