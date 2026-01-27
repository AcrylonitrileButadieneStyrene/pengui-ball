// ==UserScript==
// @name        PenguiBall Temporary Workarounds
// @match       *://localhost:8080/*
// @version     0.1.0
// @description Temporary workarounds to make pengui-ball work before official support is added.
// @noframes
// @grant       GM.xmlHttpRequest
// @downloadURL https://raw.githubusercontent.com/AcrylonitrileButadieneStyrene/pengui-ball/master/js/pengui-ball.user.js
// @supportURL  https://github.com/AcrylonitrileButadieneStyrene/pengui-ball/issues
// @homepageURL https://github.com/AcrylonitrileButadieneStyrene/pengui-ball/
// @run-at      document-start
// ==/UserScript==

let originalFetch = unsafeWindow.fetch;
unsafeWindow.fetch = function (url) {
    url = url?.url || url;
    if (!url.includes("/api/"))
        return originalFetch.apply(this, arguments);
    else return new Promise((resolve, reject) => {
        GM.xmlHttpRequest({
            url, responseType: "arraybuffer",
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
}
