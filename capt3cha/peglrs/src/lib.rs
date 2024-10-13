extern crate cgmath;
extern crate gl;
extern crate gl_loader;

pub mod camera;
pub mod frame;
pub mod mesh;
pub mod scene;
pub mod shaders;
pub mod utils;

use std::ffi::CStr;

#[no_mangle]
pub fn resize_window(width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}

#[no_mangle]
pub fn load_gl_symbol() {
    gl_loader::init_gl();
    gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);
}

#[no_mangle]
pub fn print_gl_info() {
    unsafe {
        let gl_version = gl::GetString(gl::VERSION);
        let glsl_version = gl::GetString(gl::SHADING_LANGUAGE_VERSION);

        if gl_version != std::ptr::null() && glsl_version != std::ptr::null() {
            let gl_version = CStr::from_ptr(gl_version as *const i8);
            let glsl_version = CStr::from_ptr(glsl_version as *const i8);

            println!("OGL v.{:?}", gl_version);
            println!("GLSL v.{:?}", glsl_version);
        }
    }
}

#[no_mangle]
pub fn init_gl(width: i32, height: i32) {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthMask(gl::TRUE);
        gl::DepthFunc(gl::LEQUAL);
        gl::DepthRange(0.0, 1.0);
        gl::Enable(gl::DEPTH_CLAMP);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::ONE, gl::ONE);
        gl::Viewport(0, 0, width, height);
    }
}
