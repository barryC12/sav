use crossterm::{
    cursor,
    style::{Color, PrintStyledContent, Stylize},
    ExecutableCommand,
};
use rand::Rng;
use std::f64::consts::PI;
use std::io::Stdout;

pub struct Decoration {
    pub x: u16,
    pub y: u16,
    pub ch: char,
    pub color: Color,
}

pub struct OrbitalCherries {
    pub w: u16,
    pub h: u16,
    pub t: f64,
    pub decorations: Vec<Decoration>,
}

impl OrbitalCherries {
    pub fn new(w: u16, h: u16) -> Self {
        let mut rng = rand::thread_rng();

        let cx = w / 2;
        let cy = h / 2;
        let radius = (w.min(h) / 4) as f64;
        let margin = radius as i32 + 3;

        let mut decorations = Vec::new();

        for _ in 0..100 {
            let (mut sx, mut sy);

            loop {
                sx = rng.gen_range(0..w);
                sy = rng.gen_range(0..h);

                let dx = sx as i32 - cx as i32;
                let dy = sy as i32 - cy as i32;
                let dist = ((dx * dx + dy * dy) as f64).sqrt();

                if dist as i32 > margin {
                    break;
                }
            }

            let chars = ['*', '+', '.', '♥', '•'];
            let ch = chars[rng.gen_range(0..chars.len())];

            let color = Color::Rgb {
                r: rng.gen_range(0..=255),
                g: rng.gen_range(0..=255),
                b: rng.gen_range(0..=255),
            };

            decorations.push(Decoration {
                x: sx,
                y: sy,
                ch,
                color,
            });
        }

        Self {
            w,
            h,
            t: 0.0,
            decorations,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.t += dt * 1.5;
    }

    pub fn draw(&self, stdout: &mut Stdout) {
        let cx = self.w / 2;
        let cy = self.h / 2;
        let radius = (self.w.min(self.h) / 4) as f64;

        let num = 6;

        for i in 0..num {
            let angle = self.t + i as f64 * 2.0 * PI / num as f64;

            let x = cx as i32 + (radius * angle.cos()) as i32;
            let y = cy as i32 + (radius * angle.sin()) as i32;

            let x = x as u16;
            let y = y as u16;

            let _ = stdout.execute(cursor::MoveTo(x, y.saturating_sub(1)));
            let _ = stdout.execute(PrintStyledContent("|".green()));

            let _ = stdout.execute(cursor::MoveTo(x + 1, y.saturating_sub(1)));
            let _ = stdout.execute(PrintStyledContent("/".green()));

            let _ = stdout.execute(cursor::MoveTo(x, y));
            let _ = stdout.execute(PrintStyledContent(
                "o".with(Color::Rgb { r: 220, g: 40, b: 80 }),
            ));
        }

        for d in &self.decorations {
            let _ = stdout.execute(cursor::MoveTo(d.x, d.y));
            let _ = stdout.execute(PrintStyledContent(d.ch.with(d.color)));
        }
    }
}
