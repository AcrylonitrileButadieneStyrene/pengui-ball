use std::{
    hash::{Hash, Hasher},
    io::{BufRead, Write},
    sync::Arc,
};

use futures::StreamExt as _;

mod gradient;
mod shadow;

pub async fn run() {
    let themes = std::fs::read_to_string("config/themes.yaml").expect("Themes list not provided");
    let mut hash = std::hash::DefaultHasher::new();
    themes.hash(&mut hash);
    let hash = hash.finish();

    let should_recompile = std::fs::File::open("assets/css/themes.css").map_or(true, |file| {
        let mut reader = std::io::BufReader::new(file);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let fragment = &line[2..(line.len() - 3)];
        let is_eq = fragment
            .parse::<u64>()
            .is_ok_and(|fragment| fragment == hash);
        !is_eq
    });

    if !should_recompile {
        return;
    }

    log::info!("Theme recompilation started. This will take a few minutes.");

    let themes: std::collections::HashMap<Arc<str>, Vec<String>> =
        yaml_serde::from_str(&themes).unwrap();
    std::fs::create_dir_all("assets/css").unwrap();
    let output = std::fs::File::create("assets/css/themes.css").unwrap();
    let mut output = std::io::BufWriter::new(output);
    writeln!(output, "/*{hash}*/").unwrap();
    writeln!(output, ":root {{").unwrap();

    futures::stream::iter(themes)
        .flat_map(|(game, systems)| {
            futures::stream::iter(
                systems
                    .into_iter()
                    .map(move |system| (game.clone(), Arc::<str>::from(system.replace(' ', "_")))),
            )
        })
        .map(|(game, system)| async move {
            let (gradient, shadow) = tokio::join!(
                gradient::calculate(game.clone(), system.clone()),
                shadow::calculate(game.clone(), system.clone()),
            );

            [("gradient", gradient), ("shadow", shadow)]
                .into_iter()
                .filter_map(|(name, value)| {
                    value.map(|value| (game.clone(), system.clone(), name.to_string(), value))
                })
                .collect::<Vec<_>>()
        })
        .buffered(64)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .flatten()
        .for_each(|(game, system, name, value)| {
            writeln!(output, "\t--{game}-{system}-{name}: {value};").unwrap();
        });

    writeln!(output, "}}").unwrap();

    output.flush().unwrap();
    std::fs::copy("assets/css/themes.css", "public/css/themes.css").unwrap();

    log::info!("Theme compilation finished");
}

async fn get_image(url: &str) -> Option<image::DynamicImage> {
    let request = reqwest::get(url).await;
    if let Ok(response) = request
        && let Ok(bytes) = response.bytes().await
        && let Ok(image) =
            image::ImageReader::new(std::io::Cursor::new(bytes)).with_guessed_format()
        && let Ok(image) = image.decode()
    {
        Some(image)
    } else {
        None
    }
}
