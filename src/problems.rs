pub struct Problem {
    pub message: String,
}

pub struct Problems {
    pub problems: Vec<Problem>,
}

impl Problems {
    pub const fn new() -> Self {
        Self { problems: vec![] }
    }

    pub fn report(&mut self, message: String) {
        self.problems.push(Problem { message });
    }
}
