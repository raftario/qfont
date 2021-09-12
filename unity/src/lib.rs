#![feature(generic_associated_types, once_cell)]

mod atlas_population_mode;
mod glyph;
mod glyph_metrics;
mod glyph_rect;
mod glyph_render_mode;
mod list;
mod material;
mod scriptable_object;
mod shader;
mod shader_utilities;
mod texture_2d;
mod texture_format;
mod tmp_fontasset;

use fontatlas::Atlas;
use libil2cpp::Il2CppArray;
use tracing::debug;

pub use crate::tmp_fontasset::TMPFontAsset;
use crate::{
    atlas_population_mode::AtlasPopulationMode, glyph::Glyph, glyph_metrics::GlyphMetrics,
    glyph_rect::GlyphRect, glyph_render_mode::GlyphRenderMode, list::List, material::Material,
    scriptable_object::ScriptableObject, shader::Shader, texture_2d::Texture2D,
    texture_format::TextureFormat,
};

#[tracing::instrument(level = "debug", skip(atlas))]
pub fn from_atlas(atlas: Atlas) -> &'static mut TMPFontAsset {
    let Atlas {
        width,
        height,
        glyphs,
        glyph_padding,
        ..
    } = atlas;

    debug!("instanciating TMP_FontAsset");
    let font_asset = ScriptableObject::create_instance::<TMPFontAsset>();
    font_asset.set_version("1.1.0");
    font_asset.set_atlas_population_mode(AtlasPopulationMode::Static);
    font_asset.set_atlas_width(width as i32);
    font_asset.set_atlas_height(height as i32);
    font_asset.set_atlas_padding(glyph_padding as i32);
    font_asset.set_atlas_render_mode(GlyphRenderMode::Smooth);

    debug!("creating {}x{} texture", width, height);
    let texture = Texture2D::new(width as i32, height as i32, TextureFormat::Alpha8);
    let texture_data = texture.raw_texture_data();

    debug!("creating glyph table");
    let glyph_table = List::<Glyph>::new(glyphs.len() as i32);

    debug!("writing atlas to texture and glyphs to table");
    for glyph in glyphs {
        for (i, a) in glyph.bitmap.iter().copied().enumerate() {
            let x = i % glyph.width;
            let y = i / glyph.width;

            let texture_idx = (x + glyph.x) + (y + glyph.y) * width;
            texture_data.as_mut_slice()[texture_idx] = a;
        }

        let index = glyph.codepoint as u32;
        let metrics = GlyphMetrics {
            width: glyph.width as f32,
            height: glyph.height as f32,
            bearing_x: glyph.bearing_x as f32,
            bearing_y: glyph.bearing_y as f32,
            advance: glyph.advance as f32,
        };
        let glyph_rect = GlyphRect {
            x: glyph.x as i32,
            y: glyph.y as i32,
            width: glyph.width as i32,
            height: glyph.height as i32,
        };

        let glyph = Glyph::new(index, metrics, glyph_rect);
        glyph_table.add(glyph);
    }
    texture.load_raw_texture_data(texture_data);
    texture.apply();

    let texture = texture as *mut Texture2D;

    font_asset.set_atlas_textures(Il2CppArray::new([Some(unsafe { &mut *texture })]));
    font_asset.set_glyph_table(glyph_table);

    debug!("creating material");
    let material = Material::new(shader_utilities::shader_ref_mobile_bitmap());
    material.set_main_texture(unsafe { &mut *texture });
    material.set_float("_TextureWidth", width as f32);
    material.set_float("_TextureHeight", height as f32);
    font_asset.set_material(material);

    debug!("initialising TMP_FontAsset");
    font_asset.read_definition();

    font_asset
}
