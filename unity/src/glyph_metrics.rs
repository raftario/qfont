use libil2cpp::unsafe_impl_value_type;

#[repr(C)]
#[derive(Debug)]
pub struct GlyphMetrics {
    pub width: f32,
    pub height: f32,
    pub bearing_x: f32,
    pub bearing_y: f32,
    pub advance: f32,
}

unsafe_impl_value_type!(in libil2cpp for GlyphMetrics => UnityEngine.TextCore.GlyphMetrics);
