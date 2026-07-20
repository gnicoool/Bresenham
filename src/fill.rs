use crate::framebuffer::Framebuffer;

pub trait Fill {
    fn scanline_fill_individual(&mut self, points: &[(usize, usize)], color: u32);
    fn scanline_fill(&mut self, polygons: &[&[(usize, usize)]], color: u32);
}

impl Fill for Framebuffer {
    fn scanline_fill_individual(&mut self, points: &[(usize, usize)], color: u32) {
        self.scanline_fill(&[points], color);
    }

    fn scanline_fill(&mut self, polygons: &[&[(usize, usize)]], color: u32) {
        if polygons.is_empty() {
            return;
        }

        self.set_current_color(color);

        let mut y_min = usize::MAX;
        let mut y_max = usize::MIN;

        for polygon in polygons {
            if polygon.len() < 3 { continue; }
            for p in *polygon {
                if p.1 < y_min { y_min = p.1; }
                if p.1 > y_max { y_max = p.1; }
            }
        }

        if y_min > y_max {
            return;
        }

        y_min = y_min.clamp(0, self.height - 1);
        y_max = y_max.clamp(0, self.height - 1);

        for y in y_min..=y_max {
            let mut intersections = Vec::new();
            let curr_y = y as f32;

            for polygon in polygons {
                if polygon.len() < 3 { continue; }

                for i in 0..polygon.len() {
                    let p1 = polygon[i];
                    let p2 = polygon[(i + 1) % polygon.len()];

                    let (x1, y1, x2, y2) = if p1.1 < p2.1 {
                        (p1.0 as f32, p1.1 as f32, p2.0 as f32, p2.1 as f32)
                    } else {
                        (p2.0 as f32, p2.1 as f32, p1.0 as f32, p1.1 as f32)
                    };

                    if curr_y >= y1 && curr_y < y2 {
                        let x_intersect = x1 + (curr_y - y1) * (x2 - x1) / (y2 - y1);
                        intersections.push(x_intersect);
                    }
                }
            }

            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

            for pair in intersections.chunks_exact(2) {
                let x_start = pair[0].ceil().max(0.0) as usize;
                let x_end = (pair[1].floor().max(0.0) as usize).min(self.width - 1);

                if x_start > x_end {
                    continue;
                }

                for x in x_start..=x_end {
                    self.point(x, y);
                }
            }
        }
    }
}