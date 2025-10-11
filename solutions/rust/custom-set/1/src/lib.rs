#[derive(Debug)]
pub struct CustomSet<T>(Vec<T>);

impl<T: PartialEq + Clone> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        let mut vec = Vec::new();
        for x in _input {
            if vec.contains(x) {
                continue;
            }
            vec.push(x.clone());
        }
        Self(vec)
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
        if self.0.len() <= _other.0.len() {
            !self.0.iter().any(|x| _other.contains(x))
        } else {
            !_other.0.iter().any(|x| self.contains(x))
        }
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        let mut res = Self::new(&[]);
        if self.0.len() <= _other.0.len() {
            self.0
                .iter()
                .cloned()
                .filter(|x| _other.contains(x))
                .for_each(|x| res.add(x));
        } else {
            _other
                .0
                .iter()
                .cloned()
                .filter(|x| self.contains(x))
                .for_each(|x| res.add(x));
        }
        res
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        let mut res = Self::new(&[]);
        self.0
            .iter()
            .cloned()
            .filter(|x| !_other.contains(x))
            .for_each(|x| res.add(x));
        res
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        let mut res = Self::new(&[]);
        for x in self.0.clone() {
            res.add(x);
        }
        for x in _other.0.clone() {
            res.add(x);
        }
        res
    }
}

impl<T: PartialEq + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is_subset(other) && other.is_subset(self)
    }
}
