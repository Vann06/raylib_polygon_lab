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

    // Poligono 2
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

    framebuffer.render_to_file("out.bmp");
    println!("Poligono 2 dibujado!");
}
