use std::error::Error;

use shared_library_builder::build_standalone;

use libsdl2_library::latest_libsdl2;

fn main() -> Result<(), Box<dyn Error>> {
    build_standalone(|_| Ok(Box::new(latest_libsdl2())))
}
