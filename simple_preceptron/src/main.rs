// SPDX-License-Identifier: Apache-2.0

#[derive(Default, Debug)]
struct Perceptron {
    learning_rate: f64,
    iterations: usize,
    weight: f64,
    bias: f64,
}

impl Perceptron {
    fn new(learning_rate: f64, iterations: usize) -> Self {
        Self {
            learning_rate,
            iterations,
            ..Default::default()
        }
    }

    fn fit(&mut self, data: &[(i32, f64)]) {
        for _ in 0..self.iterations {
            for (x, y) in data.iter() {
                let update = self.learning_rate * (y - self.predict(*x));
                self.weight += update * (*x as f64);
                self.bias += update;
            }
        }
    }

    fn predict(&self, x: i32) -> f64 {
        if self.weight * x as f64 + self.bias > 0.0 {
            1.0
        } else {
            0.0
        }
    }
}

fn main() {
    let data: Vec<(i32, f64)> = vec![
        (1, 1.0),
        (2, 1.0),
        (3, 1.0),
        (10, 1.0),
        (20, 1.0),
        (-2, 0.0),
        (-10, 0.0),
        (-100, 0.0),
        (-5, 0.0),
        (-20, 0.0),
    ];
    let mut model = Perceptron::new(0.01, 10);
    model.fit(&data);

    for i in [30, 40, -20, -60] {
        println!("{i}, predict {}", model.predict(i));
    }

    println!("HAHA {:?}", model);
}
