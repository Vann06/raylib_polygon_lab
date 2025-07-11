use raylib::prelude::*;
use crate::framebuffers::Framebuffer;

pub fn line(
    framebuffer: &mut Framebuffer,
    start: Vector2,
    end: Vector2,
) {
    // Algoritmo de Bresenham
    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        framebuffer.set_pixel(x0 as u32, y0 as u32);

        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}


pub fn draw_poligon(framebuffer: &mut Framebuffer, points: &[Vector2]){
    if points.len() < 2 {
        return; 
    }

    for i in 0..points.len(){
        let start = points[i];
        let end = if i == points.len() - 1 {
            // conectar primer punto
            points[0] 
        } else {
            // conectar siguiente punto
            points[i + 1] 
        };
        line(framebuffer, start, end);
    }
}

// Rellenar un poligono
pub fn fill_poligon(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    let mut y_min = f32::MAX;
    let mut y_max = f32::MIN;

    // rango de filas a escanear
    for p in points {
        if p.y < y_min {
            y_min = p.y;
        }
        if p.y > y_max {
            y_max = p.y;
        }
    }
    for y in y_min as i32..=y_max as i32 {
    let mut intersections: Vec<f32> = Vec::new();

    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];

        if (a.y < y as f32 && b.y >= y as f32) || (b.y < y as f32 && a.y >= y as f32) {
            let x = a.x + (y as f32 - a.y) * (b.x - a.x) / (b.y - a.y);
            intersections.push(x);
        }
    }

    intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut i = 0;
    while i + 1 < intersections.len() {
        let x_start = intersections[i];
        let x_end = intersections[i + 1];

        for x in x_start as i32..=x_end as i32 {
            framebuffer.set_pixel(x as u32, y as u32);
        }
        i += 2;
    }
}

}