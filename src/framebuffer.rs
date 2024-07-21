use crate::color::Color;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<Color>,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let background_color = Color::new(0, 0, 0); // Default background color: black
        let current_color = Color::new(255, 255, 255); // Default current color: white
        let buffer = vec![background_color; width * height];
        Framebuffer {
            width,
            height,
            buffer,
            background_color,
            current_color,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            let index = y as usize * self.width + x as usize;
            self.buffer[index] = self.current_color;
        }
    }

    pub fn get_point_color(&self, x: isize, y: isize) -> Option<Color> {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            let index = y as usize * self.width + x as usize;
            Some(self.buffer[index])
        } else {
            None
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_buffer(&self, file_path: &str) -> std::io::Result<()> {
        crate::bmp::write_bmp_file(file_path, self.get_buffer(), self.width, self.height)
    }

    pub fn get_buffer(&self) -> &[Color] {
        &self.buffer
    }
}
