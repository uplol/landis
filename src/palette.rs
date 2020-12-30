use std::{fs::File, io::Read, path::PathBuf, sync::Arc};

use fastanvil::{Palette, RenderedPalette};
use flate2::read::GzDecoder;

/// Wraps a RenderedPalette, with helpful load utility.
pub struct BlockPalette(Arc<RenderedPalette>);

/// A palette for rendering.
/// Right now, loading happens via a file packed with the "fastanvil" tool. We can probably make this better, faster.
impl BlockPalette {
    /// Loads a block palette from a "fastanvil"-packed palette tarball.
    /// TODO: Error handling! Also, tokio compatibility? This only happens on startup though so.. less concerning.
    pub async fn load(path: PathBuf) -> Self {
        let file = GzDecoder::new(File::open(path).unwrap());
        let mut archive = tar::Archive::new(file);

        let mut grass = Err("no grass colour map");
        let mut foliage = Err("no foliage colour map");
        let mut blockstates = Err("no blockstate palette");

        for archive_file in archive.entries().unwrap() {
            let mut archive_file = archive_file.unwrap();
            match archive_file.path().unwrap().to_str().unwrap() {
                "grass-colourmap.png" => {
                    let mut buf = vec![];
                    archive_file.read_to_end(&mut buf).unwrap();

                    grass = Ok(
                        image::load(std::io::Cursor::new(buf), image::ImageFormat::Png)
                            .unwrap()
                            .into_rgba8(),
                    );
                }
                "foliage-colourmap.png" => {
                    let mut buf = vec![];
                    archive_file.read_to_end(&mut buf).unwrap();

                    foliage = Ok(
                        image::load(std::io::Cursor::new(buf), image::ImageFormat::Png)
                            .unwrap()
                            .into_rgba8(),
                    );
                }
                "blockstates.json" => {
                    let json: std::collections::HashMap<String, fastanvil::Rgba> =
                        serde_json::from_reader(archive_file).unwrap();
                    blockstates = Ok(json);
                }
                _ => {}
            }
        }

        Self(Arc::new(RenderedPalette {
            blockstates: blockstates.unwrap(),
            grass: grass.unwrap(),
            foliage: foliage.unwrap(),
        }))
    }
}

// We have to impl the foreign Palette trait.
impl Palette for BlockPalette {
    fn pick(
        &self,
        block: &fastanvil::Block,
        biome: Option<fastanvil::biome::Biome>,
    ) -> fastanvil::Rgba {
        self.0.pick(block, biome)
    }
}

// If we want to clone it we have to do it manually.
impl Clone for BlockPalette {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
