use libil2cpp::{unsafe_impl_reference_type, Il2CppArray, Il2CppObject, Il2CppString};
use std::ops::{Deref, DerefMut};

use crate::{AtlasPopulationMode, Glyph, GlyphRenderMode, List, Material, Texture2D};

pub struct TMPFontAsset {
    object: Il2CppObject,
}

unsafe_impl_reference_type!(in libil2cpp for TMPFontAsset => TMPro.TMP_FontAsset);

impl TMPFontAsset {
    pub(crate) fn set_version(&mut self, version: &str) {
        self.store("m_Version", Il2CppString::new(version));
    }

    pub(crate) fn set_atlas_population_mode(&mut self, mode: AtlasPopulationMode) {
        self.invoke("set_atlasPopulationMode", mode).unwrap()
    }

    pub(crate) fn set_atlas_width(&mut self, width: i32) {
        self.invoke("set_atlasWidth", width).unwrap()
    }

    pub(crate) fn set_atlas_height(&mut self, height: i32) {
        self.invoke("set_atlasHeight", height).unwrap()
    }

    pub(crate) fn set_atlas_padding(&mut self, padding: i32) {
        self.invoke("set_atlasPadding", padding).unwrap()
    }

    pub(crate) fn set_atlas_render_mode(&mut self, mode: GlyphRenderMode) {
        self.invoke("set_atlasRenderMode", mode).unwrap()
    }

    pub(crate) fn set_glyph_table(&mut self, table: &mut List<Glyph>) {
        self.invoke("set_glyphTable", table).unwrap()
    }

    pub(crate) fn set_atlas_textures(&mut self, textures: &mut Il2CppArray<Texture2D>) {
        self.invoke("set_atlasTextures", textures).unwrap()
    }

    pub(crate) fn set_material(&mut self, material: &mut Material) {
        self.store("material", material)
    }

    pub(crate) fn read_definition(&mut self) {
        self.invoke("ReadFontAssetDefinition", ()).unwrap()
    }
}

impl Deref for TMPFontAsset {
    type Target = Il2CppObject;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl DerefMut for TMPFontAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}
