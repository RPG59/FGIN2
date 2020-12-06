use cgmath::{Matrix4, Quaternion, Vector3, Matrix3};
use cgmath::prelude::*;
use cgmath::perspective;
use crate::core::Types::*;
use wasm_bindgen::__rt::std::fs::copy;

pub struct Camera {
    proj_matrix: float4x4,
    view_matrix: float4x4,
    rotation: qut,
    translation: float3,

    vertical_fov_rad: f32,
    aspect_ratio: f32,
    near_z_clip: f32,
    far_z_clip: f32,
}

impl Camera {
    pub fn new(
        vertical_fov_rad: f32,
        aspect_ratio: f32,
        near_z_clip: f32,
        far_z_clip: f32,
    ) -> Self {
        Camera {
            vertical_fov_rad,
            aspect_ratio,
            near_z_clip,
            far_z_clip,
            proj_matrix: Matrix4::identity(),
            view_matrix: Matrix4::identity(),
            rotation: qut::new(0.0, 0.0, 0.0, 1.0),
            translation: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        let rotationMatrix = float3x3::from(self.rotation);
        self.view_matrix = float4x4::from(rotationMatrix);
        self.view_matrix.w[0] = self.rotation[0];
        self.view_matrix.w[1] = self.rotation[1];
        self.view_matrix.w[2] = self.rotation[2];
    }

    pub fn setPerspectiveMatrix(
        &mut self,
    ) {
        // self.verticalFovRadians = verticalFovRadians;
        // self.aspectRatio = aspectRatio;
        // self.nearZClip = nearZClip;
        // self.farZClip = farZClip;

        let foo = float3x3::from(self.rotation);


        // self.updateProjMatrix();
    }

    pub fn update_proj_matrix(&mut self) {
        let y = 1. / ((self.vertical_fov_rad * 0.5) as f32).tan();
        let x = y * self.aspect_ratio;
        let q1 = self.far_z_clip / (self.near_z_clip - self.far_z_clip);
        let q2 = q1 * self.near_z_clip;

        self.proj_matrix = Matrix4::new(
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, q1, -1.0,
            0.0, 0.0, q2, 0.0,
        );
    }

    pub fn get_projection(&self) -> &float4x4 {
        &self.proj_matrix
    }

    pub fn get_view(&self) -> &float4x4 {
        &self.view_matrix
    }
}




