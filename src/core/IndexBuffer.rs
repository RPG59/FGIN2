use web_sys::{WebGlBuffer, WebGl2RenderingContext};
use crate::core::EngineCore::GL;

struct IndexBuffer {
    gl: GL,
    buffer: WebGlBuffer,
}

impl IndexBuffer {
    pub fn new(gl: &GL, data: &[u32], usage: u32) -> Self {
        IndexBuffer {
            gl: gl.clone(),
            buffer: gl.create_buffer().unwrap(),
        }
    }

    pub fn bind(&self) {
        self.gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&self.buffer));
    }

    pub fn unbind(&self) {
        // self.gl.bind_buffer()
    }

    fn set_data(&self, data: &[u32], usage: u32) {}
}
