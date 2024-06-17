// src/calculator.rs

pub struct Calculator {
    value: f64,
}

impl Calculator {
    pub fn new(value: f64) -> Calculator {
        Calculator { value }
    }

    pub fn add(&mut self, num: f64) {
        self.value += num;
    }

    pub fn multiply(&mut self, num: f64) {
        self.value *= num;
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn format_value(&self) -> String {
        format!("The result is: {}", self.value)
    }
}
