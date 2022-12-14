struct Sandbox {
    value: String,
}
impl Sandbox {
    pub fn start(&mut self) {
        let mut value = &self.value;

        self.change(); // self mutable borrow

        value = &self.value;

        value.as_str(); // value immutable borrow
    }

    fn change(&mut self) {}

    fn get_value(&self) -> &String {
        &self.value
    }
}
