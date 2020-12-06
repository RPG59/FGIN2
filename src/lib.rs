use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader, HtmlCanvasElement};

mod core;

use crate::core::EngineCore::{EngineCore, ObjectTypes};
use crate::core::Camera::Camera;
use crate::core::Scene::Scene;
use crate::core::Mesh::Mesh;
use crate::core::Types::float4x4;
use cgmath::Rad;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();

    let mut engineCore = EngineCore::new();
    let gl = engineCore.gl.clone();
    let mut camera = Camera::new(
        std::f32::consts::PI / 4.0,
        16.0 / 9.0,
        0.1,
        100.,
    );

    let mut scene = Scene::new();
    let mut test_mesh = Mesh::new(&gl, &[-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0], &[1], ObjectTypes::default as u8);
    test_mesh.modelMatrix = float4x4::from_angle_x(Rad(3.14));

    scene.add(test_mesh);

    engineCore.render_scene(&camera, &scene);

    Ok(())
}
