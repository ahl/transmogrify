fn main() {
    let _ = crate::TestStructNamed {
        a: String::from("A"),
        b: <std::vec::Vec<_>>::from([1u32, 2u32, 3u32]),
    };
}
