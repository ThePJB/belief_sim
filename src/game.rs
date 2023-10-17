use glow::HasContext;
use minvect::*;
extern crate glow_mesh;
use glow_mesh::xyzrgba::*;
use glow_mesh::xyzrgba_build2d::*;
use glutin::event::{Event, WindowEvent};
use winit::event::ElementState;
use winit::event::VirtualKeyCode;
use crate::rng;
use crate::rng::Rng;
use crate::rng::random_seed;
use crate::sim::*;
use crate::matrix::*;
pub struct Game {
    xres: i32,
    yres: i32,
    window: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    gl: glow::Context,
    sim: Sim,

    prog: ProgramXYZRGBA,
    h: HandleXYZRGBA,
}

impl Game {
    pub fn new(event_loop: &glutin::event_loop::EventLoop<()>) -> Self {
        let xres = 512;
        let yres = 512;
    
        unsafe {
            let window_builder = glutin::window::WindowBuilder::new()
                .with_title("triangle test")
                .with_inner_size(glutin::dpi::PhysicalSize::new(xres, yres));
            let window = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap();
    
            let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
            gl.enable(glow::DEPTH_TEST);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
            gl.depth_func(glow::LEQUAL);
            gl.enable(glow::BLEND);
    
            let prog = ProgramXYZRGBA::default(&gl);
            let sim = Sim::new(69, 1.0);
            let mut buf = sim.render();
            put_quad(&mut buf, vec2(1.0, 1.0), vec2(1.0, 0.0), vec2(0.0, 0.0), vec2(0.0, 1.0), vec4(0.0, 0.0, 1.0, 1.0), 0.6);
            let h = upload_xyzrgba_mesh(&buf, &gl);
            prog.bind(&gl);

            Game {
                xres,
                yres,
                window,
                gl,
                sim: Sim::new(69, 1.0),
                prog,
                h,
            }
        }
    }

    pub fn handle_event(&mut self, event: glutin::event::Event<()>) {
        unsafe {
            match event {
                Event::LoopDestroyed |
                Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
                    std::process::exit(0);
                },

                Event::WindowEvent {event, .. } => {
                    match event {
                        WindowEvent::Resized(size) => {
                            self.xres = size.width as i32;
                            self.yres = size.height as i32;
                            self.gl.viewport(0, 0, size.width as i32, size.height as i32);
                        },
                        WindowEvent::KeyboardInput { input, .. } => {
                            if let Some(vkk) = input.virtual_keycode {
                                match vkk {
                                    VirtualKeyCode::R if input.state == ElementState::Released => {self.sim = Sim::new(random_seed(), 1.0)},
                                    _ => {},
                                }
                            }
                        },
                        _ => {},
                    }
                },
                Event::MainEventsCleared => {
                    self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
                    self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT); 
                    let t = mat4_translation(-0.5, -0.5);
                    let s = mat4_scaling(2.0);
                    let p = mat4_multiply(&s, &t);
                    // let p = [1.0f32, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,];
                    self.prog.set_proj(&p, &self.gl);
                    self.h.render(&self.gl);
                    self.window.swap_buffers().unwrap();
                },
                _ => {},
            }
        }
    }
}