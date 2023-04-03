// !!!!!!!!!!!!!!!!
// the following file is copied from rust-gpu/examples/shaders/simplest-shader/src/lib.rs
// !!!!!!!!!!!!!!!!

#![cfg_attr(target_arch = "spirv", no_std)]
// HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
#![deny(warnings)]

use spirv_std::glam::{vec4, Vec4};
use spirv_std::spirv;

use engine_shader_derive::shader;

pub fn convert(x: i32) -> i32 {
	x * 2 - 1
}

// pub mod bla {
// 	use super::*;
//
// 	#[shader(fragment)]
// 	pub fn main_fs(output: &mut Vec4) {
// 		*output = vec4(1.0, 0.0, 0.0, 1.0);
// 	}
// }

#[shader(vertex)]
pub fn main_vs(
	#[spirv(vertex_index)] vert_id: i32,
	#[spirv(position, invariant)] out_pos: &mut Vec4,
) {
	*out_pos = vec4(
		(vert_id - 1) as f32,
		convert(vert_id & 1) as f32,
		0.0,
		1.0,
	);
}

pub use other_shader as my_other;
