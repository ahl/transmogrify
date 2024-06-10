#![doc = include_str!("../../README.md")]

pub use transmogrify_derive::Transmogrify;

mod basic;
#[cfg(feature = "chrono")]
mod chrono;
#[cfg(feature = "schemars")]
mod schemars;
#[cfg(any(feature = "schemars", feature = "json_value"))]
mod serde_json_value;

pub trait Transmogrify {
    fn transmogrify(&self) -> proc_macro2::TokenStream;
}

#[cfg(test)]
mod tests {
    use transmogrify_macro::transmogrify;

    use crate::Transmogrify;

    #[test]
    fn test_generated_transmogrify() {
        #[derive(Debug)]
        #[allow(dead_code)]
        enum TestEnum {
            A,
            B(u32, String),
            C { value: u32 },
        }

        #[derive(Debug)]
        struct TestStruct {
            a: u32,
            b: String,
        }

        #[transmogrify]
        impl Transmogrify for TestEnum {
            fn transmogrify(&self) -> proc_macro2::TokenStream {
                match self {
                    TestEnum::A => todo!(),
                    TestEnum::B(_, _) => todo!(),
                    TestEnum::C { value } => todo!(),
                }
            }
        }

        #[transmogrify]
        impl Transmogrify for TestStruct {
            fn transmogrify(&self) -> proc_macro2::TokenStream {
                let Self { a, b } = self;
                todo!()
            }
        }

        let value = TestEnum::A.transmogrify();
        let expected = quote::quote! {
            TestEnum::A
        };
        assert_eq!(value.to_string(), expected.to_string());

        let value = TestEnum::B(32, "s".to_string()).transmogrify();
        let expected = quote::quote! {
            TestEnum::B(32u32, String::from("s"),)
        };
        assert_eq!(value.to_string(), expected.to_string());

        let value = TestEnum::C { value: 42 }.transmogrify();
        let expected = quote::quote! {
            TestEnum::C { value: 42u32, }
        };
        assert_eq!(value.to_string(), expected.to_string());

        let value = TestStruct {
            a: 100,
            b: "b".to_string(),
        }
        .transmogrify();
        let expected = quote::quote! {
            TestStruct {
                a: 100u32,
                b: String::from("b"),
            }
        };
        assert_eq!(value.to_string(), expected.to_string());
    }
}
