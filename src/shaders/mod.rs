pub mod test_gen;
pub mod test_model;
pub mod test_triangle;
pub mod test_triangle2;
pub mod test_triangle3;

use std::sync::OnceLock;
use vc4_drm::drm::buffer::Handle;
use vc4_drm::tokio;

pub struct ShaderNode {
    code: &'static [u64],
    handle: OnceLock<Handle>,
}

impl ShaderNode {
    pub const fn new(code: &'static [u64]) -> Self {
        Self {
            code,
            handle: OnceLock::new(),
        }
    }

    pub fn initialize(&'static self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async {
            self.handle
                .set(
                    rpi_drm::get_card()
                        .vc4_create_shader_bo(&self.code)
                        .unwrap(),
                )
                .unwrap()
        })
    }
}

macro_rules! initialize_modules {
    ($($ids:ident),*) => {
        tokio::join!(
            $(
                $ids::VS_ASM.initialize(),
                $ids::CS_ASM.initialize(),
                $ids::FS_ASM.initialize(),
            )*
        )
    }
}

pub async fn initialize_shaders() {
    let _ = tokio::join!(
        test_triangle::VS_ASM.initialize(),
        test_triangle::CS_ASM.initialize(),
        test_triangle::FS_ASM.initialize(),
        test_triangle::FS_ASM_TEX.initialize(),
    );
    let _ = initialize_modules!(test_model, test_gen, test_triangle2, test_triangle3);
}
