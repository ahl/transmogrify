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
        pub time: chrono::DateTime<chrono::Utc>,
        pub nz: std::num::NonZeroU8,
    }

    let value = TestStructNamed {
        a: "A".to_string(),
        b: vec![1, 2, 3],
        time: chrono::DateTime::<chrono::Utc>::from_timestamp(0, 0).unwrap(),
        nz: 42.try_into().unwrap(),
    };

    test_value(value, "test_derive_struct");
}
