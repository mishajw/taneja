pub trait Number: Sized {
    fn add(&self, &Self) -> Option<Self>;
    fn subtract(&self, &Self) -> Option<Self>;
    fn multiply(&self, &Self) -> Option<Self>;
    fn divide(&self, &Self) -> Option<Self>;
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
}

