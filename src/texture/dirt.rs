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

    let perlin = |f, x, y, z| {
      noise::perlin3(&seed, &[f*x, f*y, f*z])
    };

    let waves = [
      Wave { freq: 8.0,   amp: 1.0 },
      Wave { freq: 16.0,  amp: 0.5 },
      Wave { freq: 128.0, amp: 0.2 },
    ];

    let mut d = 0.0;
    let mut amp = 0.0;
    for wave in waves.iter() {
      d = d + perlin(wave.freq, x, y, z) * wave.amp;
      amp = amp + wave.amp;
    }

    // Map to [0,1]
    let d = (d + amp) / (2.0 * amp);

    [0.4 + d*0.3, 0.3 + d*0.3, 0.1 + d*0.3]
  }
}
