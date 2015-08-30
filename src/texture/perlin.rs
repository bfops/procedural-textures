use noise;

pub struct T;

pub fn new() -> T {
  T
}

impl ::texture::T for T {
  fn eval(&self, x: f32, y: f32, z: f32) -> [f32; 3] {
    let seed = noise::Seed::new(0);
    let d = (noise::perlin3(&seed, &[x * 16.0, y * 16.0, z * 16.0]) + 1.0) / 2.0;
    [d, d, d]
  }
}
