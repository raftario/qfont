use libil2cpp::unsafe_impl_value_type;

#[repr(i32)]
#[derive(Debug)]
pub enum TextureFormat {
    Alpha8 = 1,
}

unsafe_impl_value_type!(in libil2cpp for TextureFormat => UnityEngine.TextureFormat);
