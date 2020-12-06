use crate::core::Mesh::Mesh;

pub struct Scene {
    pub data: Vec<Mesh>
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            data: Vec::new()
        }
    }

    pub fn add(&mut self, mesh: Mesh) {
        self.data.push(mesh);
    }
}


