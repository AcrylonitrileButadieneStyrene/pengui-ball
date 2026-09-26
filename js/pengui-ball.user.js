// ==UserScript==
// @name        PenguiBall Temporary Workarounds
// @version     0.1.18
// @description Temporary workarounds to make pengui-ball work before official support is added.
// @grant       GM.xmlHttpRequest
// @homepageURL https://github.com/AcrylonitrileButadieneStyrene/pengui-ball/
// @match       *://localhost:8080/*
// @match       *://127.0.0.1:8080/*
// @match       https://pengui-ball.jackssrt.com/*
// @match       https://ynoproject.net/%F0%9F%A5%BA
// @match       https://api.ynoproject.net/%F0%9F%A5%BA
// @run-at      document-start
// @connect     api.ynoproject.net
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
  <div style="display:flex;">
    <input type="submit" value="Register"/>
    <input type="submit" value="Login" style="flex:1;"/>
  </div>
  <div class="cf-turnstile" data-sitekey="0x4AAAAAAC_-ONgXPc49t7sd"/>
</form>
<script>
  loginForm.onsubmit = event => {
    const body = new URLSearchParams(new FormData(loginForm)).toString();
    window.parent.postMessage([event.submitter.value.toLowerCase(), body], "*");
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
} else if (location.host == "api.ynoproject.net") {
  window.addEventListener("message", e => {
    cookieStore.set({
      name: "auth",
      value: e.data,
      domain: "ynoproject.net",
      sameSite: "none",
      expires: Date.now() + 86400000,
      partitioned: true,
    });
  });
} else if (window.self == window.top) {
  const iframe = document.createElement("iframe");
  iframe.src = "https://api.ynoproject.net/%F0%9F%A5%BA";
  iframe.style.display = "none";

  window.addEventListener("load", () => document.body.appendChild(iframe));
  window.addEventListener("message", e => {
    if (e.data?.length == 2) {
      GM.xmlHttpRequest({
        method: "POST",
        url: "https://auth.ynoproject.net/" + e.data[0],
        data: e.data[1],
        headers: {
          "content-type": "application/x-www-form-urlencoded",
        },
        anonymous: true,
        onload: response => {
          if (response.status != 200)
            return alert(response.responseText);
          if (e.data[0] == "register")
            return alert("Account created successfully.");
          const auth = response.responseHeaders.split("auth=")[1].split(";")[0];
          iframe.contentWindow.postMessage(auth, "*");
          setTimeout(() => onAuthCookieSet(), 100);
        },
      });
    }
  });
}
