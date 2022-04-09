// TODO: Should probably switch `identifierField` and `id` terms.
const BASE_URL = `${window.location.protocol}//${window.location.hostname}:${window.location.port}`;

const Api = {
  shortenUrl: async (object: Object) => {
    const opts = {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(object)
    };

    const res = await fetch(`${BASE_URL}/rpc/shorten-url`, opts);

    if(res.ok) {
      return await res.json();
    } else {
      throw new Error(`${res.status}`);
    }
  }
};

const form = document.querySelector("form");

////////////////////////////////////////////////////////////////////////////////
// TODO: Move dynamic template injections to an SSE/websocket instead.

// Check if it has the custom_url query param
if (new URL(window.location.href).searchParams.has('custom_url')) {
  form?.insertAdjacentHTML("beforeend", "<div class='w-full sm:w-4/5 mt-2 mx-auto text-su-fg-1 dark:text-su-dark-fg-1'><button id='toggle-custom' type='button' class='font-medium underline'>Autogenerate a custom URL for me</button></div>");

} else {
  form?.insertAdjacentHTML("beforeend", "<div class='w-full sm:w-4/5 mt-2 mx-auto text-su-fg-1 dark:text-su-dark-fg-1'><button id='toggle-custom' type='button' class='font-medium underline'>Custom URL</button></div>");
}

form?.insertAdjacentHTML("beforeend", `<div class="w-full sm:w-4/5 hidden max-h-80 overflow-y-auto mt-6 divide-y divide-su-bg-2 dark:divide-su-dark-bg-2 shadow-md px-2.5 bg-su-bg-2 dark:bg-black/[0.3] rounded-md border border-su-dark-bg-2 mx-auto"
                        id="url-list"></div>`);
////////////////////////////////////////////////////////////////////////////////

const toggleCustomUrl = document.getElementById("toggle-custom");
let urlList = document.getElementById("url-list");

function addUrlEntry(identifier: string) {
  // DOMPurify needs `purify.min.js` before this script gets loaded in the
  // browser.
  const cleanIdentifier = DOMPurify.sanitize(identifier);
  const div = `
  <div class="py-2 flex justify-between text-su-fg-1 dark:text-su-dark-fg-1">
    <a href="/${cleanIdentifier}">
      emojied.net/${cleanIdentifier}
    </a>

    <div class="flex space-x-2.5 text-sm">
      <!--
      <button>
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
        </svg>
      </button>
      -->

      <a href="/stats/${cleanIdentifier}">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" stroke-linejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
        </svg>
      </a>
    </div>
  </div>
  `

  urlList.insertAdjacentHTML("beforeend", div);
}

////////////////////////////////////////////////////////////////////////////////

let copyButtons = document.getElementsByClassName("copy-button");

if (copyButtons != null) {
  for (let i = 0; i < copyButtons.length; i++) {
    let el = copyButtons.item(i);

    el.classList.remove("hidden");
    el.addEventListener("click", () => {
      let html_el = el as HTMLElement;
      navigator.clipboard.writeText(html_el.dataset.shortUrl);
    })
  }
}

////////////////////////////////////////////////////////////////////////////////
// Event listeners
form?.addEventListener("submit", (e: SubmitEvent) => {
  e.preventDefault();

  let object = {};
  new FormData(document.querySelector("form")).forEach((v, k) => object[k] = v);

  Api.shortenUrl(object).then(data => {
    if (urlList.classList.contains("hidden")) {
      urlList.classList.remove("hidden");
    }

    addUrlEntry(data.identifier);
  }).catch(_e => {
    alert(`I ran into a problem!\n
Alright, listen, I barely spent any time with error handling. Two possible
scenarios:

1. Not a valid URL
2. You're not using emojis
`)
  });
});

toggleCustomUrl?.addEventListener("click", () => {
  // Toggle the identifier field
  let identifierField = document.getElementById("identifier-field");
  identifierField.classList.toggle("hidden");

  let id = document.getElementById("identifier") as HTMLInputElement;
  let urlField = document.getElementById("url") as HTMLInputElement;
  const url = new URL(window.location.href);

  if (identifierField.classList.contains("hidden")) {
    toggleCustomUrl.innerText = "Custom URL";
    url.searchParams.delete('custom_url');

    id.value = "";
    urlField.focus();
    id.removeAttribute("required");
  } else {
    toggleCustomUrl.innerText = "Autogenerate a custom URL for me";
    id.setAttribute("required", "");
    url.searchParams.set("custom_url", "t");
    id.focus();
  }

  // Set history dynamically
  history.pushState({}, "", url.toString());
});
////////////////////////////////////////////////////////////////////////////////
