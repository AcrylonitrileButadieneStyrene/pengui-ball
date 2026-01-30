// ==UserScript==
// @name        PenguiBall Temporary Workarounds
// @match       *://localhost:8080/*
// @match       *://127.0.0.1:8080/*
// @match       *://ynoproject.net/%F0%9F%A5%BA
// @version     0.1.2
// @description Temporary workarounds to make pengui-ball work before official support is added.
// @grant       GM.xmlHttpRequest
// @downloadURL https://raw.githubusercontent.com/AcrylonitrileButadieneStyrene/pengui-ball/master/js/pengui-ball.user.js
// @supportURL  https://github.com/AcrylonitrileButadieneStyrene/pengui-ball/issues
// @homepageURL https://github.com/AcrylonitrileButadieneStyrene/pengui-ball/
// @run-at      document-start
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
    fetch("https://connect.ynoproject.net/seiko/login", {
      method: 'POST',
      body: new URLSearchParams(new FormData(loginForm)),
      credentials: "include",
    }).then(() => {
      window.parent.postMessage("auth cookie was set", "*");
    });
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
} else if (window.self == window.top) {
  let originalFetch = unsafeWindow.fetch;
  unsafeWindow.fetch = function (url) {
    url = url?.url || url;
    if (!url.includes("/api/"))
      return originalFetch.apply(this, arguments);
    else return new Promise((resolve, reject) => {
      GM.xmlHttpRequest({
        url: url.replace(location.origin, "https://connect.ynoproject.net/"),
        responseType: "arraybuffer",
        onload: resp => resolve(new Response(resp.response, {
          status: resp.status,
          statusText: resp.statusText,
          headers: new Headers(
            resp.responseHeaders.split("\r\n").map(line => {
              const [hd, ...tl] = line.split(":");
              return [hd, tl.join(":")];
            }).filter(x => x[0].length)
          ),
        })),
        onerror: reject,
      });
    });
  };

  window.addEventListener("message", e => {
    if (e.data != "auth cookie was set")
      return;

    onAuthCookieSet();
  });
}
