const elInput = document.getElementById("searchInput")
const elButton = document.getElementById("searchButton")
const elResults = document.getElementById("results")

let loadingTimeout = null
function showLoading() {
    let i = 0
    loadingTimeout = setInterval(() => {
        i = ++i % 4
        elResults.innerHTML = "Loading results " + Array(i + 1).join(".")
    }, 250)
}

function setResults(html) {
    if (loadingTimeout !== null) {
        clearInterval(loadingTimeout)
    }
    elResults.innerHTML = html
}

function search() {
    showLoading()

    const query = elInput.value
    fetch(`stories?query=${query}`)
        .then((response) => response.json())
        .then((results) => {
            if (results.error !== null) {
                throw results.error
            }
            else if (results.stories.length === 0) {
                setResults("No results found")
            }
            else {
                let html = "<ul>"
                for (const i in results.stories) {
                    const story = results.stories[i]
                    html += `<li><a href="${story.url}">${story.title}</a>&nbsp; (${story.score})</li>`
                }
                html += "</ul>"
                setResults(html)
            }
        })
        .catch((error) => {
            setResults(`<div id="error">Error: ${error}</div>`)
        })
}

elButton.addEventListener("click", search)
elInput.addEventListener("keyup", event => {
    if (event.key === "Enter") {
        elButton.click()
        event.preventDefault()
    }
})
