pub mod perlin;

pub trait T {
  fn eval(&self, x: f32, y: f32, z: f32) -> [f32; 3];
}
