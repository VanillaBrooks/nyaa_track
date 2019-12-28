use std::fmt::Debug;
pub trait WontError {
    fn wont_error(self, arg: &str);
}

impl<T, E: Debug> WontError for Result<T, E> {
    fn wont_error(self, arg: &str) {
        match self {
            Ok(_) => {}
            Err(err) => {
                println! {"There was an error that was piped to wont_error mistakenly"};
                println! {"arg: {}", arg};
                dbg! {err};
            }
        }
    }
}
