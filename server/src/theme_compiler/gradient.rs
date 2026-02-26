use std::sync::Arc;

use image::GenericImageView;

#[allow(clippy::cast_precision_loss)]
pub async fn calculate(game: Arc<str>, system: Arc<str>) -> Option<String> {
    let image = super::get_image(&format!(
        "https://ynoproject.net/{game}/images/ui/{game}/{system}/font1.png"
    ))
    .await;

    image.map(|image| {
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
        format!("linear-gradient(to bottom, {steps})")
    })
}
