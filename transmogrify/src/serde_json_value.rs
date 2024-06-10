use transmogrify_macro::transmogrify;

use crate::Transmogrify;

#[transmogrify]
impl Transmogrify for serde_json::Value {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        match self {
            serde_json::Value::Null => todo!(),
            serde_json::Value::Bool(_) => todo!(),
            serde_json::Value::Number(_) => todo!(),
            serde_json::Value::String(_) => todo!(),
            serde_json::Value::Array(_) => todo!(),
            serde_json::Value::Object(_) => todo!(),
        }
    }
}

impl Transmogrify for serde_json::Number {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        todo!()
    }
}

impl<K, V> Transmogrify for serde_json::Map<K, V>
where
    K: Transmogrify,
    V: Transmogrify,
{
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        todo!()
    }
}
