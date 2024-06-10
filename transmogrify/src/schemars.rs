use transmogrify_macro::transmogrify;

use crate::Transmogrify;

#[transmogrify]
impl Transmogrify for schemars::schema::Schema {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        match self {
            schemars::schema::Schema::Bool(_) => todo!(),
            schemars::schema::Schema::Object(_) => todo!(),
        }
    }
}

#[transmogrify]
impl Transmogrify for schemars::schema::SchemaObject {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        match self {
            schemars::schema::SchemaObject {
                metadata,
                instance_type,
                format,
                enum_values,
                const_value,
                subschemas,
                number,
                string,
                array,
                object,
                reference,
                extensions,
            } => todo!(),
        }
    }
}

#[transmogrify]
impl<T> Transmogrify for schemars::schema::SingleOrVec<T>
where
    T: Transmogrify,
{
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        match self {
            schemars::schema::SingleOrVec::Single(_) => todo!(),
            schemars::schema::SingleOrVec::Vec(_) => todo!(),
        }
    }
}

#[transmogrify]
impl Transmogrify for schemars::schema::Metadata {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        match self {
            Self {
                id,
                title,
                description,
                default,
                deprecated,
                read_only,
                write_only,
                examples,
            } => todo!(),
        }
    }
}

#[transmogrify]
impl Transmogrify for schemars::schema::InstanceType {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        match self {
            schemars::schema::InstanceType::Null => todo!(),
            schemars::schema::InstanceType::Boolean => todo!(),
            schemars::schema::InstanceType::Object => todo!(),
            schemars::schema::InstanceType::Array => todo!(),
            schemars::schema::InstanceType::Number => todo!(),
            schemars::schema::InstanceType::String => todo!(),
            schemars::schema::InstanceType::Integer => todo!(),
        }
    }
}

#[transmogrify]
impl Transmogrify for schemars::schema::SubschemaValidation {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        let schemars::schema::SubschemaValidation {
            all_of,
            any_of,
            one_of,
            not,
            if_schema,
            then_schema,
            else_schema,
        } = self;
    }
}

#[transmogrify]
impl Transmogrify for schemars::schema::NumberValidation {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        match self {
            Self {
                multiple_of,
                maximum,
                exclusive_maximum,
                minimum,
                exclusive_minimum,
            } => todo!(),
        }
    }
}

#[transmogrify]
impl Transmogrify for schemars::schema::StringValidation {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        match self {
            Self {
                max_length,
                min_length,
                pattern,
            } => todo!(),
        }
    }
}

#[transmogrify]
impl Transmogrify for schemars::schema::ArrayValidation {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        match self {
            Self {
                items,
                additional_items,
                max_items,
                min_items,
                unique_items,
                contains,
            } => todo!(),
        }
    }
}

#[transmogrify]
impl Transmogrify for schemars::schema::ObjectValidation {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        match self {
            Self {
                max_properties,
                min_properties,
                required,
                properties,
                pattern_properties,
                additional_properties,
                property_names,
            } => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use schemars::schema::Schema;
    use serde_json::json;

    use crate::Transmogrify;

    #[test]
    fn test_schema() {
        let raw = json!(
            {
                "example": "qsfp0",
                "title": "PortId",
                "description": "Physical switch port identifier",
                "oneOf": [
                    {
                        "title": "internal",
                        "type": "string",
                        "pattern": "(^[iI][nN][tT]0$)"
                    },
                    {
                        "title": "rear",
                        "type": "string",
                        "pattern": "(^[rR][eE][aA][rR](([0-9])|([1-2][0-9])|(3[0-1]))$)"
                    },
                    {
                        "title": "qsfp",
                        "type": "string",
                        "pattern": "(^[qQ][sS][fF][pP](([0-9])|([1-2][0-9])|(3[0-1]))$)"
                    }
                ]
            }
        );

        let schema = serde_json::from_value::<Schema>(raw).expect("from_value failed");

        let xxx = schema.transmogrify();

        println!("{}", xxx);

        let file = quote! {
            fn main() {
                let _ = #xxx;
            }
        };

        let file = syn::parse2(file).unwrap();
        let actual = prettyplease::unparse(&file);
        expectorate::assert_contents("tests/data/test_schema.rs", &actual);
    }
}
