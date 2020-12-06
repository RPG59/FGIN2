use web_sys::{WebGl2RenderingContext};

use crate::core::VertexBuffer::VertexBuffer;
use crate::core::EngineCore::{GL};
use crate::core::Shader::Shader;
use crate::core::Types::float4x4;
use cgmath::{Matrix4};
use cgmath::prelude::*;

pub struct Mesh {
    gl: GL,
    vertices: VertexBuffer,
    pub objectType: u8,
    pub modelMatrix: float4x4,
}

impl Mesh {
    pub fn new(gl: &GL, vertices: &[f32], indices: &[u32], objectType: u8) -> Self {
        Mesh {
            gl: gl.clone(),
            vertices: VertexBuffer::new(gl, vertices, WebGl2RenderingContext::STATIC_DRAW),
            objectType,
            modelMatrix: float4x4::identity(),
        }
    }

    pub fn render(&self) {
        self.vertices.bind();
        self.gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);
    }
}
