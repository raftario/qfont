use libil2cpp::unsafe_impl_value_type;

use crate::{FontStyle, Rect};

#[repr(C)]
#[derive(Debug)]
pub struct CharacterInfo {
    pub index: i32,
    pub uv: Rect,
    pub vert: Rect,
    pub width: f32,
    pub style: FontStyle,
    pub flipped: bool,
}

unsafe_impl_value_type!(in libil2cpp for CharacterInfo => UnityEngine.CharacterInfo);
