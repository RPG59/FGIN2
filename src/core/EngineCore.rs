use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlShader, WebGlUniformLocation};

use crate::Camera;
use crate::core::Scene::Scene;

use std::any::type_name;
use crate::core::Shader::Shader;
use std::collections::HashMap;
use cgmath::num_traits::real::Real;

pub type GL = std::rc::Rc<WebGl2RenderingContext>;

pub enum ObjectTypes {
    default = 0x01,
    transparent = 0x02,
}

pub struct EngineCore {
    width: usize,
    height: usize,
    default_shader: Shader,
    pub gl: GL,
}

impl EngineCore {
    pub fn new() -> EngineCore {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.query_selector("canvas").unwrap().unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let gl = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .map_err(|_| ())
            .unwrap();

        let width = window.inner_width().unwrap().as_f64().unwrap() as usize;
        let height = window.inner_height().unwrap().as_f64().unwrap() as usize;

        canvas.set_width(width as u32);
        canvas.set_height(height as u32);


        let gl = std::rc::Rc::new(gl);
        let glRef = gl.clone();

        EngineCore {
            width,
            height,
            gl,
            default_shader: EngineCore::get_defaultShader(&glRef),
        }
    }

    fn get_defaultShader(gl: &GL) -> Shader {
        let vs_shader = "#version 300 es\n\
layout (location=0) in vec4 a_Pos;\n\

uniform mat4 u_ModelMatrix;
uniform mat4 u_ProjMatrix;
uniform mat4 u_ViewMatrix;

void main() {\n\
    gl_Position = u_ViewMatrix * u_ProjMatrix * u_ModelMatrix * a_Pos;\n\
}";

        let fs_shader = "#version 300 es\n\
precision lowp float;\n\
out vec4 color;\n\
void main() {\n\
    color = vec4(0., .9, .1, 1.);\n\
}";

        Shader::new(gl, vs_shader.to_string(), fs_shader.to_string())
    }

    pub fn update(&mut self) {
        // input update
        // m_Camera update
    }

    pub fn render_scene(&mut self, camera: &Camera, scene: &Scene) {
        self.gl.clear_color(0.3, 0.2, 0.1, 1.);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        self.render_objects(scene, ObjectTypes::default as u8, &self.default_shader, camera);
    }

    fn render_objects(&self, scene: &Scene, filter: u8, shader: &Shader, camera: &Camera) {
        shader.enable();

        shader.set_uniform_matrix4f(&"u_ProjMatrix".to_string(), camera.get_projection());
        shader.set_uniform_matrix4f(&"u_ViewMatrix".to_string(), camera.get_view());

        for mesh in &scene.data {
            if (filter & mesh.objectType) != 0 {
                shader.set_uniform_matrix4f(&"u_ModelMatrix".to_string(), &mesh.modelMatrix);
                mesh.render();
            }
        }
    }
}
