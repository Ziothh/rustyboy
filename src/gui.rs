#![feature(let_chains)]

use gb::hardware::bus;
use misc::{nintendo_logo, ROM};

// TODO: read this when displaying objects (https://gbdev.io/pandocs/OAM.html#object-priority-and-conflicts)

fn main() {
    unsafe { run() }
}

const SCREEN_WIDTH: usize = 800;

unsafe fn run() -> () {
    raylib::SetTraceLogLevel(4); // Make INFO logs shut up

    let boot_rom = ROM::try_new("./roms/gb/bootix.bin")
        .expect("Failed to find bootrom")
        .read()
        .unwrap();
    let rom = ROM::try_new("./roms/gb/test.gb")
        .expect("Failed to find rom")
        .read()
        .unwrap();

    let logo = nintendo_logo::Logo::from_slice(&rom[0x0104..=0x0133]);
    assert_eq!(logo.bytes(), nintendo_logo::Logo::NINTENDO_LOGO_BYTES);

    let mut gui = misc::gui::GUI::new(SCREEN_WIDTH / misc::gui::GUI::PX_WIDTH);

    let mut bus = bus::Interface::new();
    bus.load(&rom);
    bus.load(&boot_rom);

    
    

    // Render the window
    while !(gui.window_should_close()) {
        gui.draw();

        std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
    }
}

// NOT USED
#[allow(dead_code)]
fn draw_logo(gui: &mut misc::gui::GUI) {
    let logo = nintendo_logo::Logo::default();
    let binding = logo.as_pixels();
    let logo_pixel_rows = binding.iter().flatten();
    let logo_rows = logo_pixel_rows
        .size_hint()
        .1
        .expect("Failed to get size hint of logo pixels");

    let pixels = logo_pixel_rows
        .flatten()
        .map(|lit| {
            if *lit == true {
                Some(raylib::colors::RED)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let (width, height) = gui.buf_size();
    let logo_width = pixels.len() / logo_rows;

    // let logo_height = pixels.len() % logo_rows;
    gui.render_pixbuf(
        (width / 2) - (logo_width / 2),
        height / 2 - (pixels.len() / logo_width / 2),
        logo_width,
        pixels.as_slice(),
    );
}
