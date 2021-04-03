use glium::backend::Facade;
use std::fs;

/// Loads the vertex & fragment shader from 2 files.
///
/// # Panics
///
/// Panics if on or both files don't exist.
pub fn load_program<F: Facade>(
    display: &F,
    vertex_file: &str,
    fragment_file: &str,
) -> glium::Program {
    let path = "resources/shader/";
    let vertex_shader = read_to_string(path, vertex_file, "Could not load vertex shader");
    let fragment_shader = read_to_string(path, fragment_file, "Could not load fragment shader");

    glium::Program::from_source(display, &vertex_shader, &fragment_shader, None).unwrap()
}

fn read_to_string(path: &str, file: &str, error_msg: &str) -> String {
    fs::read_to_string([path, file].concat()).expect(error_msg)
}
