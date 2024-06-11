fn main() {
    let _ = crate::TestStructNamed {
        a: String::from("A"),
        b: <std::vec::Vec<_>>::from([1u32, 2u32, 3u32]),
        time: chrono::DateTime::<chrono::Utc>::from_timestamp(0i64, 0u32).unwrap(),
        nz: std::num::NonZeroU8::new(42u8).unwrap(),
    };
}
