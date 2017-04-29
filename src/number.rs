pub trait Number: Sized {
    fn add(&self, &Self) -> Option<Self>;
    fn subtract(&self, &Self) -> Option<Self>;
    fn multiply(&self, &Self) -> Option<Self>;
    fn divide(&self, &Self) -> Option<Self>;
    fn power(&self, &Self) -> Option<Self>;
    fn concat(&self, &Self) -> Option<Self>;
    fn is_integer(&self) -> bool;
}

impl Number for i32 {
    fn add(&self, other: &Self) -> Option<Self> { Some(self + other) }
    fn subtract(&self, other: &Self) -> Option<Self> { Some(self - other) }
    fn multiply(&self, other: &Self) -> Option<Self> { Some(self * other) }
    fn divide(&self, other: &Self) -> Option<Self> {
        if *other == 0 {
            None
        } else {
            Some(self / other)
        }
    }
    fn power(&self, other: &Self) -> Option<Self> { Some(self.pow(*other as u32)) }
    fn concat(&self, other: &Self) -> Option<Self> {
        Some(self * 10i32.pow((*other as f64).log(10f64).ceil() as u32) + other)
    }
    fn is_integer(&self) -> bool { true }
}

impl Number for f64 {
    fn add(&self, other: &Self) -> Option<Self> { Some(self + other) }
    fn subtract(&self, other: &Self) -> Option<Self> { Some(self - other) }
    fn multiply(&self, other: &Self) -> Option<Self> { Some(self * other) }
    fn divide(&self, other: &Self) -> Option<Self> {
        if *other == 0f64 {
            None
        } else {
            Some(self / other)
        }
    }
    fn power(&self, other: &Self) -> Option<Self> { Some(self.powf(*other)) }
    fn concat(&self, other: &Self) -> Option<Self> {
        Some(self * 10f64.powi((*other as f64).log(10f64).ceil() as i32) + other)
    }
    fn is_integer(&self) -> bool { *self % 1f64 == 0f64 }
}

