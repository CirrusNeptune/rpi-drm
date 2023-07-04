pub mod test_triangle;

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
                .set(crate::get_card().vc4_create_shader_bo(&self.code).unwrap())
                .unwrap()
        })
    }
}

pub async fn initialize_shaders() {
    let _ = tokio::join!(
        test_triangle::VS_ASM.initialize(),
        test_triangle::CS_ASM.initialize(),
        test_triangle::FS_ASM_TEX.initialize(),
    );
}
