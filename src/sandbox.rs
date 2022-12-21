#[cfg(test)]

mod sandbox {
    trait Animal {
        fn name(&self) -> String;
    }

    trait Human: Animal {
        fn first_name(&self) -> String;
        fn last_name(&self) -> String;
    }

    impl<T: Human> Animal for T {
        fn name(&self) -> String {
            format!("{} {}", self.first_name(), self.last_name())
        }
    }

    struct Foo {}

    impl Human for Foo {
        fn first_name(&self) -> String { String::from("John") }
        fn last_name(&self) -> String { String::from("Smith") }
    }

    #[test]
    fn test() {
        println!("{}", Foo{}.name())
    }
}

