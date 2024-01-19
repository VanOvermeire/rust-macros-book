trait Hello {
    fn hello(&self);
}

impl<T: Copy> Hello for T {
    fn hello(&self) {
        println!("Hello world");
    }
}

fn main() {
    2.hello();
    true.hello();
    'c'.hello();
}
