use std::{
    io::Cursor,
    path::{Path, PathBuf},
};

use tokio::{fs::File, io::AsyncReadExt};

use crate::palette::BlockPalette;

pub async fn render_region(palette: BlockPalette, path: PathBuf) {
    let buffer = {
        // Quickly read it all into a vec so we can work with it as we please.
        let mut file = File::open(&path).await.unwrap();
        let mut buffer = Vec::with_capacity(8388608 /* 8MB */);
        file.read_to_end(&mut buffer).await.unwrap();
        buffer
    };

    let (x, z) = coords_from_region(path.as_path()).unwrap();

    let region = fastanvil::Region::new(Cursor::new(buffer));

    let map = fastanvil::RegionMap::new(x, z, [0, 0, 0, 0]);
    let mut drawer = fastanvil::RegionBlockDrawer::new(map, &palette);
    fastanvil::parse_region(region, &mut drawer).unwrap_or_default();

    let rendered_map = fastanvil::IntoMap::into_map(drawer);

    let mut img = image::ImageBuffer::new(512, 512);

    for xc in 0..32 {
        for zc in 0..32 {
            let chunk = rendered_map.chunk(xc, zc);
            for z in 0..16 {
                for x in 0..16 {
                    let pixel = chunk[z * 16 + x];
                    let x = xc * 16 + x as usize;
                    let z = zc * 16 + z as usize;
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
