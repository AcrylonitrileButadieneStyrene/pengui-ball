css:
    # linux tomfoolery
    trap "kill 0" EXIT; \
    stylance -w shared --output-file target/site/css/shared.css & \
    stylance -w pages/engine --output-file target/site/css/engine.css & \
    stylance -w pages/home --output-file target/site/css/home.css & \
    stylance -w pages/play --output-file target/site/css/play.css & \
    wait
fmt:
    leptosfmt shared pages/*
serve:
    nginx -p nginx -c nginx.conf
