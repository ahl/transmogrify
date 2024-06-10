fn main() {
    let _ = schemars::schema::Schema::Object(schemars::schema::SchemaObject {
        metadata: Some(
            Box::new(schemars::schema::Metadata {
                id: None,
                title: Some(String::from("PortId")),
                description: Some(String::from("Physical switch port identifier")),
                default: None,
                deprecated: false,
                read_only: false,
                write_only: false,
                examples: <std::vec::Vec<_>>::new(),
            }),
        ),
        instance_type: None,
        format: None,
        enum_values: None,
        const_value: None,
        subschemas: Some(
            Box::new(schemars::schema::SubschemaValidation {
                all_of: None,
                any_of: None,
                one_of: Some(
                    <std::vec::Vec<
                        _,
                    >>::from([
                        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                            metadata: Some(
                                Box::new(schemars::schema::Metadata {
                                    id: None,
                                    title: Some(String::from("internal")),
                                    description: None,
                                    default: None,
                                    deprecated: false,
                                    read_only: false,
                                    write_only: false,
                                    examples: <std::vec::Vec<_>>::new(),
                                }),
                            ),
                            instance_type: Some(
                                schemars::schema::SingleOrVec::Single(
                                    Box::new(schemars::schema::InstanceType::String),
                                ),
                            ),
                            format: None,
                            enum_values: None,
                            const_value: None,
                            subschemas: None,
                            number: None,
                            string: Some(
                                Box::new(schemars::schema::StringValidation {
                                    max_length: None,
                                    min_length: None,
                                    pattern: Some(String::from("(^[iI][nN][tT]0$)")),
                                }),
                            ),
                            array: None,
                            object: None,
                            reference: None,
                            extensions: <std::collections::BTreeMap<_, _>>::new(),
                        }),
                        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                            metadata: Some(
                                Box::new(schemars::schema::Metadata {
                                    id: None,
                                    title: Some(String::from("rear")),
                                    description: None,
                                    default: None,
                                    deprecated: false,
                                    read_only: false,
                                    write_only: false,
                                    examples: <std::vec::Vec<_>>::new(),
                                }),
                            ),
                            instance_type: Some(
                                schemars::schema::SingleOrVec::Single(
                                    Box::new(schemars::schema::InstanceType::String),
                                ),
                            ),
                            format: None,
                            enum_values: None,
                            const_value: None,
                            subschemas: None,
                            number: None,
                            string: Some(
                                Box::new(schemars::schema::StringValidation {
                                    max_length: None,
                                    min_length: None,
                                    pattern: Some(
                                        String::from(
                                            "(^[rR][eE][aA][rR](([0-9])|([1-2][0-9])|(3[0-1]))$)",
                                        ),
                                    ),
                                }),
                            ),
                            array: None,
                            object: None,
                            reference: None,
                            extensions: <std::collections::BTreeMap<_, _>>::new(),
                        }),
                        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                            metadata: Some(
                                Box::new(schemars::schema::Metadata {
                                    id: None,
                                    title: Some(String::from("qsfp")),
                                    description: None,
                                    default: None,
                                    deprecated: false,
                                    read_only: false,
                                    write_only: false,
                                    examples: <std::vec::Vec<_>>::new(),
                                }),
                            ),
                            instance_type: Some(
                                schemars::schema::SingleOrVec::Single(
                                    Box::new(schemars::schema::InstanceType::String),
                                ),
                            ),
                            format: None,
                            enum_values: None,
                            const_value: None,
                            subschemas: None,
                            number: None,
                            string: Some(
                                Box::new(schemars::schema::StringValidation {
                                    max_length: None,
                                    min_length: None,
                                    pattern: Some(
                                        String::from(
                                            "(^[qQ][sS][fF][pP](([0-9])|([1-2][0-9])|(3[0-1]))$)",
                                        ),
                                    ),
                                }),
                            ),
                            array: None,
                            object: None,
                            reference: None,
                            extensions: <std::collections::BTreeMap<_, _>>::new(),
                        }),
                    ]),
                ),
                not: None,
                if_schema: None,
                then_schema: None,
                else_schema: None,
            }),
        ),
        number: None,
        string: None,
        array: None,
        object: None,
        reference: None,
        extensions: <std::collections::BTreeMap<
            _,
            _,
        >>::from([
            (String::from("example"), serde_json::Value::String(String::from("qsfp0"))),
        ]),
    });
}
