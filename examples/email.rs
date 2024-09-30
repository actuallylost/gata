// use Gata::patterns::is_email;

fn main() {
    let input = "email@example.com";

    match is_email(input) {
        Ok(val) => val,
        Err(err) => panic!("{err}"),
    };
}
