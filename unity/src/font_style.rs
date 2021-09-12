use libil2cpp::unsafe_impl_value_type;

#[repr(i32)]
#[derive(Debug)]
pub enum FontStyle {
    Normal = 0,
    Bold = 1,
    Italic = 2,
    BoldAndItalic = 3,
}

unsafe_impl_value_type!(in libil2cpp for FontStyle => UnityEngine.FontStyle);
