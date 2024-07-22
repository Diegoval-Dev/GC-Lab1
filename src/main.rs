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

    // Framebuffer para los cuatro polígonos
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

    // Polígono 3
    let vertices3 = vec![
        glm::Vec3::new(377.0, 249.0, 0.0),
        glm::Vec3::new(411.0, 197.0, 0.0),
        glm::Vec3::new(436.0, 249.0, 0.0),
    ];

    framebuffer2.set_current_color(Color::new(255, 0, 0));
    fill_polygon(&mut framebuffer2, &vertices3, Color::new(255, 0, 0));

    framebuffer2.set_current_color(Color::new(255, 255, 255));
    draw_polygon(&mut framebuffer2, &vertices3, Color::new(255, 255, 255));

    // Polígono 4
    let vertices4 = vec![
        glm::Vec3::new(413.0, 177.0, 0.0),
        glm::Vec3::new(448.0, 159.0, 0.0),
        glm::Vec3::new(502.0, 88.0, 0.0),
        glm::Vec3::new(553.0, 53.0, 0.0),
        glm::Vec3::new(535.0, 36.0, 0.0),
        glm::Vec3::new(676.0, 37.0, 0.0),
        glm::Vec3::new(660.0, 52.0, 0.0),
        glm::Vec3::new(750.0, 145.0, 0.0),
        glm::Vec3::new(761.0, 179.0, 0.0),
        glm::Vec3::new(672.0, 192.0, 0.0),
        glm::Vec3::new(659.0, 214.0, 0.0),
        glm::Vec3::new(615.0, 214.0, 0.0),
        glm::Vec3::new(632.0, 230.0, 0.0),
        glm::Vec3::new(580.0, 230.0, 0.0),
        glm::Vec3::new(597.0, 215.0, 0.0),
        glm::Vec3::new(552.0, 214.0, 0.0),
        glm::Vec3::new(517.0, 144.0, 0.0),
        glm::Vec3::new(466.0, 180.0, 0.0),
    ];

    // Polígono 5 (Agujero)
    let vertices5 = vec![
        glm::Vec3::new(682.0, 175.0, 0.0),
        glm::Vec3::new(708.0, 120.0, 0.0),
        glm::Vec3::new(735.0, 148.0, 0.0),
        glm::Vec3::new(739.0, 170.0, 0.0),
    ];

    framebuffer2.set_current_color(Color::new(0, 255, 0));
    fill_polygon(&mut framebuffer2, &vertices4, Color::new(0, 255, 0));

    // Crear el agujero en el polígono 4
    framebuffer2.set_current_color(Color::new(0, 0, 0));
    fill_polygon(&mut framebuffer2, &vertices5, Color::new(0, 0, 0));

    framebuffer2.set_current_color(Color::new(255, 255, 255));
    draw_polygon(&mut framebuffer2, &vertices4, Color::new(255, 255, 255));
    draw_polygon(&mut framebuffer2, &vertices5, Color::new(255, 255, 255));

    // Guardar la imagen de los cuatro polígonos
    framebuffer2.render_buffer("output_polygon1_2_3_4.bmp").unwrap();
}
