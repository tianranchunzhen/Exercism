pub struct Matcher<T> {
    func: Box<dyn Fn(T) -> bool>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: ToString,
    {
        Self {
            func: Box::new(matcher),
            subs: subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
pub struct Fizzy<T>(Vec<Matcher<T>>);

impl<T> Fizzy<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.0.push(matcher);
        self
    }

    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        T: std::fmt::Display + Copy,
        I: Iterator<Item = T>,
    {
        iter.map(move |i: T| {
            let repl: Vec<String> = self
                .0
                .iter()
                .filter_map(|matcher| {
                    if (*matcher.func)(i) {
                        Some(matcher.subs.clone())
                    } else {
                        None
                    }
                })
                .collect();
            if repl.is_empty() {
                i.to_string()
            } else {
                repl.join("")
            }
        })
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: From<u8> + PartialEq + std::ops::Rem<Output = T> + Copy,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % T::from(5) == T::from(0), "buzz"))
}
