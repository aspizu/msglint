use std::cell::RefCell;

thread_local! {
    pub static PROBLEMS: RefCell<Problems> = RefCell::new(Problems::new());
}

#[macro_export]
macro_rules! report {
    ($($t:tt)*) => {
        PROBLEMS.with_borrow_mut(|problems| {
            problems.report(format!($($t)*))
        })
    }
}

#[macro_export]
macro_rules! report_string {
    ($($t:tt)*) => {
        PROBLEMS.with(|problems| {
            problems.borrow_mut().report($($t)*)
        })
    }
}

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
