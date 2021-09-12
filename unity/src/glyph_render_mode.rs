use libil2cpp::unsafe_impl_value_type;

#[repr(i32)]
#[derive(Debug)]
pub enum GlyphRenderMode {
    Smooth = 4117,
}

unsafe_impl_value_type!(in libil2cpp for GlyphRenderMode => UnityEngine.TextCore.LowLevel.GlyphRenderMode);
