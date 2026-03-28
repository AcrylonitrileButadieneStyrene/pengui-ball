set windows-shell := ["powershell.exe", "-c"]

css:
    stylance -w pages/play --output-file target/site/css/play.css

[linux]
fmt:
    leptosfmt shared pages/*

[linux]
serve:
    trap 'kill 0' EXIT; nginx -p nginx -c nginx.conf

[windows]
serve:
    nginx -p nginx -c nginx.conf
