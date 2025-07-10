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

    // relleno poligono 4 
    framebuffer.set_current_color(Color::GREEN);
    fill_poligon(&mut framebuffer, &polygon_4);

    // rellenar poligono 5
    framebuffer.set_current_color(Color { r: 50, g: 50, b: 100, a: 255 });
    fill_poligon(&mut framebuffer, &polygon_5);

    // bordes blancos
    framebuffer.set_current_color(Color::WHITE);
    draw_poligon(&mut framebuffer, &polygon_4);
    draw_poligon(&mut framebuffer, &polygon_5);

    framebuffer.render_to_file("out.bmp"); 
    println!("Poligono 4 dibujado!");
}
