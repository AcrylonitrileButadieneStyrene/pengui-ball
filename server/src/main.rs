#![feature(lock_value_accessors)]

// #[cfg(any(feature = "ssr", clippy))]
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
    use figment::providers::{Env, Format, Yaml};
    use leptos::prelude::provide_context;

    let load_config = || {
        figment::Figment::new()
            .join(Env::prefixed("PENGUI_"))
            .join(Yaml::file("config.yaml"))
            .extract::<common::Config>()
    };

    let config = std::sync::Arc::new(arc_swap::ArcSwap::from_pointee(load_config().unwrap()));

    tokio::task::spawn({
        let config = config.clone();
        async move {
            let mut watcher = watch_config();
            while let Some(()) = watcher.recv().await {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                if !watcher.is_empty() {
                    continue;
                }

                match load_config() {
                    Ok(new_config) => {
                        leptos::logging::log!("Configuration updated");
                        config.store(std::sync::Arc::new(new_config))
                    }
                    Err(reason) => {
                        println!("{reason:?}");
                    }
                }
            }
        }
    });

    move || {
        provide_context(config.load_full());
    }
}

fn watch_config() -> tokio::sync::mpsc::Receiver<()> {
    use notify::Watcher as _;

    let (sender, receiver) = tokio::sync::mpsc::channel(16);

    // this is unnecessary because the watching should never stop
    // so it could be replaced with std::mem::forget
    let holder = std::sync::Arc::new(std::sync::Mutex::new(None));
    if let Ok(mut watcher) = notify::recommended_watcher({
        let holder = holder.clone();
        move |event: Result<notify::Event, notify::Error>| {
            if let Ok(event) = event
                && matches!(event.kind, notify::EventKind::Modify(_))
            {
                let Ok(()) = sender.blocking_send(()) else {
                    // drop the watcher to stop it
                    holder.set(None).unwrap();
                    return;
                };
            }
        }
    }) {
        watcher
            .watch(
                std::path::Path::new("config.yaml"),
                notify::RecursiveMode::NonRecursive,
            )
            .unwrap();
        holder.set(Some(watcher)).unwrap();
    }

    receiver
}
