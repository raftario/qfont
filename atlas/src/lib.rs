use fontdue::{Font, FontSettings};
use std::{cmp, collections::BTreeSet};
use tracing::debug;
use ttf_parser::Face;

#[derive(Debug)]
pub struct Atlas {
    pub width: usize,
    pub height: usize,
    pub glyphs: Vec<Glyph>,
}

#[derive(Debug)]
pub struct Glyph {
    pub codepoint: char,

    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,

    pub advance: usize,
    pub bearing: i32,

    pub bitmap: Vec<u8>,
}

struct GlyphAndOffset(Glyph, i32);

impl PartialEq for GlyphAndOffset {
    fn eq(&self, other: &Self) -> bool {
        self.0.codepoint.eq(&other.0.codepoint)
    }
}
impl Eq for GlyphAndOffset where char: Eq {}

impl PartialOrd for GlyphAndOffset {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.codepoint.partial_cmp(&other.0.codepoint)
    }
}
impl Ord for GlyphAndOffset {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.codepoint.cmp(&other.0.codepoint)
    }
}

#[tracing::instrument(level = "debug", skip(data))]
pub fn atlas(data: &[u8], size: f32) -> Atlas {
    let face = Face::from_slice(data, 0).unwrap();
    let font = Font::from_bytes(
        data,
        FontSettings {
            scale: size,
            ..Default::default()
        },
    )
    .unwrap();

    debug!("calculating metrics");

    let horizontal_metrics = font.horizontal_line_metrics(size).unwrap();
    let estimated_ascent = horizontal_metrics.ascent.ceil() as i32;
    let estimated_height = estimated_ascent - horizontal_metrics.descent.floor() as i32;

    let mut glyph_set = BTreeSet::new();
    let mut total_width = 0;

    let mut ascent = estimated_ascent;
    let mut y_bump = 0;
    let mut yoffset_bump = 0;

    debug!("rasterizing glyphs");
    for subtable in face.character_mapping_subtables() {
        subtable.codepoints(|c| {
            if subtable.glyph_index(c).is_none() {
                return;
            }
            let codepoint = match char::from_u32(c) {
                Some(c) => c,
                None => return,
            };

            let (metrics, bitmap) = font.rasterize(codepoint, size);

            let yoffset = ascent as i32 - metrics.ymin - metrics.height as i32;
            if yoffset < 0 {
                let new_ascent = estimated_ascent - yoffset;
                if new_ascent > ascent {
                    yoffset_bump += new_ascent - ascent;
                    ascent = new_ascent;
                }
            }

            if metrics.height as i32 + yoffset > estimated_height {
                y_bump = y_bump.max(metrics.height as i32 + yoffset - estimated_height);
            };

            glyph_set.insert(GlyphAndOffset(
                Glyph {
                    codepoint,
                    x: 0,
                    y: 0,
                    width: metrics.width,
                    height: estimated_height as usize,
                    advance: metrics.advance_width.ceil() as usize,
                    bearing: metrics.xmin,
                    bitmap,
                },
                yoffset,
            ));
            total_width += metrics.width;
        });
    }

    let height = (estimated_height - estimated_ascent + ascent + y_bump) as usize;

    let atlas_area = total_width * height;
    let atlas_width = (atlas_area as f64).sqrt().ceil() as usize;

    let mut glyphs = Vec::new();

    let mut x = 0;
    let mut y = 0;

    debug!("positioning glyphs");
    for GlyphAndOffset(mut glyph, yoffset) in glyph_set {
        if x + glyph.width > atlas_width {
            x = 0;
            y += height;
        }

        glyph.x = x;
        glyph.y = (y as i32 + yoffset + yoffset_bump) as usize;
        glyph.height = height;

        x += glyph.width;
        glyphs.push(glyph);
    }

    debug!("generated a font atlas with {} glyphs", glyphs.len());
    Atlas {
        width: atlas_width,
        height: y + height,
        glyphs,
    }
}
