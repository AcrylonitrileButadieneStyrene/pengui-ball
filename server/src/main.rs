#[cfg(any(feature = "ssr", clippy))]
#[tokio::main]
async fn main() {
    use leptos_axum::LeptosRoutes as _;

    let conf = leptos::config::get_configuration(None)
        .unwrap()
        .leptos_options;

    axum::serve(
        tokio::net::TcpListener::bind(&conf.site_addr)
            .await
            .unwrap(),
        axum::Router::new()
            .route_service(
                "/favicon.ico",
                tower_http::services::ServeFile::new(
                    std::path::Path::new(&*conf.site_root).join("favicon.ico"),
                ),
            )
            .route_service(
                "/robots.txt",
                tower_http::services::ServeFile::new(
                    std::path::Path::new(&*conf.site_root).join("robots.txt"),
                ),
            )
            .leptos_routes_with_context(
                &conf,
                leptos_axum::generate_route_list(shared::App),
                setup_context(),
                {
                    let conf = conf.clone();
                    move || shared::shell(conf.clone())
                },
            )
            .fallback(leptos_axum::file_and_error_handler(shared::shell))
            .with_state(conf)
            .into_make_service(),
    )
    .await
    .unwrap();
}

fn setup_context() -> impl Fn() + Clone {
    use figment::providers::{Env, Format, Yaml};
    use leptos::prelude::provide_context;

    let config = std::sync::Arc::new(
        figment::Figment::new()
            .join(Env::prefixed("PENGUI_"))
            .join(Yaml::file("config.yaml"))
            .extract::<shared::Config>()
            .unwrap(),
    );

    move || {
        provide_context(config.clone());
    }
}
