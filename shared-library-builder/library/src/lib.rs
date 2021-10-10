use shared_library_builder::{CMakeLibrary, CompiledLibraryName, GitLocation, LibraryLocation};

pub fn libsdl2(binary_version: Option<impl Into<String>>) -> CMakeLibrary {
    CMakeLibrary::new(
        "SDL2",
        LibraryLocation::Git(GitLocation::github("libsdl-org", "SDL").tag("release-2.0.14")),
    )
    .compiled_name(CompiledLibraryName::Matching("SDL2".to_string()))
    .with_release_location(binary_version.map(|version| {
        LibraryLocation::Git(GitLocation::github("feenkcom", "libsdl2").tag(version))
    }))
}
