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

    // Poligono 3
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

    framebuffer.render_to_file("out.bmp");
    println!("Poligono 3 dibujado!");
}
