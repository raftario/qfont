use libil2cpp::Il2CppClass;

use crate::Shader;

pub fn shader_ref_mobile_bitmap() -> &'static mut Shader {
    Il2CppClass::find("TMPro", "ShaderUtilities")
        .unwrap()
        .invoke("get_ShaderRef_MobileBitmap", ())
        .unwrap()
}
