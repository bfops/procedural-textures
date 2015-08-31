use env_logger;
use gl;
use std;
use std::mem;
use sdl2;
use sdl2::event::Event;
use sdl2::video;
use stopwatch;
use yaglw::gl_context::GLContext;
use yaglw::vertex_buffer::{ArrayHandle};

use shader;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 800;

pub fn main() {
  env_logger::init().unwrap();

  let sdl = sdl2::init().unwrap();
  let _event = sdl.event().unwrap();
  let video = sdl.video().unwrap();
  let window = make_window(&video);

  let _sdl_gl_context = window.gl_create_context().unwrap();

  // Load the OpenGL function pointers.
  gl::load_with(|s| unsafe {
    mem::transmute(video.gl_get_proc_address(s))
  });

  let mut gl = unsafe {
    GLContext::new()
  };
  let gl = &mut gl;

  match gl.get_error() {
    gl::NO_ERROR => {},
    err => {
      println!("OpenGL error 0x{:x} in setup", err);
      return;
    },
  }

  let shader = shader::make(gl);
  shader.use_shader(gl);

  info!("Done loading");

  let empty_vao = ArrayHandle::new(gl);

  unsafe {
    gl::BindVertexArray(empty_vao.gl_id);
  }

  info!("Looping");

  let mut event_pump = sdl.event_pump().unwrap();

  while process_events(&mut event_pump) {
    stopwatch::time("draw", || {
      gl.clear_buffer();
      unsafe {
        gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
      }
      // swap buffers
      window.gl_swap_window();
    });

    std::thread::sleep_ms(10);
  }

  info!("Done");
  stopwatch::clone().print();
}

fn make_window(video: &sdl2::VideoSubsystem) -> video::Window {
  video.gl_attr().set_context_profile(video::GLProfile::Core);
  video.gl_attr().set_context_version(3, 3);

  // Open the window as fullscreen at the current resolution.
  let mut window =
    video.window(
      "Procedural texturing",
      WINDOW_WIDTH, WINDOW_HEIGHT,
    );

  let window = window.position(0, 0);
  window.opengl();

  window.build().unwrap()
}

fn process_events<'a>(
  event_pump: &mut sdl2::EventPump,
) -> bool {
  for event in event_pump.poll_iter() {
    match event {
      Event::Quit {..} => {
        return false;
      },
      Event::AppTerminating {..} => {
        return false;
      },
      _ => {},
    }
  }

  true
}
