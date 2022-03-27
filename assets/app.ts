const form = document.querySelector("form");

// TODO: Move this to an SSE/websocket.
form.insertAdjacentHTML("beforeend", `
<div id="identifier-field" class="hidden">
  <div class="mx-auto mt-2 text-center text-su-fg-1 dark:text-su-dark-fg-1 font-serif text-lg font-semibold">to</div>

  <div class="shadow-md dark:shadow-black/[0.2] mx-auto flex w-4/5 mt-2">
    <div class="h-full text-su-fg-1 dark:text-su-dark-fg-1 bg-gray-200 dark:bg-white/[0.1] px-2 py-2 rounded-l-md text-lg">emojied.net/</div>
    <input id="identifier" class='flex-1 text-su-fg-1 dark:text-su-dark-fg-1 rounded-r-md bg-white dark:bg-su-dark-bg-2 p-2 text-lg' type='text' name='identifier' autocomplete='off'/>
  </div>
</div>
                        `);

form.insertAdjacentHTML("beforeend", "<div class='w-4/5 mt-2 mx-auto text-su-fg-1 dark:text-su-dark-fg-1'><button id='toggle-custom' type='button' class='font-medium underline'>Custom URL</button></div>");

form.insertAdjacentHTML("beforeend", `<div class="w-4/5 hidden mt-6 divide-y divide-su-bg-2 dark:divide-su-dark-bg-2 shadow-md px-2.5 bg-su-bg-2 dark:bg-black/[0.3] rounded-md border border-su-dark-bg-2 mx-auto"
                        id="url-list"></div>`);

const toggleCustomUrl = document.getElementById("toggle-custom");
let urlList = document.getElementById("url-list");

const Api = {
  shortenUrl: async (object: Object) => {
    const opts = {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(object)
    };

    // Both `SCHEME` and `BASE_URL` have to be set by a `script` tag since
    // this isn't using Node.JS or Deno, there's no way to set environment
    // variables. Which is fine in this case since the server configuration
    // will go through the web server.
    const res = await fetch(`${SCHEME}://${BASE_URL}/rpc/shorten-url`, opts);

    if(res.ok) {
      return await res.json();
    } else {
      throw new Error(`${res.status}`);
    }
  }
};

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
      <button>
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
        </svg>
      </button>
      <button>
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" stroke-linejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
        </svg>
      </button>
    </div>
  </div>
  `

  urlList.insertAdjacentHTML("beforeend", div);
}

form.addEventListener("submit", (e: SubmitEvent) => {
  e.preventDefault();

  let object = {};
  new FormData(document.querySelector("form")).forEach((v, k) => object[k] = v);

  Api.shortenUrl(object).then(data => {
    if (urlList.classList.contains("hidden")) {
      urlList.classList.remove("hidden");
    }

    addUrlEntry(data.identifier);
  }).catch(e => console.log(e));
});

toggleCustomUrl.addEventListener("click", () => {
  // Toggle the identifier field
  let identifierField = document.getElementById("identifier-field");
  identifierField.classList.toggle("hidden");


  let id = document.getElementById("identifier") as HTMLInputElement;
  let urlField = document.getElementById("url") as HTMLInputElement;
  const url = new URL(window.location.href);

  if (identifierField.classList.contains("hidden")) {
    toggleCustomUrl.innerText = "Custom URL";
    url.searchParams.delete('custom_url');

    // Not sure if this is a good idea, tbh.
    id.value = "";
    urlField.focus();
  } else {
    toggleCustomUrl.innerText = "Autogenerate a custom URL for me";
    url.searchParams.set("custom_url", "t");
    id.focus();
  }

  // Set history dynamically
  history.pushState({}, "", url.toString());
});

