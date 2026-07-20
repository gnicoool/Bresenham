mod framebuffer;
mod line;
mod fill;
mod bmp;

use crate::framebuffer::Framebuffer;
use crate::line::DrawLine;
use crate::fill::Fill;
use crate::bmp::WriteBmp;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    let poligono1 = [
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)
    ];

    let poligono2 = [
        (321, 335), (288, 286), (339, 251), (374, 302)
    ];

    let poligono3 = [
        (377, 249), (411, 197), (436, 249)
    ];

    let poligono4 = [
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180)
    ];

    let poligono5 = [
        (682, 175), (708, 120), (735, 148), (739, 170)
    ];

    let taza = [
        &poligono4[..],
        &poligono5[..]
    ];
    framebuffer.scanline_fill(&taza, 0xFF0000);

    framebuffer.scanline_fill_individual(&poligono1, 0xFF00FF);
    framebuffer.scanline_fill_individual(&poligono2, 0x00FFFF);
    framebuffer.scanline_fill_individual(&poligono3, 0xFFFF00);

    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.draw_polygon(&poligono1);
    framebuffer.draw_polygon(&poligono2);
    framebuffer.draw_polygon(&poligono3);
    framebuffer.draw_polygon(&poligono4);
    framebuffer.draw_polygon(&poligono5);

    let _ = framebuffer.render_buffer("output.png");

    println!("Framebuffer rendered to output.png");
} 
