// ==UserScript==
// @name        PenguiBall Temporary Workarounds
// @version     0.1.12
// @description Temporary workarounds to make pengui-ball work before official support is added.
// @grant       GM.xmlHttpRequest
// @homepageURL https://github.com/AcrylonitrileButadieneStyrene/pengui-ball/
// @match       *://localhost:8080/*
// @match       *://127.0.0.1:8080/*
// @match       https://pengui-ball.jackssrt.com/*
// @match       https://ynoproject.net/%F0%9F%A5%BA
// @match       https://connect.ynoproject.net/%F0%9F%A5%BA
// @run-at      document-start
// @connect     connect.ynoproject.net
// @downloadURL https://raw.githubusercontent.com/AcrylonitrileButadieneStyrene/pengui-ball/master/js/pengui-ball.user.js
// @supportURL  https://github.com/AcrylonitrileButadieneStyrene/pengui-ball/issues
// ==/UserScript==

if (location.host == "ynoproject.net") {
  document.close();
  document.write(`
<script src="https://challenges.cloudflare.com/turnstile/v0/api.js"></script>
<form id="loginForm">
  <label>
    Username
    <input name="user" type="text">
  </label>
  <label>
    Password
    <input name="password" type="password">
  </label>
  <label>
    Log In
    <input type="submit"/>
  </label>
  <div class="cf-turnstile" data-sitekey="0x4AAAAAAB2ijZ45647GuniE"/>
</form>
<script>
  loginForm.onsubmit = () => {
    const body = new URLSearchParams(new FormData(loginForm)).toString();
    window.parent.postMessage(["login", body], "*");
    return false;
  }
</script>
<style>
  body {
    margin: 0;
    display: flex;
    height: 100%;
    height: -webkit-fill-available;
    height: stretch;
  }
  form {
    display: flex;
    flex-direction: column;
    color: white;
    width: fit-content;
    margin: auto;
  }
  label {
    display: flex;
    gap: 8px;
  }
  label input {
    flex: 1;
  }
</style>
  `);
  document.close();
} else if (location.host == "connect.ynoproject.net") {
  window.addEventListener("message", e => {
    if (e.data.length == 2) {
      if (e.data[0] == "set-auth")
        cookieStore.set({
          name: "auth",
          value: e.data[1],
          domain: "ynoproject.net",
          sameSite: "none",
          expires: Date.now() + 86400000,
          partitioned: true,
        });
    } else if (e.data.length == 3) {
      if (e.data[0].startsWith("/api/"))
        e.data[0] = e.data[0].replace("/api/", "/");
      if (e.data[0] == "/seiko/logout")
        document.cookie = "auth=; Max-Age=0; Path=/; SameSite=none; Secure";
      fetch(e.data[0], e.data[1])
        .then(async resp => [resp.status, resp.statusText, await resp.arrayBuffer()])
        .then(resp => window.parent.postMessage([e.data[2], "resolve", resp], "*"))
        .catch(err => window.parent.postMessage([e.data[2], "reject", err.toString()], "*"));
    }
  });
} else if (window.self == window.top) {
  let queue = [];
  const ongoing = {};

  // please give me a CORS excemption so i don't have to do this
  const iframe = document.createElement("iframe");
  iframe.src = "https://connect.ynoproject.net/%F0%9F%A5%BA";
  iframe.style.display = "none";
  iframe.onload = () => {
    let items = queue;
    queue = undefined;
    for (const item of items)
      iframe.contentWindow.postMessage(item, "*");
  };
  window.addEventListener("load", () => document.body.appendChild(iframe));

  let originalFetch = unsafeWindow.fetch;
  unsafeWindow.fetch = function (url, options) {
    url = new URL(url?.url || url, location.href).href;
    if (!url.includes("/api/"))
      return originalFetch.apply(this, arguments);
    else return new Promise((resolve, reject) => {
      const key = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);
      ongoing[key] = [resolve, reject];

      let opts = [url.replace(location.origin, ""), options, key];
      if (queue) queue.push(opts)
      else iframe.contentWindow.postMessage(opts, "*");
    });
  };

  window.addEventListener("message", e => {
    if (e.data?.length == 2) {
      GM.xmlHttpRequest({
        method: "POST",
        url: "https://connect.ynoproject.net/seiko/login",
        data: e.data[1],
        headers: {
          "content-type": "application/x-www-form-urlencoded",
        },
        anonymous: true,
        onload: response => {
          if (response.status != 200)
            return alert(response.responseText);
          const auth = response.responseHeaders.split("auth=")[1].split(";")[0];
          iframe.contentWindow.postMessage(["set-auth", auth], "*");
          onAuthCookieSet();
        },
      });
    } else if (e.data?.length == 3) {
      let [resolve, reject] = ongoing[e.data[0]];
      if (e.data[1] == "resolve")
        resolve(new Response(e.data[2][2], {
          status: e.data[2][0],
          statusText: e.data[2][1],
        }));
      else reject(new Error(e.data[2]));
    }
  });
}

