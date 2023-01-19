use std::error::Error;
use engine::management;

#[cfg(not(target_arch = "spirv"))]
fn main() -> Result<(), Box<dyn Error>> {
	management::main()
}
