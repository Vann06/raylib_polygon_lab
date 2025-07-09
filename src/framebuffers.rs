use raylib::prelude::*;

// Definición de un framebuffer simple que
// almacena un color de fondo y permite dibujar píxeles
pub struct Framebuffer {
    color_buffer: Image,
    width: u32,
    height: u32,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    // Constructor para crear un framebuffer con un color de fondo
    pub fn new(width: u32, height:u32, background_color: Color) -> Self {
    let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);
        Framebuffer {
            color_buffer,
            width,
            height,
            background_color,
            current_color: Color::WHITE,
        }
    }


    //limpiar el buffer
    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width as i32, self.height as i32, self.background_color);
    }

    // colocar un pixel en la pantalla y que no se salga del buffer 
    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    // setear el color del fondo
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
        self.clear();
    }

    // setear el color actual
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    //guardar el buffer en un archivo
    pub fn render_to_file(&self, file_path: &str){
        self.color_buffer.export_image(file_path);
        println!("Framebuffer saved to '{}'", file_path);
    }
}