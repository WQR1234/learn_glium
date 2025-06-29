extern crate core;

mod object_3d;
mod camera;
mod my_simple_window_builders;

use cgmath::InnerSpace;
use glium::{Display, Surface};
use glium::glutin::surface::WindowSurface;
use glium::winit::dpi::{LogicalSize, PhysicalPosition, PhysicalSize};
use glium::uniforms::{DynamicUniforms};


use crate::object_3d::{Object3d, Object3dKind, Vertex, Material};
use crate::object_3d::light::{Light, DirectionalLight, PointLight, FlashLight};
use crate::object_3d::cube::CUBE_SHAPE;
use crate::object_3d::sphere::SPHERE_SHAPE_INDEX;
use crate::camera::Camera;

use glium::winit::application::ApplicationHandler;
use glium::winit::event::{DeviceEvent, DeviceId, ElementState, KeyEvent, MouseButton, WindowEvent};
use glium::winit::event_loop::{ActiveEventLoop};
use glium::winit::window::{Window, WindowId};

use std::collections::HashMap;
use std::{fs, io};
use cgmath::num_traits::Float;
use crate::my_simple_window_builders::MySimpleWindowBuilder;

struct GLBuffer {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: Option<glium::IndexBuffer<u16>>,
}



struct State {
    window: Window,
    display: Display<WindowSurface>,

    // vertex_buffer: Option<glium::VertexBuffer<Vertex>>,
    // index_buffer: Option<glium::IndexBuffer<u16>>,

    cube_buffer: GLBuffer,
    sphere_buffer: GLBuffer,

    programs: HashMap<String ,glium::Program>,
    textures: Vec<glium::texture::Texture2d>,

    now: std::time::Instant,
    last_frame: f32,
    delta_time: f32,

    object3ds: Vec<Object3d>,

    projection: [[f32; 4]; 4],

    camera: Camera,

    // last_mouse_pos: (f32, f32),
    camera_rotate: bool,

    // light_pos: Option<cgmath::Vector3<f32>>,
    // light_color: Option<[f32; 3]>,

    light: Option<Light>,
    directional_light: Option<DirectionalLight>,
    point_light: Option<PointLight>,
    flash_light: Option<FlashLight>,
}

impl State {

    fn new(event_loop: &ActiveEventLoop) -> Self {
        let (window, display) = MySimpleWindowBuilder::new()
            .with_title("test01")
            .with_vsync(true)
            .build(event_loop);

        let _ = window.request_inner_size(LogicalSize::new(800.0, 600.0));   // resize window

        let projection= Self::generate_projection(cgmath::Deg(45.0), window.inner_size(), 0.1, 100.0);

        let cube_buffer = GLBuffer {
            vertex_buffer: glium::VertexBuffer::new(&display, CUBE_SHAPE).unwrap(),
            index_buffer: None
        };

        let sphere_buffer = GLBuffer {
            vertex_buffer: glium::VertexBuffer::new(&display, SPHERE_SHAPE_INDEX.0.as_slice()).unwrap(),
            index_buffer: Some(glium::IndexBuffer::new(&display,
                                                  glium::index::PrimitiveType::TrianglesList,
                                                  SPHERE_SHAPE_INDEX.1.as_slice())
                .unwrap())
        };

        Self {
            window,
            display,
            cube_buffer,
            sphere_buffer,
            programs: HashMap::new(),
            textures: Vec::new(),
            now: std::time::Instant::now(),
            last_frame: 0.0,
            delta_time: 0.0,
            object3ds: vec![],
            projection,
            camera: Camera::new(),
            // last_mouse_pos: (800.0/2.0, 600.0/2.0),
            camera_rotate: false,
            // light_pos: None,
            // light_color: None,
            light: None,
            directional_light: None,
            point_light: None,
            flash_light: None,
        }
    }

    fn add_shader(&mut self, shader_name : &str) -> io::Result<()> {
        let vertex_shader_path = format!("shaders/{}.vert", shader_name);
        let fragment_shader_path = format!("shaders/{}.frag", shader_name);
        let vertex_shader_src = fs::read_to_string(vertex_shader_path)?;
        let fragment_shader_src = fs::read_to_string(fragment_shader_path)?;
        let program = glium::Program::from_source(&self.display,
                                                  vertex_shader_src.as_str(),
                                                  fragment_shader_src.as_str(), None)
            .unwrap();
        self.programs.insert(shader_name.to_string(), program);

        Ok(())
    }

    fn start(&mut self) {

        // shader
        // self.add_shader("texture").unwrap();
        // self.add_shader("color").unwrap();
        self.add_shader("light").unwrap();
        self.add_shader("light_map").unwrap();

        // texture
        self.textures.push(self.generate_texture("container2.png"));
        self.textures.push(self.generate_texture("container2_specular.png"));

        // light
        // self.light_color = Some([1.0, 1.0, 1.0]);
        // self.light_pos = Some(cgmath::vec3(1.2, 1.0, 2.0));

        let light_pos = [1.2, 1.0, 2.0];
        let light_ambient = [0.1, 0.1, 0.1f32];
        let light_diffuse = [0.8, 0.8, 0.8f32];
        let light_specular = [1.0, 1.0, 1.0f32];
        // self.light = Some(Light::new(light_pos, light_ambient, light_diffuse, light_specular, Object3dKind::Sphere));

        // directional light
        // let light_dir = [-0.2, -1.0, -0.3f32];
        // self.directional_light = Some(DirectionalLight::new(light_dir, light_ambient, light_diffuse, light_specular));

        // point light
        // let light = Light::new(light_pos, light_ambient, light_diffuse, light_specular, Object3dKind::Sphere);
        // self.point_light = Some(PointLight {
        //     light,
        //     constant: 1.0,
        //     linear: 0.09,
        //     quadratic: 0.032,
        // });

        // flashlight
        self.flash_light = Some(FlashLight {
            position: self.camera.pos.into(),
            direction: self.camera.front.into(),
            cut_off: 12.5.to_radians().cos(),
            outer_cut_off: 17.5.to_radians().cos(),
            ambient: light_ambient,
            diffuse: light_diffuse,
            specular: light_specular,
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
        });

        // add cubes
        self.add_objects();
    }

    fn get_uniforms<'a>(&'a self, object3d: &'a Object3d, view: &'a [[f32; 4]; 4]) ->  DynamicUniforms<'a, 'a>   {
        let model: &[[f32; 4]; 4]  = object3d.model.as_ref();

        let camera_pos: &[f32; 3] = self.camera.pos.as_ref();

        let mut uniforms = glium::dynamic_uniform! {
            model: model,
            view: view,
            projection: &self.projection,
            viewPos: camera_pos,
            
        };


        // if let Some(light_pos) = &self.light_pos {
        //     let light_pos: &[f32; 3] = light_pos.as_ref();
        //     uniforms.add("lightPos", light_pos);
        // }
        // if let Some(light_color) = &self.light_color {
        //     uniforms.add("lightColor", light_color)
        // }
        if let Some(light) = &self.light {
            uniforms.add("light.position", light.get_position());
            uniforms.add("light.ambient", &light.ambient);
            uniforms.add("light.diffuse", &light.diffuse);
            uniforms.add("light.specular", &light.specular);
        }

        if let Some(light) = &self.directional_light {
            uniforms.add("light.direction", &light.direction);
            uniforms.add("light.ambient", &light.ambient);
            uniforms.add("light.diffuse", &light.diffuse);
            uniforms.add("light.specular", &light.specular);
        }

        if let Some(point_light) = &self.point_light {
            uniforms.add("light.position", point_light.light.get_position());
            uniforms.add("light.ambient", &point_light.light.ambient);
            uniforms.add("light.diffuse", &point_light.light.diffuse);
            uniforms.add("light.specular", &point_light.light.specular);

            uniforms.add( "light.constant", &point_light.constant);
            uniforms.add("light.linear", &point_light.linear);
            uniforms.add("light.quadratic", &point_light.quadratic);
        }
        

        if let Some(texture_id) = object3d.texture_id {
            uniforms.add("tex", &self.textures[texture_id]);
        }

        
        if let Some(color) = &object3d.color {
            uniforms.add("objectColor", color);
        }

        if let Some(material) = &object3d.material {
            // uniforms.add("material.ambient", &material.ambient);
            uniforms.add("material.diffuse", &self.textures[material.diffuse]);
            uniforms.add("material.specular", &self.textures[material.specular]);
            uniforms.add("material.shininess", &material.shininess);    

        }

        if let Some(flashlight) = &self.flash_light {
            // println!("{:?}, {:?}", flashlight.position, flashlight.direction);
            flashlight.set_uniforms(&mut uniforms);
        }

        uniforms
    }

    fn draw_object3d(&self, frame: &mut glium::Frame, object3d: &Object3d, view: &[[f32; 4]; 4], params: &glium::DrawParameters) {

        let obj_buffer = match object3d.kind {
            Object3dKind::Cube => { &self.cube_buffer },
            Object3dKind::Sphere => { &self.sphere_buffer },
        };

        let uniforms = self.get_uniforms(object3d, view);

        let program = self.programs.get(object3d.shader_name.as_str()).unwrap();

        if let Some(indices) = &obj_buffer.index_buffer {
            frame.draw(&obj_buffer.vertex_buffer, indices, program, &uniforms, params).unwrap()
        } else {
            frame.draw(&obj_buffer.vertex_buffer,
                       glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                       program, &uniforms, params).unwrap()
        }
    }

    fn draw_frame(&mut self) {
        // get delta time
        let current_frame = self.now.elapsed().as_secs_f32() * 1000.0;
        self.delta_time = current_frame - self.last_frame;
        self.last_frame = current_frame;

        let mut frame = self.display.draw();
        frame.clear_color_and_depth((0.1, 0.1, 0.1, 1.0), 1.0);

        // let radius = 10f32;
        //
        // self.camera.pos.x = self.now.elapsed().as_secs_f32().sin() * radius;
        // self.camera.pos.z = self.now.elapsed().as_secs_f32().cos() * radius;
        // self.camera.front = cgmath::point3(0., 0. ,0.) - self.camera.pos;

        let view = self.camera.get_view();
        let view: [[f32; 4]; 4] = view.into();

        if let Some(flashlight) = &mut self.flash_light {
            flashlight.position = self.camera.pos.into();
            flashlight.direction = self.camera.front.into();
        }

        // move light position
        // if let Some(light_pos) = &mut self.light_pos {
        //     let radius = 10f32;
        //     light_pos.x = self.now.elapsed().as_secs_f32().sin() * radius;
        //     light_pos.z = self.now.elapsed().as_secs_f32().cos() * radius;

        //     let light = &mut self.object3ds[0];
        //     light.reset();
        //     light.translate(*light_pos);
        //     light.scale(cgmath::vec3(0.2, 0.2, 0.2));
        // }


        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        // change light color
        // self.change_light_color();

        // draw light
        if let Some(light) = &self.light {
            let light_obj = light.get_object();
            self.draw_object3d(&mut frame, light_obj, &view, &params);
        }

        // draw point light
        if let Some(point_light) = &self.point_light {
            let light_obj = point_light.light.get_object();
            self.draw_object3d(&mut frame, light_obj, &view, &params);
        }

        // draw objects
        for object_3d in &self.object3ds {
            self.draw_object3d(&mut frame, object_3d, &view, &params);
        }

        frame.finish().unwrap();
    }

    fn generate_projection<A: Into<cgmath::Rad<f32>>>(fov: A, window_size: PhysicalSize<u32>,
                                                      near: f32, far: f32) -> [[f32; 4]; 4] {
        let aspect_ratio = window_size.width as f32 / window_size.height as f32;

        let projection = cgmath::perspective(fov, aspect_ratio, near, far);
        projection.into()
    }

    fn generate_texture(&self, tex_path: &str) -> glium::texture::Texture2d {
        // let image = image::load(std::io::Cursor::new(&include_bytes!(tex_path)),
        //                         image::ImageFormat::from_path(tex_path).unwrap()).unwrap().to_rgba8();
        let image = image::ImageReader::open(tex_path).unwrap().decode().unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        glium::texture::Texture2d::new(&self.display, image).unwrap()

    }

    fn add_objects(&mut self) {

        let cube_positions = vec![
            cgmath::vec3(0., 0., 0f32),
            cgmath::vec3(2., 5., -15.),
            cgmath::vec3(-1.5, -2.2, -2.5),
            cgmath::vec3(-3.8, -2.0, -12.3),
            cgmath::vec3(2.4, -0.4, -3.5),
            cgmath::vec3(-1.7,  3.0, -7.5),
            cgmath::vec3( 1.3, -2.0, -2.5),
            cgmath::vec3( 1.5,  2.0, -2.5),
            cgmath::vec3(1.5,  0.2, -1.5),
            cgmath::vec3( -1.3,  1.0, -1.5),
        ];

        for (i, &cube_pos) in cube_positions.iter().enumerate() {
            let mut cube = Object3d::new(Object3dKind::Cube);
            cube.shader_name = "light_map".to_string();
            let material = Material {
                diffuse: 0,
                specular: 1,
                shininess: 32.0,
            };
            cube.material = Some(material);

            cube.translate(cube_pos);
            let angle = 20.0 * i as f32;
            cube.rotate(cgmath::vec3(1.0, 0.3, 0.5).normalize(), cgmath::Deg(angle));

            self.object3ds.push(cube);
        }

        // let mut cube = Object3d::new(Object3dKind::Cube);
        // cube.shader_name = "light_map".to_string();
        // cube.color = Some([1.0, 0.5, 0.31]);
        // let material = Material {
        //     ambient: [1.0, 0.5, 0.31],
        //     diffuse: 0,
        //     specular: 1,
        //     shininess: 32.0,
        // };
        // cube.material = Some(material);

        // self.object3ds.push(cube);


    }

    pub fn change_light_color(&mut self) {
        if let Some(light) = &mut self.light {
            
            let time_rate = [2.0, 0.7, 1.3];
            let light_color = time_rate.map(|x| (self.now.elapsed().as_secs_f32() * x).sin());
            
            light.diffuse = light_color.map(|x| x * 0.5);
            light.ambient = light.diffuse.map(|x| x * 0.2);
        }

    }
}

struct App {
    state: Option<State>
}

impl App {
    fn key_callback(&mut self, event: KeyEvent) {
        use glium::winit::keyboard::{PhysicalKey, KeyCode};

        if let Some(state) = &mut self.state {
            let speed = state.delta_time * 0.04;
            if event.state == ElementState::Pressed {
                match &event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyW) => {
                        state.camera.pos += speed * state.camera.front;
                    }
                    PhysicalKey::Code(KeyCode::KeyA) => {
                        state.camera.pos -= speed * state.camera.front.cross(state.camera.up).normalize();
                    }
                    PhysicalKey::Code(KeyCode::KeyS) => {
                        state.camera.pos -= speed * state.camera.front;
                    }
                    PhysicalKey::Code(KeyCode::KeyD) => {
                        state.camera.pos += speed * state.camera.front.cross(state.camera.up).normalize();
                    }
                    _ => {}
                }
            }
        }
    }

    fn mouse_callback(&mut self, delta: (f64, f64) ) {
        if let Some(state) = &mut self.state {
            if !state.camera_rotate {
                return;
            }

            let x_offset = delta.0 as f32;
            let y_offset = -delta.1 as f32;

            let sensitivity = 0.08f32;
            // let yew = x_offset * sensitivity;
            // let mut pitch = y_offset * sensitivity;
            // pitch = pitch.min(89.0);
            // pitch = pitch.max(-89.0);

            // let euler_angle = cgmath::Euler::new(cgmath::Deg(pitch), cgmath::Deg(yew), cgmath::Deg(0.0));
            // state.camera.rotate(euler_angle);

            state.camera.rotate(x_offset * sensitivity, y_offset * sensitivity);

        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut state = State::new(event_loop);
        state.start();
        self.state = Some(state);

    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::Resized(size) => {
                if let Some(state) = &mut self.state {
                    state.display.resize(size.into());
                }
            },
            WindowEvent::RedrawRequested => {
                if let Some(state) = &mut self.state {
                    state.draw_frame();
                }
            },
            WindowEvent::KeyboardInput {
                event, ..
            } => {
                self.key_callback(event);
            },
            WindowEvent::MouseInput { state, button, .. } => {
                if let Some(self_state) = &mut self.state {
                    if state==ElementState::Pressed && button == MouseButton::Right {
                        self_state.camera_rotate = true;
                    } else {
                        self_state.camera_rotate = false;
                    }
                }
            }
            _ => ()
        }
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion {delta} => {
                self.mouse_callback(delta);
            },
            _ => ()
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &mut self.state {
            state.window.request_redraw();
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.state = None;
    }

}


fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build().expect("Couldn't create the event_loop");

    let mut app = App { state: None };
    let result = event_loop.run_app(&mut app);
    result.unwrap();
}
