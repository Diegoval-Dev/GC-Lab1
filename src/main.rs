use nalgebra_glm as glm;
use crate::framebuffer::Framebuffer;
use crate::line_impl::draw_line;
use crate::polygon_impl::{draw_polygon, fill_polygon};
use crate::color::Color;

mod framebuffer;
mod line_impl;
mod polygon_impl;
mod color;
mod bmp;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Polígono 1
    let vertices1 = vec![
        glm::Vec3::new(165.0, 380.0, 0.0),
        glm::Vec3::new(185.0, 360.0, 0.0),
        glm::Vec3::new(180.0, 330.0, 0.0),
        glm::Vec3::new(207.0, 345.0, 0.0),
        glm::Vec3::new(233.0, 330.0, 0.0),
        glm::Vec3::new(230.0, 360.0, 0.0),
        glm::Vec3::new(250.0, 380.0, 0.0),
        glm::Vec3::new(220.0, 385.0, 0.0),
        glm::Vec3::new(205.0, 410.0, 0.0),
        glm::Vec3::new(193.0, 383.0, 0.0),
    ];

    framebuffer.set_current_color(Color::new(255, 255, 0));
    fill_polygon(&mut framebuffer, &vertices1, Color::new(255, 255, 0));

    framebuffer.set_current_color(Color::new(255, 255, 255));
    draw_polygon(&mut framebuffer, &vertices1, Color::new(255, 255, 255));

    framebuffer.render_buffer("poligono1.bmp").unwrap();
}
