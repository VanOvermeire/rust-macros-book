use builder_macro::Builder;

struct DoesNotImplementDefault;

#[derive(Builder)]
#[builder_defaults]
struct ExampleStruct {
    not: DoesNotImplementDefault
}

fn main() {}
