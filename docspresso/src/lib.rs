// src/lib.rs
use cxx_qt::bridge;

#[cxx_qt::bridge]
mod my_object {
    #[cxx_qt::qobject]
    pub struct MyObject {
        #[qproperty]
        pub counter: i32,
    }

    impl Default for MyObject {
        fn default() -> Self {
            Self { counter: 0 }
        }
    }

    impl MyObject {
        #[qinvokable]
        pub fn increment(&mut self) {
            self.counter += 1;
            println!("Counter: {}", self.counter);
        }
    }
}
