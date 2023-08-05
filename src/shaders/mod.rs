#![allow(unused_imports, nonstandard_style)]
pub mod generated;
pub mod test_model;

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

pub async fn initialize_shaders() {
    generated::initialize_shaders().await;
}
