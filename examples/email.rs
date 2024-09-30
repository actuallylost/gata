// use Gata::prelude::*;

fn main() {
    let input = Gata::new("email@example.com");

    match input.is_email() {
        Ok(val) => val,
        Err(err) => panic!("{err}"),
    };
}
