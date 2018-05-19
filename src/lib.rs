extern crate drawille;

use drawille::{Canvas as BrailleCanvas};
use std::collections::HashMap;
use std::cmp;
use std::default::Default;

pub struct Chart {
    pub width: u32,
    pub height: u32,
    pub xmin: f32,
    pub xmax: f32,
    pub ymin: f32,
    pub ymax: f32,
    pub canvas: BrailleCanvas,
    pub data: HashMap<(u32, u32), u32>,
}

// pub trait Canvas {
//     fn clear() -> ();
//     fn borders() -> ();
//     fn axis() -> ();
//     fn line(x1: u32, y1: u32, x2: u32, y2: u32) -> ();
//     fn display() -> String;
// }

pub trait Plot {
    fn lineplot<'a>(&'a mut self, func: &Fn(f32) -> f32) -> &'a mut Chart;
    // fn scatterplot(&mut self, Vec<(f32, f32)>) -> ();
    // fn histogram(&self, values: Vec<f32>, bins: usize) -> ();
}

impl Default for Chart {
    fn default() -> Self {
        Self::new(120, 60, -10.0, 10.0)
    }
}

impl Chart {
    pub fn new(width: u32, height: u32, xmin: f32, xmax: f32) -> Self {
        Self {
            xmin,
            xmax,
            ymin: 0.0,
            ymax: 0.0,
            width,
            height,
            canvas: BrailleCanvas::new(width, height),
            data: HashMap::new(),
        }
    }

    pub fn borders(&mut self) {
        let w = self.width;
        let h = self.height;

        self.canvas.line(0, 0, 0, h);
        self.canvas.line(0, 0, w, 0);
        self.canvas.line(0, h, w, h);
        self.canvas.line(w, 0, w, h);
    }

    pub fn i_reference(&mut self, i: u32) {
        if i > 0 && i < self.width {
            for j in 0..self.height {
                if j % 3 == 0 {
                    self.canvas.set(i, j);
                }
            }
        }
    }

    pub fn j_reference(&mut self, j: u32) {
        if j > 0 && j < self.height {
            for i in 0..self.width {
                if i % 3 == 0 {
                    self.canvas.set(i, self.height - j);
                }
            }
        }
    }

    pub fn display(&self) {
        let frame = self.canvas.frame();
        let rows = frame.split("\n").into_iter().count();
        for (i, row) in frame.split("\n").into_iter().enumerate() {
            if i == 0 {
                println!("{0} {1:.1}", row, self.ymax);
            } else if i == (rows - 1) {
                println!("{0} {1:.1}", row, self.ymin);
            } else {
                println!("{}", row);
            }
        }

        println!("{0: <width$.1}{1:.1}", self.xmin, self.xmax, width=(self.width as usize) / 2 - 3);
    }
}

impl Plot for Chart {
    fn lineplot<'a>(&'a mut self, func: &Fn(f32) -> f32) -> &'a mut Chart {
        self.borders();

        // calculation of x range
        let xrange = (self.xmax - self.xmin).abs();
        let xstep = xrange / self.width as f32;

        // auto calculation of y range
        let ys: Vec<_> = (0..self.width)
            .into_iter()
            .map(|i| func(self.xmin + (i as f32) * xstep) )
            .collect();

        let mut ymax = *ys.iter().max_by( |x, y| x.partial_cmp(y).unwrap_or(cmp::Ordering::Equal) ).unwrap_or(&0.0);
        let mut ymin = *ys.iter().min_by( |x, y| x.partial_cmp(y).unwrap_or(cmp::Ordering::Equal) ).unwrap_or(&0.0);

        self.ymin = f32::min(self.ymin, ymin);
        self.ymax = f32::max(self.ymax, ymax);

        let margin = (self.ymax - self.ymin) * 0.10;
        ymin = self.ymin - margin;
        ymax = self.ymax + margin;
        let yrange = ymax - ymin;

        // show axis
        let i_center = ((xrange - self.xmax) / xrange) * self.width as f32;
        self.i_reference(i_center as u32);

        let j_center = ((yrange - ymax) / yrange) * self.height as f32;
        self.j_reference(j_center as u32);

        // calculate func and translate (x, y) points into screen coordinates
        let points: Vec<_> = (0..self.width)
            .into_iter()
            .filter_map(|i| {
                let x = self.xmin + (i as f32) * xstep;
                let y = func(x);
                if y.is_normal() {
                    let j = (((y - ymin) / yrange) * self.height as f32) as i32;
                    let j = j.max(0) as u32;
                    let j = j.min(self.width);
                    Some((i, self.height - j))
                } else {
                    None
                }
            }).collect();

        for pair in points.windows(2) {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            self.canvas.line(x1, y1, x2, y2);
        }

        self
    }
}
