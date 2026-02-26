use std::sync::Arc;

use image::GenericImageView;

#[allow(clippy::cast_precision_loss)]
pub async fn calculate(game: Arc<str>, system: Arc<str>) -> Option<String> {
    let image = super::get_image(&format!(
        "https://ynoproject.net/{game}/images/ui/{game}/{system}/fontshadow.png"
    ))
    .await;

    image.map(|image| {
        let color = u32::from_be_bytes(image.get_pixel(0, 8).0);
        format!("#{:06X}", color >> 8)
    })
}
