

struct PerlinInput {
    size: vec2<f32>,
    frequency: vec2<f32>,
    uv_offset: vec2<f32>,
    ty: u32,
    octaves: u32,
};

@group(0) @binding(0)
var<uniform> parameters: PerlinInput;

@group(0) @binding(1)
var texture: texture_storage_2d<rgba8unorm, write>;

fn permute4(x: vec4<f32>) -> vec4<f32> { return ((x * 34. + 1.) * x) % vec4<f32>(289.); }
fn fade2(t: vec2<f32>) -> vec2<f32> { return t * t * t * (t * (t * 6. - 15.) + 10.); }

// copied from https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39 
// output value seems to be in [-1; 1] range
fn perlinNoise2(P: vec2<f32>) -> f32 {
    var Pi: vec4<f32> = floor(P.xyxy) + vec4<f32>(0., 0., 1., 1.);
    let Pf = fract(P.xyxy) - vec4<f32>(0., 0., 1., 1.);
    Pi = Pi % vec4<f32>(289.); // To avoid truncation effects in permutation
    let ix = Pi.xzxz;
    let iy = Pi.yyww;
    let fx = Pf.xzxz;
    let fy = Pf.yyww;
    let i = permute4(permute4(ix) + iy);
    var gx: vec4<f32> = 2. * fract(i * 0.0243902439) - 1.; // 1/41 = 0.024...
    let gy = abs(gx) - 0.5;
    let tx = floor(gx + 0.5);
    gx = gx - tx;
    var g00: vec2<f32> = vec2<f32>(gx.x, gy.x);
    var g10: vec2<f32> = vec2<f32>(gx.y, gy.y);
    var g01: vec2<f32> = vec2<f32>(gx.z, gy.z);
    var g11: vec2<f32> = vec2<f32>(gx.w, gy.w);
    let norm = 1.79284291400159 - 0.85373472095314 *
        vec4<f32>(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));
    g00 = g00 * norm.x;
    g01 = g01 * norm.y;
    g10 = g10 * norm.z;
    g11 = g11 * norm.w;
    let n00 = dot(g00, vec2<f32>(fx.x, fy.x));
    let n10 = dot(g10, vec2<f32>(fx.y, fy.y));
    let n01 = dot(g01, vec2<f32>(fx.z, fy.z));
    let n11 = dot(g11, vec2<f32>(fx.w, fy.w));
    let fade_xy = fade2(Pf.xy);
    let n_x = mix(vec2<f32>(n00, n01), vec2<f32>(n10, n11), vec2<f32>(fade_xy.x));
    let n_xy = mix(n_x.x, n_x.y, fade_xy.y);
    return 2.3 * n_xy;
}

fn worley_hash(p: vec2<f32>) -> vec2<f32>
{
 	return fract(cos(p*mat2x2<f32>(-64.2,71.3,81.4,-29.8))*8321.3); 
}

fn worley_noise(p: vec2<f32>)-> f32
{
    var Dist = 1.;
    let I = floor(p);
    let F = fract(p);
    
    for(var X = -1;X<=1;X++)
  {
    for(var Y = -1;Y<=1;Y++)
    {
        let shift = vec2<f32>(f32(X), f32(Y));
        let D = distance(worley_hash(I+shift)+shift,F);
        Dist = min(Dist,D);
    }
  }
    return Dist;
}

fn minus_worley(p: vec2<f32>) -> f32
{
  return 1.0 - worley_noise(p);
}

fn cauliflower(uv: vec2<f32>) -> f32 {
    return (1.0 + perlinNoise2(uv * 8.0)) * (1.0 + minus_worley(uv *  8.0) + 0.5 * minus_worley(uv * 16.0) + 0.25 * minus_worley(uv * 32.0)) * 0.25;
}

fn noise(uv: vec2<f32>)-> f32 {
  // values same as NoiseType from noise.rs
    switch parameters.ty {
      case 1u: {
        return perlinNoise2(uv);
      }
      case 2u: {
        return (worley_noise(uv) - 0.5) * 2.0;
      }
      case 3u: {
        return (cauliflower(uv) - 0.5) * 2.0;
      }
      default: {
        return perlinNoise2(uv);
      }
    }
}

@compute
@workgroup_size(1)
fn main(
    @builtin(global_invocation_id) id: vec3<u32>
) {
    let uv = (vec2<f32>(id.xy) / parameters.size) + parameters.uv_offset; // 0-1

    let G = 0.5; // could be 0.5-1.0 as explained here:https://iquilezles.org/articles/fbm/ 
    var f = parameters.frequency;
    var amplitude = 1.0;
    var total = 0.0;

    for (var i: u32 = 0u; i<=parameters.octaves; i++) {
        total += amplitude * noise(uv * f);
        f *= 2.0;
        amplitude *= G;
    }

    let noise_value = 0.5 + 0.5 * total; // not ideal - values can be clamped at min and max
    let color = vec4(noise_value, noise_value, noise_value, 1.0);
    textureStore(texture, vec2(i32(id.x), i32(id.y)), color);
}
