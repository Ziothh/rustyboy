use glium::Surface;

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

    event_loop.run(move |ev, _target, control_flow| match ev {
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            _ => (),
        },
        _ => (),
    })
}
