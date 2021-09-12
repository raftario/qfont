use libil2cpp::unsafe_impl_value_type;

#[repr(i32)]
#[derive(Debug)]
pub enum AtlasPopulationMode {
    Static = 0,
}

unsafe_impl_value_type!(in libil2cpp for AtlasPopulationMode => TMPro.AtlasPopulationMode);
