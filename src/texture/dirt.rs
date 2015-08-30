use noise;

pub struct T;

pub fn new() -> T {
  T
}

struct Wave {
  freq: f32,
  amp: f32,
}

impl ::texture::T for T {
  fn eval(&self, x: f32, y: f32, z: f32) -> [f32; 3] {
    let seed = noise::Seed::new(0);

    let waves = [
      Wave { freq: 8.0,   amp: 1.0 },
      Wave { freq: 16.0,  amp: 0.6 },
      Wave { freq: 64.0,  amp: 0.2 },
      Wave { freq: 128.0, amp: 0.4 },
    ];

    let mut d = 0.0;
    let mut amp = 0.0;
    for wave in waves.iter() {
      let dd = noise::perlin3(&seed, &[wave.freq*x, wave.freq*y, wave.freq*z]);
      // sharpen
      let dd = dd.signum() * dd.abs().sqrt();
      d = d + dd * wave.amp;
      amp = amp + wave.amp;
    }

    // map to [-1, 1]
    let d = d / amp;
    // map to [0,1]
    let d = (d + 1.0) / 2.0;

    let lerp = 0.3 * d;

    [0.4 + lerp, 0.3 + lerp, 0.1 + lerp]
  }
}
