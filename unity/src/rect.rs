use libil2cpp::unsafe_impl_value_type;

#[repr(C)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

unsafe_impl_value_type!(in libil2cpp for Rect => UnityEngine.Rect);
