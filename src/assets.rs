pub mod pic {
    use bevy::render::texture::{CompressedImageFormats, Image, ImageSampler, ImageType};
    use once_cell::sync::Lazy;

    pub static CLICK_AFTER: Lazy<Image> = Lazy::new(|| {
        Image::from_buffer(
            include_bytes!("../assets/pic/ClickAfter.png"),
            ImageType::Extension("png"),
            CompressedImageFormats::default(),
            true,
            ImageSampler::default(),
        )
        .unwrap()
    });
    pub static CLICK_BEFORE: Lazy<Image> = Lazy::new(|| {
        Image::from_buffer(
            include_bytes!("../assets/pic/ClickBefore.png"),
            ImageType::Extension("png"),
            CompressedImageFormats::default(),
            true,
            ImageSampler::default(),
        )
        .unwrap()
    });
}
pub mod font {
    use bevy::text::Font;
    use once_cell::sync::Lazy;

    pub static MAIN_FONT: Lazy<Font> = Lazy::new(|| {
        Font::try_from_bytes(Vec::from(include_bytes!(
            "../assets/fonts/tiny_font/Alibaba-PuHuiTi-Heavy.ttf"
        )))
        .unwrap()
    });
}
