use anyhow::Result;
use std::path::Path;
use tray_icon::Icon;

pub fn load_icon<P: AsRef<Path>>(path: P) -> Result<Icon> {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)?.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)?;
    Ok(icon)
}
