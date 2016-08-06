struct Callbacks<T> {
    callbacks: Vec<Box<FnMut(&T)>>,
}

impl<T> Callbacks<T> {
    pub fn new() -> Self {
        Callbacks { callbacks: Vec::new() }
    }

    pub fn register<F: FnMut(&T) + 'static>(&mut self, c: F) {
        self.callbacks.push(Box::new(c));
    }

    pub fn call(&mut self, val: &T) {
        for c in self.callbacks.iter_mut() {
            c(val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Callbacks;

    #[test]
    fn basic_cb() {
        let mut callbacks = Callbacks::new();

        callbacks.register(|num| assert_eq!(num, &55));

        callbacks.call(&55);
    }

    #[test]
    fn vec_cb() {
        let mut callbacks: Callbacks<Vec<f32>> = Callbacks::new();
        let test_vec = vec![4.5, 5.5, 6.6];
        let mut sum = 1.0;

        callbacks.register(move |vec| {
            for v in vec.iter() {
                sum = sum + v;
            }
            assert_eq!(sum, 1.0 + 4.5 + 5.5 + 6.6);
        });

        assert_eq!(sum, 1.0);
        callbacks.call(&test_vec);
    }
}
