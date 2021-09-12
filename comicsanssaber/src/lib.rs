#![feature(generic_associated_types)]

mod tmp;

use fontunity::TMPFontAsset;
use quest_hook::hook;

use crate::tmp::TMPText;

static DATA: &[u8] = include_bytes!("../font.ttf");
thread_local! {
    static FONT: *mut TMPFontAsset = fontunity::from_atlas(fontatlas::atlas(DATA, 64.0));
}

#[hook("TMPro", "TextMeshPro", "GenerateTextMesh")]
fn generate_text_mesh(this: &mut TMPText) {
    FONT.with(|font| {
        let font = unsafe { &mut **font };
        this.set_font(font);
    });
    generate_text_mesh.original(this);
}

#[hook("TMPro", "TextMeshProUGUI", "GenerateTextMesh")]
fn generate_text_mesh_ugui(this: &mut TMPText) {
    FONT.with(|font| {
        let font = unsafe { &mut **font };
        this.set_font(font);
    });
    generate_text_mesh_ugui.original(this);
}

#[no_mangle]
pub extern "C" fn setup() {
    quest_hook::setup("comic sans saber");
}

#[no_mangle]
pub extern "C" fn load() {
    generate_text_mesh.install().unwrap();
    generate_text_mesh_ugui.install().unwrap();
}
