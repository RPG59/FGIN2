use web_sys::{WebGlBuffer, WebGl2RenderingContext};
use crate::core::EngineCore::GL;

pub struct VertexBuffer {
    gl: GL,
    buffer: WebGlBuffer
}

impl VertexBuffer {
    pub fn new(gl: &GL, data: &[f32], usage: u32) -> Self {
        let buffer = VertexBuffer{
            gl: gl.clone(),
            buffer: gl.create_buffer().unwrap()
        };
        buffer.set_data(data, usage);



        buffer
    }

    pub fn bind(&self) {
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer));
    }

    pub fn unbind(&self) {
        // self.gl.bind
    }

    fn set_data(&self, data: &[f32], usage: u32) {
        use wasm_bindgen::JsCast;
        self.bind();
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>().unwrap()
            .buffer();
        let data_location = data.as_ptr() as u32 / 4;
        let array = js_sys::Float32Array::new(&memory_buffer).subarray(data_location, data_location + data.len() as u32);

        self.gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &array,
            usage
        );

        self.unbind();
    }
}
