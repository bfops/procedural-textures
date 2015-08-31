use env_logger;
use gl;
use gl::types::GLint;
use std;
use std::mem;
use sdl2;
use sdl2::event::Event;
use sdl2::video;
use stopwatch;
use yaglw::gl_context::GLContext;
use yaglw::shader::Shader;
use yaglw::texture::{BufferTexture, TextureUnit};
use yaglw::vertex_buffer::{ArrayHandle};

use texture;
use texture::T;

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

  let mut shader = make_shader(gl);
  shader.use_shader(gl);

  let pixels = make_pixels(gl);
  info!("Done loading");

  {
    let mut bind = |name, id| {
      let unit: TextureUnit = Default::default();
      unsafe {
        gl::ActiveTexture(unit.gl_id());
        gl::BindTexture(gl::TEXTURE_BUFFER, id);
      }
      let loc = shader.get_uniform_location(name);
      unsafe {
        gl::Uniform1i(loc, unit.glsl_id as GLint);
      }
    };

    bind("pixels", pixels.buffer.byte_buffer.handle.gl_id);
  }

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

fn make_shader<'a, 'b:'a>(
  gl: &'a GLContext,
) -> Shader<'b> {
  let vertex_shader: String = format!("
    #version 330 core

    out vec4 world_pos;

    void main() {{
      if (gl_VertexID == 0) {{
        world_pos = vec4(-1, 1, 0, 1);
      }} else if (gl_VertexID == 1) {{
        world_pos = vec4(-1, -1, 0, 1);
      }} else if (gl_VertexID == 2) {{
        world_pos = vec4(1, 1, 0, 1);
      }} else {{
        world_pos = vec4(1, -1, 0, 1);
      }}
      gl_Position = world_pos;
    }}
  ");

  let fragment_shader: String = format!("
    #version 330 core

    const int WINDOW_WIDTH = {};

    uniform samplerBuffer pixels;

    in vec4 world_pos;

    layout(location=0) out vec4 frag_color;

    void main() {{
      int x = int(round(gl_FragCoord.x));
      int y = int(round(gl_FragCoord.y));
      vec3 c = texelFetch(pixels, x + y*WINDOW_WIDTH).rgb;
      frag_color = vec4(c, 1);
    }}
  ", WINDOW_WIDTH);

  let components = vec!(
    (gl::VERTEX_SHADER, vertex_shader),
    (gl::FRAGMENT_SHADER, fragment_shader),
  );

  Shader::new(gl, components.into_iter())
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

fn make_pixels<'a, 'b:'a>(
  gl: &'a mut GLContext,
) -> BufferTexture<'b, f32> {
  let len = (WINDOW_WIDTH * WINDOW_HEIGHT * 3) as usize;
  let mut ram_pixels = Vec::with_capacity(len);
  unsafe {
    ram_pixels.set_len(len);
  }

  {
    let ram_pixels: &mut [[[f32; 3]; WINDOW_WIDTH as usize]; WINDOW_HEIGHT as usize] = unsafe {
      mem::transmute(ram_pixels.as_ptr())
    };
    let tex = texture::dirt::new();
  
    for y in 0..WINDOW_HEIGHT as usize {
    for x in 0..WINDOW_WIDTH as usize {
      ram_pixels[y][x] =
        tex.eval(
          x as f32 / WINDOW_WIDTH as f32,
          y as f32 / WINDOW_HEIGHT as f32,
          0.0,
        );
    }}
  }

  let mut vram_pixels = BufferTexture::new(gl, gl::RGB32F, len);
  vram_pixels.buffer.push(gl, ram_pixels.as_slice());
  vram_pixels
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
