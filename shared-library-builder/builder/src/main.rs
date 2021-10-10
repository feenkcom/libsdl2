use sdl2_library::sdl2;
use shared_library_builder::{Library, LibraryCompilationContext};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let version: Option<String> = None;
    let library = sdl2(version);

    let context = LibraryCompilationContext::new_release("target");
    let compiled_library = library.compile(&context)?;
    println!("Compiled {}", compiled_library.display());
    Ok(())
}
