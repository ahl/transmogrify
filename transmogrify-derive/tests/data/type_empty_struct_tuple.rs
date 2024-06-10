#[transmogrify(prefix = foo_crate)]
pub struct EmptyStructTuple();
impl ::transmogrify::Transmogrify for EmptyStructTuple {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        quote::quote! {
            foo_crate::EmptyStructTuple()
        }
    }
}
