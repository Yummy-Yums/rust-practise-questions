/*
    Create an enum named Json that can work with arbitrary JSON data.
 */

pub mod arbitrary_json_parser{
    use serde_json::Value;
    use std::fmt::Display;

    // Simple wrapper enum for JSON values
    #[derive(Debug, PartialEq)]
    pub enum Json {
        Null,
        Bool(bool),
        Number(f64),
        String(String),
        Array(Vec<Json>),
        Object(std::collections::HashMap<String, Json>),
    }

    // Conversion from serde_json::Value for convenience
    impl From<Value> for Json {
        fn from(value: Value) -> Self {
            match value {
                Value::Null => Json::Null,
                Value::Bool(b) => Json::Bool(b),
                Value::Number(n) => Json::Number(n.as_f64().unwrap_or(0.0)),
                Value::String(s) => Json::String(s),
                Value::Array(arr) => Json::Array(arr.into_iter().map(Json::from).collect()),
                Value::Object(obj) => Json::Object(
                    obj.into_iter()
                        .map(|(k, v)| (k, Json::from(v)))
                        .collect(),
                ),
            }
        }
    }

    use std::fmt;

    impl fmt::Display for Json {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Json::Null => write!(f, "Null"),
                Json::Bool(b) => write!(f, "Bool({})", b),
                Json::Number(n) => write!(f, "Number({})", n),
                Json::String(s) => write!(f, "String({:?})", s),  // Use `{:?}` for escaping quotes
                Json::Array(arr) => {
                    write!(f, "Array(")?;
                    let mut first = true;
                    for item in arr {
                        if !first {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", item)?;
                        first = false;
                    }
                    write!(f, ")")
                }
                Json::Object(obj) => {
                    write!(f, "Object(")?;
                    let mut first = true;
                    for (k, v) in obj {
                        if !first {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}: {}", k, v)?;
                        first = false;
                    }
                    write!(f, ")")
                }
            }
        }
    }

    // Example usage
    pub fn main() {
        let json_data = r#"
        {
            "name": "John",
            "age": 30,
            "is_active": true,
            "tags": ["rust", "serde"],
            "address": {
                "street": "123 Main St",
                "city": "New York"
            }
        }
    "#;

        // Parse using serde_json then convert to our Json enum
        let value: Value = serde_json::from_str(json_data).unwrap();
        let json = Json::from(value);

        println!("{:#?}", json);
    }
}

mod tests{
    use serde_json::Value;
    use serde_json::Value::Object;
    use crate::chp4::arbitrary_json_parser::arbitrary_json_parser::Json;


    #[test]
    fn test_json_parser() {
        let json_data = r#"
            {
                "name": "John",
                "age": 30,
                "is_active": true,
                "tags": ["rust", "serde"],
                "address": {
                    "street": "123 Main St",
                    "city": "New York"
                }
            }
        "#;

        let value: Value = serde_json::from_str(json_data).unwrap();
        let parsed_json = Json::from(value);

        // Manually construct the expected Json
        let expected_json = Json::Object(
            vec![
                ("name".to_string(), Json::String("John".to_string())),
                ("age".to_string(), Json::Number(30.0)),
                ("is_active".to_string(), Json::Bool(true)),
                (
                    "tags".to_string(),
                    Json::Array(vec![
                        Json::String("rust".to_string()),
                        Json::String("serde".to_string()),
                    ]),
                ),
                (
                    "address".to_string(),
                    Json::Object(
                        vec![
                            (
                                "street".to_string(),
                                Json::String("123 Main St".to_string()),
                            ),
                            (
                                "city".to_string(),
                                Json::String("New York".to_string()),
                            ),
                        ]
                            .into_iter()
                            .collect(),
                    ),
                ),
            ]
                .into_iter()
                .collect(),
        );

        assert_eq!(parsed_json, expected_json);
    }
}