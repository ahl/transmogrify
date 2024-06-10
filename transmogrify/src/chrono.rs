use crate::Transmogrify;

use quote::quote;

impl Transmogrify for chrono::DateTime<chrono::Utc> {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        let secs = self.timestamp();
        let nanos = self.timestamp_subsec_nanos();
        quote! {
            chrono::DateTime::<chrono::Utc>::from_timestamp(#secs, #nanos).unwrap()
        }
    }
}
