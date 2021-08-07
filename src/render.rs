use fastanvil::{
    render_region, CCoord, Dimension, HeightMode, JavaChunk, RCoord, RegionFileLoader,
    TopShadeRenderer,
};
use std::path::{Path, PathBuf};

use crate::palette::BlockPalette;

pub async fn render_map(palette: BlockPalette, path: PathBuf) {
    let loader =
        RegionFileLoader::<JavaChunk>::new(Path::new("mcserver/world/region/").to_path_buf());
    let dim = Dimension::new(Box::new(loader));
    let (x, z) = coords_from_region(path.clone().as_path()).unwrap();
    let drawer = TopShadeRenderer::new(&palette, HeightMode::Calculate);
    let map = render_region(RCoord(x), RCoord(z), dim, drawer);
    let mut img = image::ImageBuffer::new(512, 512);

    for xc in 0..32 {
        for zc in 0..32 {
            let chunk = map.chunk(CCoord(xc), CCoord(zc));
            for z in 0..16 {
                for x in 0..16 {
                    let pixel = chunk[z * 16 + x];
                    let x = xc * 16 + x as isize;
                    let z = zc * 16 + z as isize;
                    img.put_pixel(x as u32, z as u32, image::Rgba(pixel))
                }
            }
        }
    }
    println!("./out/{}.{}.png", x, z);
    img.save_with_format(format!("./out/{}.{}.png", x, z), image::ImageFormat::Png)
        .unwrap();
}

fn coords_from_region(region: &Path) -> Option<(isize, isize)> {
    let filename = region.file_name()?.to_str()?;
    let mut parts = filename.split('.').skip(1);
    Some((parts.next()?.parse().ok()?, parts.next()?.parse().ok()?))
}
