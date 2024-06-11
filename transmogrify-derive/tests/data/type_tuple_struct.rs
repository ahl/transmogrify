#[transmogrify(prefix = foo_crate)]
pub struct TupleStruct(pub String);
impl ::transmogrify::Transmogrify for TupleStruct {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        let value_0 = ::transmogrify::Transmogrify::transmogrify(&self.0);
        quote::quote! {
            foo_crate::TupleStruct(#value_0,)
        }
    }
}
