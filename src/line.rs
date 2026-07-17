use crate::framebuffer::Framebuffer;

pub trait DrawLine {
    fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize);
    fn draw_polygon(&mut self, points: &[(usize, usize)]);
}

impl DrawLine for Framebuffer { //Se usa el algoritmo de bresenham para dibujar una linea de un punto a otro
    fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let dx = (x2 as i32 - x1 as i32).abs();
        let dy = -(y2 as i32 - y1 as i32).abs();

        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        
        let mut err = dx + dy;

        let mut x = x1 as i32;
        let mut y = y1 as i32;

        loop {
            self.point(x as usize, y as usize);

            if x == x2 as i32 && y == y2 as i32 {
                break;
            }

            let e2 = 2 * err;
            
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    fn draw_polygon(&mut self, points: &[(usize, usize)]) {
        if points.len() < 3 {
            return;
        }

        for i in 0..points.len() - 1 {
            let (x1, y1) = points[i];
            let (x2, y2) = points[i + 1];
            self.draw_line(x1, y1, x2, y2);
        }

        let (x_last, y_last) = points[points.len() - 1];
        let (x_first, y_first) = points[0];
        self.draw_line(x_last, y_last, x_first, y_first);
    }
}