#![feature(let_chains)]

use std::os;

use gb::hardware::cartridge;
use misc::nintendo_logo;

fn main() {
    unsafe { run() }
}

const WIDTH: os::raw::c_int = 800;
const HEIGHT: os::raw::c_int = 450;

unsafe fn run() -> () {
    raylib::SetTraceLogLevel(4);

    // Create a window
    raylib::InitWindow(WIDTH, HEIGHT, raylib::rl_str!("Game Boy"));

    let logo = nintendo_logo::Logo::default();

    let active_pixels: Vec<_> = logo
        .as_pixels()
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
        .collect();

    // Render the window
    while !(raylib::WindowShouldClose()) {
        raylib::BeginDrawing();
        raylib::ClearBackground(raylib::colors::BLACK);
        let width = raylib::GetScreenWidth();
        let height = raylib::GetScreenHeight();

        for (x, y) in &active_pixels {
            println!("\n\n");
            dbg!(x, y);
            dbg!(width / 48 * *x as i32);
            dbg!(height / 48 * *y as i32);

            raylib::DrawRectangle(
                width / 48 * *x as i32,
                height / 48 * *y as i32,
                width / 48,
                width / 48,
                raylib::colors::RED,
            );
        }

        raylib::EndDrawing();
    }

    // Clean up
    raylib::CloseWindow();
}
