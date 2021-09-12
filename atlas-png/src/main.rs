use std::{env, fs};

use image::{GrayAlphaImage, LumaA};

fn main() {
    let mut args = env::args().skip(1);
    let input = args.next().unwrap();
    let size = args.next().unwrap().parse().unwrap();
    let output = args.next().unwrap();

    let data = fs::read(&input).unwrap();

    let atlas = fontatlas::atlas(&data, size);
    let mut image = GrayAlphaImage::new(atlas.width as u32, atlas.height as u32);

    for glyph in atlas.glyphs {
        for (i, a) in glyph.bitmap.iter().copied().enumerate() {
            let x = i % glyph.width;
            let y = i / glyph.width;

            image.put_pixel((x + glyph.x) as u32, (y + glyph.y) as u32, LumaA([0, a]));
        }
    }

    image.save(&output).unwrap();
}
