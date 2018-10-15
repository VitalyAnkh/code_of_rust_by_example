use std::fmt;

#[derive(Debug)]
pub struct P{
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for P {
    // This trait require `fmt` with this exact signature.
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
    write ! (f, "struct{{x:{}, y:{}}}'", self.x, self.y)

    }
}
