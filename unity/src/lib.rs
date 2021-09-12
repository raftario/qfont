#![feature(generic_associated_types, once_cell)]

mod character_info;
mod font;
mod font_style;
mod material;
mod native_array;
mod rect;
mod shader;
mod texture_2d;
mod texture_format;

use fontatlas::Atlas;
use libil2cpp::Il2CppArray;

use crate::{
    character_info::CharacterInfo, font::Font, font_style::FontStyle, material::Material,
    native_array::NativeArray, rect::Rect, shader::Shader, texture_2d::Texture2D,
    texture_format::TextureFormat,
};

pub fn from_atlas(atlas: Atlas) -> &'static mut Font {
    let Atlas {
        width,
        height,
        glyphs,
    } = atlas;

    let texture = Texture2D::new(
        atlas.width as i32,
        atlas.height as i32,
        TextureFormat::Alpha8,
    );
    let mut texture_data = texture.raw_texture_data();
    let texture_data = unsafe { texture_data.as_mut_slice() };

    let uv_x_factor = 1.0 / width as f32;
    let uv_y_factor = 1.0 / height as f32;

    let character_info = glyphs.into_iter().map(|glyph| {
        for (i, a) in glyph.bitmap.iter().copied().enumerate() {
            let x = i % glyph.width;
            let y = i / glyph.width;

            let texture_idx = (x + glyph.x) + (y + glyph.y) * width;
            texture_data[texture_idx] = a;
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

        CharacterInfo {
            index,
            uv,
            vert,
            width,
            style: FontStyle::Normal,
            flipped: false,
        }
    });

    texture.apply();
    let character_info = Il2CppArray::<CharacterInfo>::new(character_info);

    let shader = Shader::find("GUI/Text Shader").unwrap();
    let material = Material::new(shader);
    material.set_main_texture(texture);

    let font = Font::new();
    font.set_material(material);
    font.set_character_info(character_info);

    font
}
