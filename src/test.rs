#![feature(let_chains)]

use std::os;

use gb::hardware::cartridge;
use misc::nintendo_logo;

fn main() {
    unsafe { run() }
}

const SCREEN_WIDTH: os::raw::c_int = 800;
const SCREEN_HEIGHT: os::raw::c_int = 450;

unsafe fn run() -> () {
    raylib::SetTraceLogLevel(4); // Make INFO logs shut up

    // Create a window
    raylib::InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, raylib::rl_str!("Game Boy"));
    raylib::SetTargetFPS(60);

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

    let width = raylib::GetScreenWidth();
    let height = raylib::GetScreenHeight();

    // Render the window
    while !(raylib::WindowShouldClose()) {
        raylib::BeginDrawing();
        raylib::ClearBackground(raylib::colors::BLACK);

        for (x, y) in &active_pixels {
            let pixel_size = width / 48;

            raylib::DrawRectangle(
                pixel_size * *x as i32,
                pixel_size * *y as i32,
                pixel_size,
                pixel_size,
                raylib::colors::RED,
            );
        }

        raylib::EndDrawing();
    }

    // Clean up
    raylib::CloseWindow();
}
