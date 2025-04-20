// using traits for operations
trait MathOperation {
    fn execute(&self, a: f64, b: f64) -> f64;
}

pub struct Addition;

impl MathOperation for Addition {
    fn execute(&self, a: f64, b: f64) -> f64 {
        a + b
    }
}

pub struct Mutiplication;

impl MathOperation for Mutiplication {
    fn execute(&self, a: f64, b: f64) -> f64 {
        a * b
    }
}

// generic function using static dispatch
pub fn perform_operation<T: MathOperation>(op: T, x: f64, y: f64) -> f64 {
    op.execute(x, y)
}
