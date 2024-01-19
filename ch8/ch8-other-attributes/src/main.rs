use other_attributes_macro::analyze;

analyze!(
    /// outer comment
    /** comment block */
    struct Example {
        //! inner comment
        /*! inner comment block */
        val: String
    }
);

fn main() {}
