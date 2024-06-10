#[transmogrify(prefix = foo_crate)]
pub struct EmptyStruct {}
impl ::transmogrify::Transmogrify for EmptyStruct {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        quote::quote! {
            foo_crate::EmptyStruct {}
        }
    }
}
