#[macro_use]
extern crate glium;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use glium::{ DisplayBuild, Surface, glutin };


fn main() {
    let display = glutin::WindowBuilder::new().build_glium().unwrap();

    let p1 = Vertex { position: [-1.0, -1.0] };
    let p2 = Vertex { position: [-1.0, 1.0] };
    let p3 = Vertex { position: [1.0, -1.0] };
    let p4 = Vertex { position: [1.0, -1.0] };
    let p5 = Vertex { position: [1.0, 1.0] };
    let p6 = Vertex { position: [-1.0, 1.0] };
    let data = vec![p1, p2, p3, p4, p5, p6];

    let vertex_buffer = glium::VertexBuffer::new(&display, &data).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = load_shader("src/shader/vert_shader.glslv");
    let fragment_shader_src = load_shader("src/shader/frag_shader.glslf");

    let program = glium::Program::from_source(&display, &vertex_shader_src,
                                              &fragment_shader_src, None).unwrap();

    let mut num_iterations = 100;
    let mut x_shift: f32 = -0.7;
    let mut y_shift: f32 = -0.375;
    let mut zoom: f32 = 1000.0;

    loop {
        let mut target = display.draw();


        target.clear_color(1.0, 1.0, 1.0, 0.0);
        target.draw(&vertex_buffer, &indices, &program,
                    &uniform!{  num_iterations: num_iterations, zoom: zoom,
                                x_shift: x_shift, y_shift: y_shift },
                    &Default::default()).unwrap();
        target.finish().unwrap();

        for e in display.poll_events() {
            use glium::glutin::Event;
            match e {
                Event::Closed => return,
                Event::KeyboardInput(glutin::ElementState::Pressed,_,Some(glutin::VirtualKeyCode::Q)) => return,
                Event::KeyboardInput(glutin::ElementState::Pressed,_,Some(glutin::VirtualKeyCode::Escape)) => return,
                Event::KeyboardInput(glutin::ElementState::Pressed,_,Some(glutin::VirtualKeyCode::A)) => num_iterations = (num_iterations as f32 * 1.1) as i32 + 1,
                Event::KeyboardInput(glutin::ElementState::Pressed,_,Some(glutin::VirtualKeyCode::Y)) => num_iterations = (num_iterations as f32 / 1.1) as i32,
                Event::KeyboardInput(glutin::ElementState::Pressed,_,Some(glutin::VirtualKeyCode::Right)) => x_shift += 10.0/zoom,
                Event::KeyboardInput(glutin::ElementState::Pressed,_,Some(glutin::VirtualKeyCode::Left)) => x_shift -= 10.0/zoom,
                Event::KeyboardInput(glutin::ElementState::Pressed,_,Some(glutin::VirtualKeyCode::Up)) => y_shift += 10.0/zoom,
                Event::KeyboardInput(glutin::ElementState::Pressed,_,Some(glutin::VirtualKeyCode::Down)) => y_shift -= 10.0/zoom,
                Event::KeyboardInput(glutin::ElementState::Pressed,_,Some(glutin::VirtualKeyCode::L)) => zoom *= 1.01,
                Event::KeyboardInput(glutin::ElementState::Pressed,_,Some(glutin::VirtualKeyCode::M)) => zoom /= 1.01,
                _ => ()
            }
        }
    }
}

fn load_shader(file: &str) -> String {
    let path = Path::new(file);
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}

#[derive(Copy, Clone)]
struct Vertex {
        position: [f32; 2],
}

implement_vertex!(Vertex, position);
