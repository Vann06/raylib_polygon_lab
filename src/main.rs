mod framebuffers;
mod line;

use raylib::prelude::*;
use framebuffers::Framebuffer;
use line::{draw_poligon, fill_poligon};

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(
        width,
        height,
        Color { r: 50, g: 50, b: 100, a: 255 },
    );

    framebuffer.set_background_color(Color { r: 50, g: 50, b: 100, a: 255 });
    framebuffer.clear();

    // Polígono 1
    let polygon_1: Vec<Vector2> = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)
    ]
    .into_iter()
    .map(|(x, y)| Vector2::new(x as f32, y as f32))
    .collect();

    framebuffer.set_current_color(Color::YELLOW);
    fill_poligon(&mut framebuffer, &polygon_1);
    framebuffer.set_current_color(Color::WHITE);
    draw_poligon(&mut framebuffer, &polygon_1);

    // Polígono 2
    let polygon_2: Vec<Vector2> = vec![
        (321, 335), (288, 286), (339, 251), (374, 302)
    ]
    .into_iter()
    .map(|(x, y)| Vector2::new(x as f32, y as f32))
    .collect();

    framebuffer.set_current_color(Color::BLUE);
    fill_poligon(&mut framebuffer, &polygon_2);
    framebuffer.set_current_color(Color::WHITE);
    draw_poligon(&mut framebuffer, &polygon_2);

    // Polígono 3
    let polygon_3: Vec<Vector2> = vec![
        (377, 249), (411, 197), (436, 249)
    ]
    .into_iter()
    .map(|(x, y)| Vector2::new(x as f32, y as f32))
    .collect();

    framebuffer.set_current_color(Color::RED);
    fill_poligon(&mut framebuffer, &polygon_3);
    framebuffer.set_current_color(Color::WHITE);
    draw_poligon(&mut framebuffer, &polygon_3);

    // Polígono 4
    let polygon_4: Vec<Vector2> = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37),
        (660, 52), (750, 145), (761, 179), (672, 192), (659, 214), (615, 214),
        (632, 230), (580, 230), (597, 215), (552, 214), (517, 144), (466, 180)
    ]
    .into_iter()
    .map(|(x, y)| Vector2::new(x as f32, y as f32))
    .collect();

    // Polígono 5
    let polygon_5: Vec<Vector2> = vec![
        (682, 175), (708, 120), (735, 148), (739, 170)
    ]
    .into_iter()
    .map(|(x, y)| Vector2::new(x as f32, y as f32))
    .collect();

    // Rellenar polígono 4
    framebuffer.set_current_color(Color::GREEN);
    fill_poligon(&mut framebuffer, &polygon_4);

    // Rellenar el agujero con el color de fondo
    framebuffer.set_current_color(Color { r: 50, g: 50, b: 100, a: 255 });
    fill_poligon(&mut framebuffer, &polygon_5);

    // Bordes blancos
    framebuffer.set_current_color(Color::WHITE);
    draw_poligon(&mut framebuffer, &polygon_4);
    draw_poligon(&mut framebuffer, &polygon_5);

    framebuffer.render_to_file("out.bmp");
    framebuffer.render_to_file("out.png");

    println!("Polígonos 1–4 dibujados con polígono 5 como agujero!");
}
