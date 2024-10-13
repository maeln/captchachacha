
use cgmath::{Matrix4, Point3, Rad, Vector2, Vector3};
use cgmath::{prelude::*, PerspectiveFov};

pub enum Direction {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl PartialEq for Direction {
    fn eq(&self, other: &Direction) -> bool {
        self == other
    }
}

#[derive(Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub forward: Vector3<f32>,
    pub right: Vector3<f32>,
    pub up: Vector3<f32>,
    pub fov: f32,
    pub vps: Vector2<f32>,
}

impl Camera {
    pub fn new(
        position: Point3<f32>,
        target: Point3<f32>,
        fov: f32,
        width: i32,
        height: i32,
    ) -> Camera {
        let forward = (position - target).normalize();
        let right = Vector3::unit_y().cross(forward).normalize();
        let up = forward.cross(right).normalize();
        Camera {
            position,
            target,
            forward,
            right,
            up,
            fov: fov * 0.017453,
            vps: Vector2 {
                x: width as f32,
                y: height as f32,
            },
        }
    }

    fn update(&mut self) {
        self.forward = (self.position - self.target).normalize();
        self.right = Vector3::unit_y().cross(self.forward).normalize();
        self.up = self.forward.cross(self.right).normalize();
    }

    pub fn view(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.position, self.target, self.up)
    }

    pub fn perspective(&self) -> Matrix4<f32> {
        let perspective = PerspectiveFov {
            fovy: Rad { 0: self.fov },
            aspect: self.vps.x as f32 / self.vps.y as f32,
            near: 0.1,
            far: 100.0,
        };
        Matrix4::from(perspective.to_perspective())
    }

    pub fn normal_mat(&self, model: &Matrix4<f32>) -> Matrix4<f32> {
        let model_view = self.view() * model;
        model_view.invert().unwrap().transpose()
    }

    pub fn move_cam(&mut self, dir: &Direction, dt: f32, speed: f32) {
        let vel = speed * dt;
        match dir {
            &Direction::FORWARD => self.position = self.position + self.forward * vel,
            &Direction::BACKWARD => self.position = self.position - self.forward * vel,
            &Direction::LEFT => self.position = self.position - self.right * vel,
            &Direction::RIGHT => self.position = self.position + self.right * vel,
            &Direction::UP => self.position = self.position + self.up * vel,
            &Direction::DOWN => self.position = self.position - self.up * vel,
        }
        self.update();
    }

    pub fn scroll(&mut self, y: f32, sensitivity: f32) {
        self.position += self.forward * sensitivity * y;
        self.update();
    }

    pub fn rotate(&mut self, dx: f32, dy: f32, sensitivity: f32) {
        self.position += self.right * sensitivity * dx;
        self.position += self.up * sensitivity * dy;
        self.update();
    }
}
