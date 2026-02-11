#![feature(lock_value_accessors)]

mod config;

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
    use leptos::prelude::provide_context;

    let config = std::sync::Arc::new(arc_swap::ArcSwap::from_pointee(
        config::load().expect("The configuration file was invalid, so the server cannot start."),
    ));

    config::watch(config.clone());

    // runs on each request, keep as light as possible
    move || {
        provide_context(config.load_full());
    }
}
