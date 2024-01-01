#![feature(let_chains)]

use gb::hardware::cartridge;
use glium::Surface;
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

    let logo = nintendo_logo::Logo::default();


    logo.as_pixels().iter().flatten().enumerate().for_each(|(i, row)| {

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
