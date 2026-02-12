use std::{
    hash::{Hash, Hasher},
    io::{BufRead, Write},
};

use image::GenericImageView;

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

    if should_recompile {
        log::info!("Theme recompilation started. This will take a few minutes.");

        let themes: std::collections::HashMap<String, Vec<String>> =
            yaml_serde::from_str(&themes).unwrap();
        let output = std::fs::File::create("assets/css/themes.css").unwrap();
        let mut output = std::io::BufWriter::new(output);
        writeln!(output, "/*{hash}*/").unwrap();
        writeln!(output, ":root {{").unwrap();
        for (game, systems) in themes {
            for system in systems {
                writeln!(
                    output,
                    "\t--{game}-{}-gradient: {};",
                    system.replace(' ', "_"),
                    calculate_gradient(&game, &system)
                        .await
                        .as_deref()
                        .unwrap_or("0")
                )
                .unwrap();
            }
        }
        writeln!(output, "}}").unwrap();

        output.flush().unwrap();
        std::fs::copy("assets/css/themes.css", "target/site/css/themes.css").unwrap();

        log::info!("Theme compilation finished");
    }
}

#[allow(clippy::cast_precision_loss)]
async fn calculate_gradient(game: &str, system: &str) -> Option<String> {
    let request = reqwest::get(format!(
        "https://ynoproject.net/{game}/images/ui/{game}/{system}/font1.png"
    ))
    .await;

    if let Ok(response) = request
        && let Ok(bytes) = response.bytes().await
        && let Ok(image) =
            image::ImageReader::new(std::io::Cursor::new(bytes)).with_guessed_format()
        && let Ok(image) = image.decode()
    {
        assert_eq!(image.dimensions(), (16, 16));

        let steps = (0..16)
            .map(|y| image.get_pixel(0, y))
            .map(|pixel| u32::from_be_bytes(pixel.0))
            .enumerate()
            .map(|(index, color)| {
                format!(
                    "#{:06X} {:.2}% {:.2}%",
                    color >> 8,
                    // 6.25 is 1 / 16 * 100
                    index as f32 * 6.25,
                    (index + 1) as f32 * 6.25,
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        Some(format!("linear-gradient(to bottom, {steps})"))
    } else {
        None
    }
}
