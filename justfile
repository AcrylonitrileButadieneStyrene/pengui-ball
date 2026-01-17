# css:
#     stylance -w shared --output-file target/site/pkg/pengui-ball.css
fmt:
    leptosfmt shared pages/*
serve:
    nginx -p nginx -c nginx.conf
