set windows-shell := ["powershell.exe", "-c"]

css:
    just _css-{{os()}}
_css-windows:
    echo NOT YET IMPLEMENTED
_css-linux:
    # linux tomfoolery
    trap "kill 0" EXIT; \
    stylance -w shared --output-file target/site/css/shared.css & \
    stylance -w pages/engine --output-file target/site/css/engine.css & \
    stylance -w pages/home --output-file target/site/css/home.css & \
    stylance -w pages/play --output-file target/site/css/play.css & \
    wait

fmt:
    just _fmt-{{os()}}
_fmt-windows:
    echo NOT YET IMPLEMENTED
_fmt-linux:
    leptosfmt shared pages/*

serve:
    just _serve-{{os()}}
_serve-windows:
    New-Item -Path "./target/nginx/" -ItemType Directory -Force
    nginx -p nginx -c nginx.conf
_serve-linux:
    mkdir -p target/nginx
    nginx -p nginx -c nginx.conf
