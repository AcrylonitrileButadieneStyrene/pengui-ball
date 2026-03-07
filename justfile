set windows-shell := ["powershell.exe", "-c"]

[linux]
css:
    stylance -w pages/play --output-file target/site/css/play.css

[linux]
fmt:
    leptosfmt shared pages/*

serve:
    nginx -p nginx -c nginx.conf
