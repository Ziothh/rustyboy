pub type PixelValue = Option<raylib::Color>;
pub type PixBuf = [PixelValue; GUI::PX_WIDTH * GUI::PX_HEIGHT];
pub type PixSlice = [PixelValue];

pub struct GUI {
    pixbuf: PixBuf,
    /// Equal to raylib window height / pixelbuf_width
    scale: usize,
}

impl GUI {
    /// Amount of x pixels
    pub const PX_WIDTH: usize = gb::io::graphics::lcd::WINDOW_WIDTH;
    /// Amount of y pixels
    pub const PX_HEIGHT: usize = gb::io::graphics::lcd::WINDOW_HEIGHT;

    /// Creates a raylib window
    pub fn new(scale: usize) -> Self {
        unsafe {
            raylib::InitWindow(
                (Self::PX_WIDTH * scale) as i32,
                (Self::PX_HEIGHT * scale) as i32,
                raylib::rl_str!("Game Boy"),
            );
            raylib::SetTargetFPS(60);
        }

        Self {
            pixbuf: [None; _],
            scale,
        }
    }

    /// Computes the index in the pixel buffer
    #[inline]
    pub fn pixel_index(x: usize, y: usize) -> usize {
        x + (Self::PX_WIDTH * y)
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> PixelValue {
        self.pixbuf[Self::pixel_index(x, y)]
    }

    pub fn render_pixel(&mut self, x: usize, y: usize, color: raylib::Color) -> &mut Self {
        self.pixbuf[Self::pixel_index(x, y)] = Some(color);
        return self;
    }


    #[rustfmt::skip]
    pub fn render_pixbuf(&mut self, x: usize, y: usize, width: usize, pixbuf: &PixSlice) -> &mut Self {
        for (i, color) in pixbuf
            .iter()
            .enumerate()
            .filter_map(|(i, pixel)| pixel.and_then(|color| Some((i, color))))
        {
            let px = i % width;
            let py = i / width;
            self.render_pixel(px + x, py + y, color);
        }

        return self;
    }

    /// (pixels_width, pixels_height)
    pub fn buf_size(&self) -> (usize, usize) {
        (Self::PX_WIDTH, Self::PX_HEIGHT)
    }
    /// (screen_height, screen_width)
    pub fn dimensions(&self) -> (usize, usize) {
        #[rustfmt::skip]
        return unsafe {(
            raylib::GetScreenWidth() as usize / self.scale,
            raylib::GetScreenHeight() as usize / self.scale,
        )};
    }

    /// Draws all pixels to the screen
    pub fn draw(&self) -> &Self {
        unsafe {
            raylib::BeginDrawing();
            raylib::ClearBackground(raylib::colors::BLACK);

            for (i, color) in self
                .pixbuf
                .iter()
                .enumerate()
                .filter_map(|(i, color)| color.and_then(|x| Some((i, x))))
            {
                let x = i % Self::PX_WIDTH;
                let y = i / Self::PX_WIDTH;

                raylib::DrawRectangle(
                    (x * self.scale) as i32,
                    (y * self.scale) as i32,
                    self.scale as i32,
                    self.scale as i32,
                    color,
                );
            }

            raylib::EndDrawing();
        }

        return self;
    }

    /// Asks raylib if the window should close
    pub fn window_should_close(&self) -> bool {
        unsafe { raylib::WindowShouldClose() }
    }
}

impl Drop for GUI {
    fn drop(&mut self) {
        unsafe {
            raylib::CloseWindow();
        }
    }
}
