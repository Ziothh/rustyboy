#![feature(let_chains)]

use gb::hardware::cartridge;
use glium::{Surface, implement_vertex};
use misc::nintendo_logo;
use winit::event::VirtualKeyCode;

fn main() {
    // 1. The **winit::EventLoop** for handling events.
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    // 2. Create a glutin context and glium Display
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Game Boy")
        .build(&event_loop);

    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 0.0, 1.0);
    frame.finish().unwrap();

    let shape = vec![
        Vertex {
            position: [-0.5, -0.5],
        },
        Vertex {
            position: [0.0, 0.5],
        },
        Vertex {
            position: [0.5, -0.25],
        },
    ];

    
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    frame.draw(vertex_buffer, indices, program, uniforms, draw_parameters)

    let logo = nintendo_logo::Logo::default();

    logo.as_pixels()
        .iter()
        .flatten()
        .enumerate()
        .map(|(ri, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, col)| **col == true)
                .map(move |(ci, _)| (ci, ri))
        })
        .flatten()
        .for_each(|(x, y)| {
            ()
            // vec4
        });

    event_loop.run(move |ev, _target, control_flow| match ev {
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            winit::event::WindowEvent::KeyboardInput {
                device_id: _,
                input,
                is_synthetic: _,
            } => {
                if let Some(key) = input.virtual_keycode
                    && key == VirtualKeyCode::Q
                {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                    println!("Q has been pressed. Exiting...");
                }
            }
            _ => (),
        },
        _ => (),
    })
}

#[derive(Clone, Copy)]
struct Vertex {
    // The fields in Vertex are usually there
    // to be passed into the shader file.
    position: [f32; 2],
}

// This line implements the Vertex using a macro inside glium.
// Don't forget to include all of the fields as parameters otherwise
// glium won't pass those into the shader.
implement_vertex!(Vertex, position);
