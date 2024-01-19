use iac_macro::iac;

fn main() {
    iac! {
        bucket uniquename
    }
    iac! {
        lambda (name = a_name)
    }
    iac! {
        lambda (name = my_name, mem = 1024, time = 15)
    }
    iac! {
        bucket uniquename => lambda (name = my_name, mem = 1024, time = 15)
    }
}
