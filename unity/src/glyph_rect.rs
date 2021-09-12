use libil2cpp::unsafe_impl_value_type;

#[repr(C)]
#[derive(Debug)]
pub struct GlyphRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

unsafe_impl_value_type!(in libil2cpp for GlyphRect => UnityEngine.TextCore.GlyphRect);
