mod measure;

mod qubit;
mod system;
mod tensor;

mod macros {

    #[macro_export]
    macro_rules! test_cases {
            ( $($label:ident : $eval:ident $args:expr, $input:expr, $expected:expr);* $(;)? ) => {
            $(
                #[test]
                fn $label() {
                    $eval($args, $input, $expected);
                }
            )*
        }
    }

    pub use test_cases;
}
