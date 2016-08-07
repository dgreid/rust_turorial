use std::rc::Rc;
use std::cell::RefCell;

struct Callbacks<T> {
    callbacks: Vec<Rc<Fn(&T)>>,
}

impl<T> Callbacks<T> {
    pub fn new() -> Self {
        Callbacks { callbacks: Vec::new() }
    }

    pub fn register<F: Fn(&T) + 'static>(&mut self, c: F) {
        self.callbacks.push(Rc::new(c));
    }

    pub fn call(&self, val: &T) {
        for c in self.callbacks.iter() {
            c(val);
        }
    }
}

#[derive(Clone)]
struct CallbacksMut {
    callbacks: Vec<Rc<RefCell<FnMut(i32)>>>,
}

impl CallbacksMut {
    pub fn new() -> Self {
        CallbacksMut { callbacks: Vec::new() }
    }

    pub fn register<F: FnMut(i32) + 'static>(&mut self, c: F) {
        let cell = Rc::new(RefCell::new(c));
        self.callbacks.push(cell);
    }

    pub fn call(&mut self, val: i32) {
        for c in self.callbacks.iter_mut() {
            let mut closure = c.borrow_mut();
            (&mut *closure)(val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Callbacks;
    use super::CallbacksMut;
    use std::cell::Cell;

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
        let sum = Cell::new(1.0);

        callbacks.register(move |vec| {
            for v in vec.iter() {
                sum.set(sum.get() + v);
            }
            assert_eq!(sum.get(), 1.0 + 4.5 + 5.5 + 6.6);
        });

        callbacks.call(&test_vec);
    }

    #[test]
    fn test_refcell() {
        let mut c = CallbacksMut::new();
        c.register(|val| println!("Callback 1: {}", val));
        c.call(0);

        {
            let mut count: usize = 0;
            c.register(move |val| {
                    count = count+1;
                    println!("Callback 2: {} ({}. time)", val, count);
                    } );
        }
        c.call(1); c.clone().call(2);
    }
}
