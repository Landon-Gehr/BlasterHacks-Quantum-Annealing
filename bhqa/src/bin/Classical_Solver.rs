use std::fs::File;
use std::io::Write;
use plotters::prelude::*;
use sprs::{CsMat, TriMat};

use std::f64::consts::PI;

pub struct Solver {
    pub nx: usize,
    pub ny: usize,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub dx: f64,
    pub dy: f64,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub a: CsMat<f64>,
    pub b: Vec<f64>,
    pub soln: Vec<f64>,
}

impl Solver {
    pub fn new(
        nx: usize,
        ny: usize,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
    ) -> Self {
        assert!(nx >= 2, "nx must be at least 2");
        assert!(ny >= 2, "ny must be at least 2");
        assert!(x_max > x_min, "x_max must be greater than x_min");
        assert!(y_max > y_min, "y_max must be greater than y_min");

        let dx = (x_max - x_min) / ((nx - 1) as f64);
        let dy = (y_max - y_min) / ((ny - 1) as f64);
        let n = nx * ny;

        let mut solver = Self {
            nx,
            ny,
            x_min,
            x_max,
            y_min,
            y_max,
            dx,
            dy,
            x: Vec::new(),
            y: Vec::new(),
            a: CsMat::zero((n, n)),
            b: vec![0.0; n],
            soln: vec![0.0; n],
        };

        solver.build_mesh();
        solver
    }

    pub fn build_qubo(&mut self) {
        
    }

    pub fn build_mesh(&mut self) {
        self.x = vec![0.0; self.nx];
        self.y = vec![0.0; self.ny];

        for i in 0..self.nx {
            self.x[i] = self.x_min + (i as f64) * self.dx;
        }

        for j in 0..self.ny {
            self.y[j] = self.y_min + (j as f64) * self.dy;
        }
    }

    pub fn solve(&mut self, max_iter: usize, tol: f64) {
        let n = self.npoints();
        let mut new_u = self.soln.clone();

        for _ in 0..max_iter {
            let mut max_diff = 0.0_f64;

            for row in 0..n {
                let row_view = self.a.outer_view(row).expect("Missing sparse row");

                let mut diag = 0.0;
                let mut sigma = 0.0;

                for (col, val) in row_view.iter() {
                    if col == row {
                        diag = *val;
                    } else {
                        sigma += val * self.soln[col];
                    }
                }

                if diag == 0.0 {
                    panic!("Zero diagonal at row {}", row);
                }

                new_u[row] = (self.b[row] - sigma) / diag;
                max_diff = max_diff.max((new_u[row] - self.soln[row]).abs());
            }

            self.soln.clone_from_slice(&new_u);

            if max_diff < tol {
                break;
            }
        }
    }   

    pub fn plot(&self, filename: &str) {
        let root = BitMapBackend::new(filename, (800, 800)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption("PDE Solution", "sans-serif, 20")
        .build_cartesian_2d(
            self.x_min..self.x_max,
            self.y_min..self.y_max
        )
        .unwrap();

        chart.configure_mesh().draw().unwrap();

        let dx = self.dx;
        let dy = self.dy;

        for j in 0..self.ny {
            for i in 0..self.nx {
                let idx = self.idx(i, j);
                let u = self.soln[idx];

                let color = HSLColor(240.0 / 360.0 - u * 0.2, 1.0, 0.5);

                chart.draw_series(std::iter::once(Rectangle::new(
                    [
                        (self.x[i], self.y[j]),
                        (self.x[i] + dx, self.y[j] + dy),
                    ],
                    color.filled(),
                ))).unwrap();
            }
        }
    }

    pub fn idx(&self, i: usize, j: usize) -> usize {
        j * self.nx + i
    }

    pub fn npoints(&self) -> usize {
        self.nx * self.ny
    }

    pub fn build<F, G>(&mut self, f: F, g: G)
    where
        F: Fn(f64, f64) -> f64,
        G: Fn(f64, f64) -> f64,
    {
        let n = self.npoints();
        let mut tri = TriMat::<f64>::new((n, n));
        self.b = vec![0.0; n];

        let dx2 = self.dx * self.dx;
        let dy2 = self.dy * self.dy;

        for j in 0..self.ny {
            for i in 0..self.nx {
                let row = self.idx(i, j);
                let x = self.x[i];
                let y = self.y[j];

                let is_boundary =
                    i == 0 || i == self.nx - 1 || j == 0 || j == self.ny - 1;

                if is_boundary {
                    tri.add_triplet(row, row, 1.0);
                    self.b[row] = g(x, y);
                    self.soln[row] = g(x, y);
                } else {
                    let left = self.idx(i - 1, j);
                    let right = self.idx(i + 1, j);
                    let down = self.idx(i, j - 1);
                    let up = self.idx(i, j + 1);

                    tri.add_triplet(row, left, 1.0 / dx2);
                    tri.add_triplet(row, right, 1.0 / dx2);
                    tri.add_triplet(row, down, 1.0 / dy2);
                    tri.add_triplet(row, up, 1.0 / dy2);
                    tri.add_triplet(row, row, -2.0 / dx2 - 2.0 / dy2);
                    
                    self.b[row] = f(x, y);
                }
            }
        }

        self.a = tri.to_csr()
    }

    pub fn write(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file.");

        for j in 0..self.ny {
            for i in 0..self.nx {
                let idx = self.idx(i, j);
                let x = self.x[i];
                let y = self.y[j];
                let u = self.soln[idx];

                writeln!(file, "{} {} {}", x, y, u).expect("Write failed.");
            }
        }
    }
}

fn main() {
    let filename: &str = "solution.dat";
    let mut solver = Solver::new(64, 64, 0.0, 4.0 * PI, 0.0, 4.0 * PI);

    println!("dx = {}", solver.dx);
    println!("dy = {}", solver.dy);
    println!("x[0] = {}", solver.x[0]);
    println!("x[last] = {}", solver.x[solver.nx - 1]);
    println!("y[0] = {}", solver.y[0]);
    println!("y[last] = {}", solver.y[solver.ny - 1]);

    solver.build(
        |x, y| -2.0 * x.cos() * y.cos(),
        |x, y| x.cos() * y.cos(),
    );

    solver.solve(5000,1e-9);
    solver.write(filename);

    solver.plot("Solution.png");

}