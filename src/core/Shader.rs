use web_sys::{WebGlShader, WebGlProgram, WebGl2RenderingContext, WebGlUniformLocation};
use std::fs;
use crate::core::EngineCore::GL;
use crate::core::Types::{float4x4, to_slice};
use std::collections::HashMap;

pub struct Shader {
    gl: GL,
    program: WebGlProgram,
    // uniforms: HashMap<String, WebGlUniformLocation>,
}

impl Shader {
    pub fn new(gl: &GL, vs_path: String, fs_path: String) -> Self {
        let vs_shader = Shader::compile(gl, vs_path, WebGl2RenderingContext::VERTEX_SHADER);
        let fs_shader = Shader::compile(gl, fs_path, WebGl2RenderingContext::FRAGMENT_SHADER);

        Shader {
            gl: gl.clone(),
            program: Shader::creteProgram(gl, &vs_shader, &fs_shader),
        }
    }

    fn get_shader_source(path: String) -> String {
        let data = fs::read_to_string(path).expect(("ERROR: [get_shader_source]: "));
        return data;
    }

    fn compile(gl: &GL, source: String, shader_type: u32) -> WebGlShader {
        let shader = gl.create_shader(shader_type).unwrap();

        gl.shader_source(&shader, &source);
        gl.compile_shader(&shader);

        if !gl.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false) {
            let log = gl.get_shader_info_log(&shader).unwrap();
            // console::log_1(&log.into());
        }

        shader
    }

    fn creteProgram(gl: &GL, vs_shader: &WebGlShader, fs_shader: &WebGlShader) -> WebGlProgram {
        let program = gl.create_program().unwrap();

        gl.attach_shader(&program, vs_shader);
        gl.attach_shader(&program, fs_shader);

        gl.delete_shader(Some(vs_shader));
        gl.delete_shader(Some(fs_shader));

        gl.link_program(&program);

        program
    }

    pub fn enable(&self) {
        self.gl.use_program(Some(&self.program));
        self.gl.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(0);
    }

    pub fn set_uniform_matrix4f(&self, name: &String, data: &float4x4) {
        let location = self.gl.get_uniform_location(&self.program, name).unwrap();
        self.gl.uniform_matrix4fv_with_f32_array(Some(&location), false, &to_slice(data));
    }
}
