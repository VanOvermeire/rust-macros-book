macro_rules! my_vec {
    () => {
        Vec::new()
    };
    (make an empty vec) => (
        Vec::new()
    );
    ($x:expr) => [
        {
            let mut v = Vec::new();
            v.push($x);
            v
        }
    ];
    ($($x:expr),+) => (
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )+
            v
        }
    );
}

fn main() {
    let empty: Vec<i32> = my_vec![];
    println!("{:?}", empty);
    let another_empty: Vec<i32> = my_vec![make an empty vec];
    println!("{:?}", another_empty);
    let t = my_vec!(1, 2, 3);
    println!("{:?}", t);
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_create_empty_vec() {
        let actual: Vec<i32> = my_vec!();

        assert_eq!(actual.len(), 0);
    }
    #[test]
    fn should_create_empty_vec_alt() {
        let actual: Vec<i32> = my_vec!(make an empty vec);

        assert_eq!(actual.len(), 0);
    }

    #[test]
    fn should_create_vec_with_one_element() {
        let actual = my_vec!(1);

        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], 1);
    }

    #[test]
    fn should_create_vec_with_given_elements() {
        let actual = my_vec!(1, 2, 3);

        assert_eq!(actual.len(), 3);
        assert_eq!(actual[0], 1);
        assert_eq!(actual[1], 2);
        assert_eq!(actual[2], 3);
    }
}
