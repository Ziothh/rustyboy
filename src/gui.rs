#![feature(let_chains)]

// use misc::nintendo_logo;

// TODO: read this when displaying objects (https://gbdev.io/pandocs/OAM.html#object-priority-and-conflicts)

fn main() {
    todo!("DISABLED");

    // unsafe { run() }
}

// const SCREEN_WIDTH: usize = 800;
//
// unsafe fn run() -> () {
//     raylib::SetTraceLogLevel(4); // Make INFO logs shut up
//
//     let mut gui = misc::gui::GUI::new(SCREEN_WIDTH / misc::gui::GUI::PX_WIDTH);
//
//     let logo = nintendo_logo::Logo::default();
//     let binding = logo.as_pixels();
//     let logo_pixel_rows = binding.iter().flatten();
//     let logo_rows = logo_pixel_rows
//         .size_hint()
//         .1
//         .expect("Failed to get size hint of logo pixels");
//
//     let pixels = logo_pixel_rows
//         .flatten()
//         .map(|lit| {
//             if *lit == true {
//                 Some(raylib::colors::RED)
//             } else {
//                 None
//             }
//         })
//         .collect::<Vec<_>>();
//
//     let (width, height) = gui.buf_size();
//     let logo_width = pixels.len() / logo_rows;
//
//     // let logo_height = pixels.len() % logo_rows;
//     gui.render_pixbuf(
//         (width / 2) - (logo_width / 2),
//         height / 2 - (pixels.len() / logo_width / 2),
//         logo_width,
//         pixels.as_slice(),
//     );
//
//     // Render the window
//     while !(gui.window_should_close()) {
//         gui.draw();
//     }
// }
