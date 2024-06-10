use expectorate::assert_contents;
use prettyplease::unparse;
use transmogrify::Transmogrify;

fn test_value<T: Transmogrify>(value: T, name: &'static str) {
    let output = value.transmogrify();
    println!("{}", output);
    let file = syn::parse_quote! {
        fn main() {
            let _ = #output;
        }
    };

    let actual = unparse(&file);
    assert_contents(format!("tests/data/{}.rs", name), &actual)
}

#[test]
fn test_derive_struct() {
    #[derive(Transmogrify)]
    #[transmogrify(prefix = crate)]
    pub struct TestStructNamed {
        pub a: String,
        pub b: Vec<u32>,
    }

    let value = TestStructNamed {
        a: "A".to_string(),
        b: vec![1, 2, 3],
    };

    test_value(value, "test_derive_struct");
}
