pub struct ErrorNoAttr {}
::core::compile_error! {
    "must specify a path prefix #[transmogrify(prefix = <path>)]"
}
impl ::transmogrify::Transmogrify for ErrorNoAttr {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        quote::quote! {
            ::ErrorNoAttr {}
        }
    }
}
