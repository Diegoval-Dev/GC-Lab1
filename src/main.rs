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
    
    // Framebuffer para el Polígono 1
    let mut framebuffer1 = Framebuffer::new(width, height);

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

    framebuffer1.set_current_color(Color::new(255, 255, 0));
    fill_polygon(&mut framebuffer1, &vertices1, Color::new(255, 255, 0));

    framebuffer1.set_current_color(Color::new(255, 255, 255));
    draw_polygon(&mut framebuffer1, &vertices1, Color::new(255, 255, 255));

    // Guardar la imagen del Polígono 1
    framebuffer1.render_buffer("output_polygon1.bmp").unwrap();

    // Framebuffer para ambos polígonos
    let mut framebuffer2 = Framebuffer::new(width, height);

    // Polígono 1
    framebuffer2.set_current_color(Color::new(255, 255, 0));
    fill_polygon(&mut framebuffer2, &vertices1, Color::new(255, 255, 0));

    framebuffer2.set_current_color(Color::new(255, 255, 255));
    draw_polygon(&mut framebuffer2, &vertices1, Color::new(255, 255, 255));

    // Polígono 2
    let vertices2 = vec![
        glm::Vec3::new(321.0, 335.0, 0.0),
        glm::Vec3::new(288.0, 286.0, 0.0),
        glm::Vec3::new(339.0, 251.0, 0.0),
        glm::Vec3::new(374.0, 302.0, 0.0),
    ];

    framebuffer2.set_current_color(Color::new(0, 0, 255));
    fill_polygon(&mut framebuffer2, &vertices2, Color::new(0, 0, 255));

    framebuffer2.set_current_color(Color::new(255, 255, 255));
    draw_polygon(&mut framebuffer2, &vertices2, Color::new(255, 255, 255));

    framebuffer2.render_buffer("output_polygon1_and_2.bmp").unwrap();
}
