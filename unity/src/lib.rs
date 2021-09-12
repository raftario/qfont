#![feature(generic_associated_types, once_cell)]

mod character_info;
mod font;
mod font_style;
mod material;
mod rect;
mod shader;
mod texture_2d;
mod texture_format;

use fontatlas::Atlas;
use libil2cpp::{Il2CppArray, WrapRaw};
use std::mem::{size_of, transmute};
use tracing::debug;

pub use crate::font::Font;
use crate::{
    character_info::CharacterInfo, font_style::FontStyle, material::Material, rect::Rect,
    shader::Shader, texture_2d::Texture2D, texture_format::TextureFormat,
};

#[tracing::instrument(level = "debug", skip(atlas))]
pub fn from_atlas(atlas: Atlas) -> &'static mut Font {
    let Atlas {
        width,
        height,
        glyphs,
    } = atlas;

    debug!("creating a {}x{} texture", width, height);
    let texture = Texture2D::new(width as i32, height as i32, TextureFormat::Alpha8);
    let texture_data = texture.raw_texture_data();

    let uv_x_factor = 1.0 / width as f32;
    let uv_y_factor = 1.0 / height as f32;

    let spoofed_array =
        Il2CppArray::<u8>::new(zero_iter(size_of::<CharacterInfo>() * glyphs.len()));
    let character_info = unsafe {
        let raw = spoofed_array.raw_mut();
        if !raw.bounds.is_null() {
            (*raw.bounds).length /= size_of::<CharacterInfo>();
        }
        raw.max_length /= size_of::<CharacterInfo>();

        transmute::<&mut Il2CppArray<u8>, &mut Il2CppArray<CharacterInfo>>(spoofed_array)
    };

    debug!("copying atlas to texture and extracting character info");
    for (i, glyph) in glyphs.into_iter().enumerate() {
        for (i, a) in glyph.bitmap.iter().copied().enumerate() {
            let x = i % glyph.width;
            let y = i / glyph.width;

            let texture_idx = (x + glyph.x) + (y + glyph.y) * width;
            texture_data.as_mut_slice()[texture_idx] = a;
        }

        let index = glyph.codepoint as i32;
        let uv = Rect {
            x: glyph.x as f32 * uv_x_factor,
            y: glyph.y as f32 * uv_y_factor,
            width: glyph.width as f32 * uv_x_factor,
            height: glyph.height as f32 * uv_y_factor,
        };
        let vert = Rect {
            x: 0.0,
            y: 0.0,
            width: glyph.width as f32,
            height: -(glyph.height as f32),
        };
        let width = glyph.advance as f32;

        character_info.as_mut_slice()[i] = CharacterInfo {
            index,
            uv,
            vert,
            width,
            style: FontStyle::Normal,
            flipped: false,
        };
    }
    texture.load_raw_texture_data(texture_data);
    texture.apply();

    debug!("creating material");
    let shader = Shader::find("GUI/Text Shader").unwrap();
    let material = Material::new(shader);
    material.set_main_texture(texture);

    debug!("creating font");
    let font = Font::new();
    font.set_material(material);
    font.set_character_info(character_info);

    font
}

fn zero_iter(n: usize) -> impl ExactSizeIterator<Item = u8> {
    struct Iter {
        n: usize,
        max: usize,
    }

    impl Iterator for Iter {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            if self.n >= self.max {
                return None;
            }

            self.n += 1;
            Some(0)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.max - self.n, Some(self.max - self.n))
        }
    }

    impl ExactSizeIterator for Iter {}

    Iter { n: 0, max: n }
}
