pub fn load() -> anyhow::Result<common::ServerConfiguration> {
    let mut config: common::ServerConfiguration =
        yaml_serde::from_reader(std::fs::File::open("config/config.yaml")?)?;
    config.menu_themes = yaml_serde::from_reader(std::fs::File::open("config/menu_themes.yaml")?)?;

    Ok(config)
}

pub fn watch(
    config: std::sync::Arc<arc_swap::ArcSwapAny<std::sync::Arc<common::ServerConfiguration>>>,
) {
    tokio::task::spawn(async move {
        let mut watcher = watcher();
        while watcher.recv().await == Some(()) {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            if !watcher.is_empty() {
                continue;
            }

            match load() {
                Ok(new_config) => {
                    leptos::logging::log!("Configuration updated");
                    config.store(std::sync::Arc::new(new_config));
                }
                Err(reason) => {
                    println!("{reason:?}");
                }
            }
        }
    });
}

fn watcher() -> tokio::sync::mpsc::Receiver<()> {
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
                std::path::Path::new("config"),
                notify::RecursiveMode::Recursive,
            )
            .unwrap();
        holder.set(Some(watcher)).unwrap();
    }

    receiver
}
