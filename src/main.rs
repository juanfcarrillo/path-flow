fn main() {
    println!("Hello, world!");

    struct Test {
        list: Vec<i8>,
    }

    impl Test {
        fn new() -> Self {
            Test {
                list: Vec::new(),
            }
        }

        fn add(&mut self, value: i8) {
            self.list.push(value);
        }
    }
}
