css:
    # linux tomfoolery
    trap "kill 0" EXIT; \
    stylance -w shared --output-file target/site/pkg/shared.css & \
    stylance -w pages/engine --output-file target/site/pkg/engine.css & \
    stylance -w pages/home --output-file target/site/pkg/home.css & \
    stylance -w pages/play --output-file target/site/pkg/play.css & \
    wait
fmt:
    leptosfmt shared pages/*
serve:
    nginx -p nginx -c nginx.conf
