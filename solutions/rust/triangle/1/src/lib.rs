use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T>
where
    T: Copy + PartialEq + PartialOrd + Add<Output = T> + Default,
{
    pub fn build(sides: [T; 3]) -> Option<Self> {
        let [a, b, c] = sides;

        if a <= T::default() || b <= T::default() || c <= T::default() {
            return None;
        }

        if a + b < c || a + c < b || b + c < a {
            return None;
        }

        Some(Self { a, b, c })
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.a != self.c && self.b != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.a == self.c || self.b == self.c
    }
}
