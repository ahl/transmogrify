#[transmogrify(prefix = foo_crate)]
pub struct MarkerStruct;
impl ::transmogrify::Transmogrify for MarkerStruct {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        quote::quote! {
            foo_crate::MarkerStruct
        }
    }
}
