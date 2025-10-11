#[derive(Debug)]
pub struct CustomSet<T>(Vec<T>);

impl<T: Clone + PartialEq> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        let mut res = Self(vec![]);
        for x in _input {
            res.add(x.clone());
        }
        res
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.0.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        if !self.contains(&_element) {
            self.0.push(_element);
        }
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.0.iter().all(|x| _other.contains(x))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        !self.0.iter().any(|x| _other.contains(x))
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        let vec = self
            .0
            .iter()
            .filter(|x| _other.contains(x))
            .cloned()
            .collect();
        Self(vec)
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        let vec = self
            .0
            .iter()
            .filter(|x| !_other.contains(x))
            .cloned()
            .collect();
        Self(vec)
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        let vec: Vec<T> = self.0.iter().chain(_other.0.iter()).cloned().collect();
        Self::new(&vec)
    }
}

impl<T: Clone + PartialEq> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is_subset(other) && other.is_subset(self)
    }
}
