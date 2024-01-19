use crate::other_file::Example;

mod other_file;

fn main() {
    let e = Example::new();

    e.get_string_value();
    e.get_number_value();
}

#[cfg(test)]
mod tests {
    use crate::Example;

    #[test]
    fn generates_necessary_methods() {
        let e = Example::new();

        assert_eq!(e.get_string_value(), &"value");
        assert_eq!(e.get_number_value(), &2);
    }
}