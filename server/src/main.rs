#[cfg(any(feature = "ssr", clippy))]
#[tokio::main]
async fn main() {
    use leptos_axum::LeptosRoutes;

    let conf = leptos::config::get_configuration(None).unwrap();

    axum::serve(
        tokio::net::TcpListener::bind(&conf.leptos_options.site_addr)
            .await
            .unwrap(),
        axum::Router::new()
            .leptos_routes_with_context(
                &conf.leptos_options,
                leptos_axum::generate_route_list(shared::App),
                setup_context(),
                {
                    let options = conf.leptos_options.clone();
                    move || shared::shell(options.clone())
                },
            )
            .fallback(leptos_axum::file_and_error_handler(shared::shell))
            .with_state(conf.leptos_options)
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
