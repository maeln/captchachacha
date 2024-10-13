use crate::camera::Camera;
use crate::mesh::Mesh;
use crate::shaders::shader_loader::ShaderManager;
use crate::shaders::Program;

use cgmath::{Matrix4, Vector2, Vector3, Vector4};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum Uniform {
    Float(f32),
    Vec2(Vector2<f32>),
    Vec3(Vector3<f32>),
    Vec4(Vector4<f32>),
}

#[derive(Debug)]
pub struct DrawableObject {
    pub program: Arc<Mutex<Program>>,
    pub mesh: Rc<RefCell<Mesh>>,
    pub transform: Matrix4<f32>,
    pub uniforms: HashMap<String, Uniform>,
}

impl DrawableObject {
    pub fn new(
        program: Arc<Mutex<Program>>,
        mesh: Rc<RefCell<Mesh>>,
        transform: Matrix4<f32>,
    ) -> Self {
        DrawableObject {
            program,
            mesh,
            transform,
            uniforms: HashMap::new(),
        }
    }
    pub fn draw(&self, state: &SceneState) {
        let _plock = self.program.try_lock();
        if let Ok(ref prog) = _plock {
            prog.bind();
            prog.set_float("time", state.time);
            prog.set_mat4("model", &self.transform);
            prog.set_mat4("view", &state.camera.view());
            prog.set_mat4("perspective", &state.camera.perspective());
            prog.set_mat4("normalMat", &state.camera.normal_mat(&self.transform));
            if prog.has_uniform("vps") {
                prog.set_vec2("vps", &state.camera.vps);
            }
            for (k, v) in self.uniforms.iter() {
                if prog.has_uniform(k) {
                    match v {
                        Uniform::Float(f) => prog.set_float(k, *f),
                        Uniform::Vec2(ve) => prog.set_vec2(k, ve),
                        Uniform::Vec3(ve) => prog.set_vec3(k, ve),
                        Uniform::Vec4(ve) => prog.set_vec4(k, ve),
                    }
                }
            }
        }
        {
            let mut borrowed_mesh = self.mesh.borrow_mut();
            borrowed_mesh.draw();
        }
    }
}

#[derive(Debug)]
pub struct SceneState {
    pub time: f32,
    pub camera: Camera,
}

#[derive(Debug)]
pub struct NewScene {
    pub shader_manager: ShaderManager,
    pub drawable: HashMap<u32, DrawableObject>,
    drawable_next_id: u32,
    pub state: SceneState,
}

impl NewScene {
    pub fn new(camera: Camera) -> Self {
        NewScene {
            shader_manager: ShaderManager::new(),
            drawable: HashMap::new(),
            drawable_next_id: 0,
            state: SceneState { time: 0.0, camera },
        }
    }

    pub fn add_drawable(&mut self, drawable: DrawableObject) -> u32 {
        self.drawable.insert(self.drawable_next_id, drawable);
        let val = self.drawable_next_id;
        self.drawable_next_id += 1;
        val
    }

    pub fn rm_drawable(&mut self, id: u32) {
        self.drawable.remove(&id);
    }

    pub fn render(&self) {
        for drawable in self.drawable.values() {
            drawable.draw(&self.state);
        }
    }
}
