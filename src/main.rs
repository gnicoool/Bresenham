mod framebuffer;
mod line;
mod bmp;

use crate::framebuffer::Framebuffer;
use crate::line::DrawLine;
use crate::bmp::WriteBmp;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    framebuffer.set_current_color(0xFF00FF);

    let J = [
        (200,100),(600,100),(600,150),(480,150),(480,450),(160,450),(160,300),(260,300),(260,380),(380,380),(350,150),(200,150)
    ];

    framebuffer.draw_polygon(&J);

    let _ = framebuffer.render_buffer("output.bmp");

    println!("Framebuffer rendered to output.bmp");
} 
