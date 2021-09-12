use libil2cpp::{unsafe_impl_reference_type, Il2CppObject, ObjectExt};
use std::ops::{Deref, DerefMut};

use crate::{GlyphMetrics, GlyphRect};

#[repr(C)]
pub struct Glyph {
    object: Il2CppObject,
}

unsafe_impl_reference_type!(in libil2cpp for Glyph => UnityEngine.TextCore.Glyph);

impl Glyph {
    pub fn new(index: u32, metrics: GlyphMetrics, glyph_rect: GlyphRect) -> &'static mut Self {
        <Self as ObjectExt>::new((index, metrics, glyph_rect, 1_f32, 0_i32))
    }
}

impl Deref for Glyph {
    type Target = Il2CppObject;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl DerefMut for Glyph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}
