#[transmogrify(prefix = foo_crate)]
pub struct SimpleStruct {
    pub foo: String,
}
impl ::transmogrify::Transmogrify for SimpleStruct {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        let foo = ::transmogrify::Transmogrify::transmogrify(&self.foo);
        quote::quote! {
            foo_crate::SimpleStruct { foo : #foo, }
        }
    }
}
