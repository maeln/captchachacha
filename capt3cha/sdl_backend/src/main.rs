#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use peglrs::scene::{DrawableObject, NewScene, Uniform};
use peglrs::shaders::{Program, ShaderType};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

use cgmath::{vec3, Deg, InnerSpace, Matrix4, Vector2};
use cgmath::{EuclideanSpace, Point3};
use sdl2::mouse::MouseButton;

use rand::prelude::*;
use rand::seq::SliceRandom;

use peglrs::mesh::Mesh;

static BILL_VS: &'static str = include_str!("./shaders/bill.vs");
static BASE_FS: &'static str = include_str!("./shaders/base.fs");
static BASE_VS: &'static str = include_str!("./shaders/base.vs");

fn char3d(c: &char) -> Mesh {
    match c {
        'a' => Mesh::glyph_a(),
        'b' => Mesh::glyph_b(),
        'c' => Mesh::glyph_c(),
        'd' => Mesh::glyph_d(),
        'e' => Mesh::glyph_e(),
        'f' => Mesh::glyph_f(),
        'g' => Mesh::glyph_g(),
        'h' => Mesh::glyph_h(),
        'i' => Mesh::glyph_i(),
        'j' => Mesh::glyph_j(),
        'k' => Mesh::glyph_k(),
        'l' => Mesh::glyph_l(),
        'm' => Mesh::glyph_m(),
        'n' => Mesh::glyph_n(),
        'o' => Mesh::glyph_o(),
        'p' => Mesh::glyph_p(),
        'q' => Mesh::glyph_q(),
        'r' => Mesh::glyph_r(),
        's' => Mesh::glyph_s(),
        't' => Mesh::glyph_t(),
        'u' => Mesh::glyph_u(),
        'v' => Mesh::glyph_v(),
        'w' => Mesh::glyph_w(),
        'x' => Mesh::glyph_x(),
        'y' => Mesh::glyph_y(),
        'z' => Mesh::glyph_z(),
        '0' => Mesh::glyph_0(),
        '1' => Mesh::glyph_1(),
        '2' => Mesh::glyph_2(),
        '3' => Mesh::glyph_3(),
        '4' => Mesh::glyph_4(),
        '5' => Mesh::glyph_5(),
        '6' => Mesh::glyph_6(),
        '7' => Mesh::glyph_7(),
        '8' => Mesh::glyph_8(),
        '9' => Mesh::glyph_9(),
        _ => Mesh::glyph_0(),
    }
}

type MeshMap = HashMap<char, Rc<RefCell<Mesh>>>;

fn load_all_and_ready_up() -> MeshMap {
    let all_char = ('a'..='z').chain('0'..='9');
    let mut map: MeshMap = HashMap::new();
    for c in all_char {
        let mesh = Rc::new(RefCell::new(char3d(&c)));
        {
            let mut br = mesh.borrow_mut();
            br.ready_up();
        }
        map.insert(c, mesh);
    }
    map
}

fn rand_captcha(size: usize) -> Vec<char> {
    let mut rng = &mut rand::thread_rng();
    let chars: Vec<char> = ('a'..='z').chain('0'..='9').collect();
    let rand_glyph = chars.choose_multiple(&mut rng, size).cloned().collect();
    return rand_glyph;
}

static HELP_TXT: [char; 28] = [
    'c', 'l', 'i', 'c', 'k', ' ', 'a', 'n', 'd', ' ', 'd', 'r', 'a', 'g', ' ', 'm', 'o', 'u', 's',
    'e', ' ', 't', 'o', ' ', 'm', 'o', 'v', 'e',
];

fn display_txt(
    mesh_map: &MeshMap,
    scene: &mut NewScene,
    program: Arc<Mutex<Program>>,
    txt: &[char],
    y_offset: f32,
    x_offset: f32,
    scale: f32,
    spacing: f32,
    color: f32,
) -> Vec<u32> {
    let mut drawable_id = vec![];
    let base_transform =
        Matrix4::from_translation(vec3(x_offset, y_offset, 0.0)) * Matrix4::from_scale(scale);
    for (i, c) in txt.iter().enumerate() {
        if *c == ' ' {
            continue;
        }
        let mesh = mesh_map.get(c).unwrap().clone();
        let transform =
            Matrix4::from_translation(vec3(i as f32 * spacing, 0.0, 0.0)) * base_transform;
        let mut drawable = DrawableObject::new(program.clone(), mesh, transform);
        drawable
            .uniforms
            .insert("color".to_string(), Uniform::Float(color));
        drawable_id.push(scene.add_drawable(drawable));
    }
    drawable_id
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut view_x: i32 = 800;
    let mut view_y: i32 = 600;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_version(4, 1);
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);

    let window = video_subsystem
        .window("CAPT3CHA", view_x as u32, view_y as u32)
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let _gl_ctx = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|symbol| video_subsystem.gl_get_proc_address(symbol) as *const _);
    unsafe {
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
        gl::FrontFace(gl::CCW);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthMask(gl::TRUE);
        gl::DepthFunc(gl::LEQUAL);
        gl::DepthRange(0.0, 0.0);
        gl::Enable(gl::DEPTH_CLAMP);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    peglrs::init_gl(view_x, view_y);

    let mesh_map = load_all_and_ready_up();

    // Setup the camera
    let target = vec3(0.0, 0.5, 0.0);
    let position = vec3(0.0, 1.6, 4.0);
    let camera = peglrs::camera::Camera::new(
        Point3::from_vec(position),
        Point3::from_vec(target),
        60.0,
        view_x,
        view_y,
    );

    let mut shader_manager = peglrs::shaders::shader_loader::ShaderManager::new();
    let bill_vs = Arc::new(Mutex::new(
        shader_manager.compile_shader(&BILL_VS, ShaderType::VERTEX),
    ));
    let base_vs = Arc::new(Mutex::new(
        shader_manager.compile_shader(&BASE_VS, ShaderType::VERTEX),
    ));
    let base_fs = Arc::new(Mutex::new(
        shader_manager.compile_shader(&BASE_FS, ShaderType::FRAGMENT),
    ));

    let base_program_id = shader_manager.mk_program(&vec![base_vs, base_fs.clone()]);
    let bill_program_id = shader_manager.mk_program(&vec![bill_vs, base_fs.clone()]);

    let base_program_arc = shader_manager.get_program(base_program_id).unwrap();
    let bill_program_arc = shader_manager.get_program(bill_program_id).unwrap();

    let mut scene = NewScene::new(camera);

    // Create the captcha
    let rng = &mut rand::thread_rng();
    let rand_str = rand_captcha(6);
    for (i, c) in rand_str.iter().enumerate() {
        let mesh = mesh_map.get(c).unwrap().clone();
        let color: f32 = rng.gen_range(0.05..=0.95);
        let xd: f32 = rng.gen_range(-0.2..=0.2);
        let yd: f32 = rng.gen_range(-0.3..=0.3);
        let zd: f32 = rng.gen_range(-0.3..=0.3);
        let scale: f32 = rng.gen_range(0.8..=1.2);
        let r: f32 = rng.gen_range(0.0..180.0);
        let rx: f32 = rng.gen_range(0.0..=1.0);
        let ry: f32 = rng.gen_range(0.0..=1.0);
        let rz: f32 = rng.gen_range(0.0..=1.0);
        let rvec = vec3(rx, ry, rz).normalize();
        let transform = Matrix4::from_translation(vec3(i as f32 - 3.0 + xd, yd, zd))
            * Matrix4::from_scale(scale)
            * Matrix4::from_axis_angle(rvec, Deg { 0: r });
        let mut glyph = DrawableObject::new(base_program_arc.clone(), mesh, transform);
        glyph
            .uniforms
            .insert("color".to_string(), Uniform::Float(color));
        scene.add_drawable(glyph);
    }

    display_txt(
        &mesh_map,
        &mut scene,
        bill_program_arc.clone(),
        &HELP_TXT,
        0.90,
        -0.94,
        0.06,
        0.04,
        0.4,
    );

    // State variables
    let mouse_sensitivity = 0.01;
    let scroll_sensitivity = 0.1;
    let mut mouse_clicked = false;
    let mut user_input: Vec<char> = Vec::new();
    let mut input_drawable: Vec<u32> = Vec::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let counter = Instant::now();
    let mut kill_started = false;
    let mut kill_count = Instant::now();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => {
                    view_x = width;
                    view_y = height;
                    peglrs::resize_window(view_x, view_y);
                    scene.state.camera.vps = Vector2 {
                        x: view_x as f32,
                        y: view_y as f32,
                    };
                }
                Event::Quit { .. } => break 'main,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if key == Keycode::ESCAPE {
                        break 'main;
                    }
                    if key == Keycode::BACKSPACE || key == Keycode::DELETE {
                        user_input.pop();
                        if let Some(d_id) = input_drawable.pop() {
                            scene.rm_drawable(d_id);
                        }
                    }
                }
                Event::TextInput { text, .. } => {
                    for c in text.chars() {
                        if ('a' <= c && c <= 'z') || ('0' <= c && c <= '9') {
                            user_input.push(c);
                            for d_id in input_drawable.iter() {
                                scene.rm_drawable(*d_id);
                            }
                            input_drawable.extend_from_slice(&display_txt(
                                &mesh_map,
                                &mut scene,
                                bill_program_arc.clone(),
                                &user_input,
                                -0.80,
                                0.0,
                                0.09,
                                0.06,
                                0.8,
                            ));
                        }
                    }
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    mouse_clicked = true;
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    mouse_clicked = false;
                }
                Event::MouseWheel { precise_y, .. } => {
                    scene.state.camera.scroll(-precise_y, scroll_sensitivity);
                }
                Event::MouseMotion { xrel, yrel, .. } => {
                    if mouse_clicked {
                        scene
                            .state
                            .camera
                            .rotate(xrel as f32, yrel as f32, mouse_sensitivity);
                    }
                }
                _ => {}
            }
        }

        if user_input == rand_str {
            if !kill_started {
                kill_count = Instant::now();
                kill_started = true;
            }
            if kill_started && kill_count.elapsed().as_secs() > 5 {
                break 'main;
            }
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
                gl::ClearColor(50.0 / 255.0, 168.0 / 255.0, 82.0 / 255.0, 0.0);
            }
        } else {
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
                gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            }
            scene.state.time = counter.elapsed().as_secs_f32();
            scene.render();
        }
        window.gl_swap_window();
    }
}
