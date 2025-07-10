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

    // Poligono 1
    let polygon_1: Vec<Vector2> = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)
    ]
    .into_iter()
    .map(|(x, y)| Vector2::new(x as f32, y as f32))
    .collect();

    framebuffer.set_current_color(Color::YELLOW);
    fill_poligon(&mut framebuffer, &polygon_1);
    framebuffer.set_current_color(Color::WHITE);
    draw_poligon(&mut framebuffer, &polygon_1);
    


    framebuffer.render_to_file("out.png");
    println!("Poligono 1 dibujado!");
}
