use gl;
use yaglw::gl_context::GLContext;
use yaglw::shader::Shader;

fn cnoise() -> &'static str {
  r#"
  //
  //// GLSL textureless classic 3D noise "cnoise",
  ///// with an RSL-style periodic variant "pnoise".
  ///// Author:  Stefan Gustavson (stefan.gustavson@liu.se)
  ///// Version: 2011-10-11
  /////
  ///// Many thanks to Ian McEwan of Ashima Arts for the
  ///// ideas for permutation and gradient selection.
  /////
  ///// Copyright (c) 2011 Stefan Gustavson. All rights reserved.
  ///// Distributed under the MIT license. See LICENSE file.
  ///// https://github.com/ashima/webgl-noise
  /////
  ///
  ///
  vec3 mod289(vec3 x)
  {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
  }

  vec4 mod289(vec4 x)
  {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
  }

  vec4 permute(vec4 x)
  {
    return mod289(((x*34.0)+1.0)*x);
  }

  vec4 taylorInvSqrt(vec4 r)
  {
    return 1.79284291400159 - 0.85373472095314 * r;
  }

  vec3 fade(vec3 t) {
    return t*t*t*(t*(t*6.0-15.0)+10.0);
  }

  // Classic Perlin noise
  float cnoise(vec3 P)
  {
    vec3 Pi0 = floor(P); // Integer part for indexing
    vec3 Pi1 = Pi0 + vec3(1.0); // Integer part + 1
    Pi0 = mod289(Pi0);
    Pi1 = mod289(Pi1);
    vec3 Pf0 = fract(P); // Fractional part for interpolation
    vec3 Pf1 = Pf0 - vec3(1.0); // Fractional part - 1.0
    vec4 ix = vec4(Pi0.x, Pi1.x, Pi0.x, Pi1.x);
    vec4 iy = vec4(Pi0.yy, Pi1.yy);
    vec4 iz0 = Pi0.zzzz;
    vec4 iz1 = Pi1.zzzz;

    vec4 ixy = permute(permute(ix) + iy);
    vec4 ixy0 = permute(ixy + iz0);
    vec4 ixy1 = permute(ixy + iz1);

    vec4 gx0 = ixy0 * (1.0 / 7.0);
    vec4 gy0 = fract(floor(gx0) * (1.0 / 7.0)) - 0.5;
    gx0 = fract(gx0);
    vec4 gz0 = vec4(0.5) - abs(gx0) - abs(gy0);
    vec4 sz0 = step(gz0, vec4(0.0));
    gx0 -= sz0 * (step(0.0, gx0) - 0.5);
    gy0 -= sz0 * (step(0.0, gy0) - 0.5);

    vec4 gx1 = ixy1 * (1.0 / 7.0);
    vec4 gy1 = fract(floor(gx1) * (1.0 / 7.0)) - 0.5;
    gx1 = fract(gx1);
    vec4 gz1 = vec4(0.5) - abs(gx1) - abs(gy1);
    vec4 sz1 = step(gz1, vec4(0.0));
    gx1 -= sz1 * (step(0.0, gx1) - 0.5);
    gy1 -= sz1 * (step(0.0, gy1) - 0.5);

    vec3 g000 = vec3(gx0.x,gy0.x,gz0.x);
    vec3 g100 = vec3(gx0.y,gy0.y,gz0.y);
    vec3 g010 = vec3(gx0.z,gy0.z,gz0.z);
    vec3 g110 = vec3(gx0.w,gy0.w,gz0.w);
    vec3 g001 = vec3(gx1.x,gy1.x,gz1.x);
    vec3 g101 = vec3(gx1.y,gy1.y,gz1.y);
    vec3 g011 = vec3(gx1.z,gy1.z,gz1.z);
    vec3 g111 = vec3(gx1.w,gy1.w,gz1.w);

    vec4 norm0 = taylorInvSqrt(vec4(dot(g000, g000), dot(g010, g010), dot(g100, g100), dot(g110, g110)));
    g000 *= norm0.x;
    g010 *= norm0.y;
    g100 *= norm0.z;
    g110 *= norm0.w;
    vec4 norm1 = taylorInvSqrt(vec4(dot(g001, g001), dot(g011, g011), dot(g101, g101), dot(g111, g111)));
    g001 *= norm1.x;
    g011 *= norm1.y;
    g101 *= norm1.z;
    g111 *= norm1.w;

    float n000 = dot(g000, Pf0);
    float n100 = dot(g100, vec3(Pf1.x, Pf0.yz));
    float n010 = dot(g010, vec3(Pf0.x, Pf1.y, Pf0.z));
    float n110 = dot(g110, vec3(Pf1.xy, Pf0.z));
    float n001 = dot(g001, vec3(Pf0.xy, Pf1.z));
    float n101 = dot(g101, vec3(Pf1.x, Pf0.y, Pf1.z));
    float n011 = dot(g011, vec3(Pf0.x, Pf1.yz));
    float n111 = dot(g111, Pf1);

    vec3 fade_xyz = fade(Pf0);
    vec4 n_z = mix(vec4(n000, n100, n010, n110), vec4(n001, n101, n011, n111), fade_xyz.z);
    vec2 n_yz = mix(n_z.xy, n_z.zw, fade_xyz.y);
    float n_xyz = mix(n_yz.x, n_yz.y, fade_xyz.x); 
    return 2.2 * n_xyz;
  }
  "#
}

fn vertex() -> String {
  r#"
  #version 330 core

  out vec4 world_pos;

  void main() {
    if (gl_VertexID == 0) {
      world_pos = vec4(-1, 1, 0, 1);
    } else if (gl_VertexID == 1) {
      world_pos = vec4(-1, -1, 0, 1);
    } else if (gl_VertexID == 2) {
      world_pos = vec4(1, 1, 0, 1);
    } else {
      world_pos = vec4(1, -1, 0, 1);
    }
    gl_Position = world_pos;
  }
  "#.to_string()
}

fn simple() -> String {
  format!(r#"
  #version 330 core

  in vec4 world_pos;

  layout(location=0) out vec4 frag_color;

  // include cnoise
  {}

  void main() {{
    vec3 xyz = gl_FragCoord.xyz / 100;
    float noise = (cnoise(xyz) + 1) / 2;
    frag_color = vec4(vec3(noise), 1);
  }}
  "#, cnoise())
}

fn dirt() -> String {
  struct Wave {
    freq: f32,
    amp: f32,
  }

  let waves = [
    Wave { freq: 8.0,   amp: 1.0 },
    Wave { freq: 16.0,  amp: 0.6 },
    Wave { freq: 64.0,  amp: 0.2 },
    Wave { freq: 128.0, amp: 0.4 },
  ];

  let mut contents = String::new();
  for wave in waves.iter() {
    contents.push_str(format!(r#"
    {{
      float freq = {};
      float amp = {};

      float dnoise = cnoise(freq * xyz);
      // sharpen
      dnoise = sign(dnoise) * sqrt(abs(dnoise));
      noise += dnoise * amp;
      total_amp += amp;
    }}
    "#, wave.freq, wave.amp).as_str());
  }

  format!(r#"
  #version 330 core

  in vec4 world_pos;

  layout(location=0) out vec4 frag_color;

  // include cnoise
  {}

  void main() {{
    vec3 xyz = gl_FragCoord.xyz / 1000;
    float total_amp = 0.0;
    float noise = 0.0;
    {}
    noise /= total_amp;
    noise = (noise + 1) / 2;
    float lerp = 0.3 * noise;
    frag_color = vec4(0.4 + lerp, 0.3 + lerp, 0.1 + lerp, 1);
  }}
  "#, cnoise(), contents)
}

pub fn make<'a, 'b:'a>(
  gl: &'a GLContext,
) -> Shader<'b> {
  let components = vec!(
    (gl::VERTEX_SHADER, vertex()),
    (gl::FRAGMENT_SHADER, dirt()),
  );

  Shader::new(gl, components.into_iter())
}
