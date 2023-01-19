use std::env;
use std::error::Error;
use std::path::PathBuf;

use spirv_builder::{MetadataPrintout, SpirvBuilder};

fn main() -> Result<(), Box<dyn Error>> {
	let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
	if target_arch == "spirv" {
		println!("exiting successfully to prevent recursion: target arch is spir-v");
		return Ok(());
	}

	let manifest_dir = env!("CARGO_MANIFEST_DIR");
	let crate_path = [manifest_dir, "..", "shader"]
		.iter()
		.copied()
		.collect::<PathBuf>();
	let result = SpirvBuilder::new(crate_path, "spirv-unknown-vulkan1.1")
		.print_metadata(MetadataPrintout::Full)
		.build()?;

	let module_path = result.module.unwrap_single();
	println!("module_path: {}", module_path.to_str().unwrap());
	println!("entry points: {:?}", result.entry_points);
	Ok(())
}
