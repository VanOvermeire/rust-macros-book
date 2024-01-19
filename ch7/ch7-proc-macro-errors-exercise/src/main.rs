use crate::other_file::Example;

mod other_file;

fn main() {
    let e = Example::new();

    e.get_string_value();
    e.get_number_value();
}
