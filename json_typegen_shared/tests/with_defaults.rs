use rust_decimal::Decimal;
use testsyn::{parse_str, Item};

use json_typegen_shared::{codegen, Options};

/// Function to test AST equality, not string equality
fn code_output_test(name: &str, input: &str, expected: &str) {
    let res = codegen(name, input, Options::default());
    let output = res.unwrap();
    assert_eq!(
        // Wrapping in mod Foo { } since there is no impl Parse for Vec<Item>
        parse_str::<Item>(&format!("mod Foo {{ {} }}", &output)).unwrap(),
        parse_str::<Item>(&format!("mod Foo {{ {} }}", expected)).unwrap(),
        "\n\nUnexpected output code:\n  input: {}\n  output:\n{}\n  expected: {}",
        input,
        output,
        expected
    );
}

#[test]
fn empty_object() {
    code_output_test(
        "Root",
        r##"
            {}
        "##,
        r##"
            #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
            pub struct Root {}
        "##,
    );
}

#[test]
fn point() {
    code_output_test(
        "Point",
        r##"
            {
                "x": 2,
                "y": 3
            }
        "##,
        r##"
            #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
            pub struct Point {
                pub x: Decimal,
                pub y: Decimal,
            }
        "##,
    );
}

#[test]
fn pub_crate_point() {
    code_output_test(
        "pub(crate) Point",
        r##"
            {
                "x": 2,
                "y": 3
            }
        "##,
        r##"
            #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
            pub(crate) struct Point {
                pub x: Decimal,
                pub y: Decimal,
            }
        "##,
    );
}

#[test]
fn optionals() {
    code_output_test(
        "Optional",
        r##"
            [
                {
                    "in_both": 5,
                    "missing": 5,
                    "has_null": 5
                },
                {
                    "in_both": 5,
                    "has_null": null,
                    "added": 5
                }
            ]
        "##,
        r##"
            #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
            pub struct Optional {
                pub in_both: Decimal,
                pub missing: Option<Decimal>,
                pub has_null: Option<Decimal>,
                pub added: Option<Decimal>,
            }
        "##,
    );
}

#[test]
fn fallback() {
    code_output_test(
        "Fallback",
        r##"
            [
                {
                    "only_null": null,
                    "conflicting": 5,
                    "empty_array": []
                },
                {
                    "only_null": null,
                    "conflicting": "five",
                    "empty_array": []
                }
            ]
        "##,
        r##"
            #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
            pub struct Fallback {
                pub only_null: ::serde_json::Value,
                pub conflicting: ::serde_json::Value,
                pub empty_array: Vec<::serde_json::Value>,
            }
        "##,
    );
}

#[test]
fn nesting() {
    code_output_test(
        "Root",
        r##"
            [
                {
                    "nested": {
                        "a": 5,
                        "doubly_nested": { "c": 10 }
                    },
                    "in_array": [{ "b": 5 }]
                }
            ]
        "##,
        r##"
            #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
            pub struct Root {
                pub nested: Nested,
                pub in_array: Vec<InArray>,
            }

            #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
            pub struct Nested {
                pub a: Decimal,
                pub doubly_nested: DoublyNested,
            }

            #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
            pub struct DoublyNested {
                pub c: Decimal,
            }

            #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
            pub struct InArray {
                pub b: Decimal,
            }
        "##,
    );
}

#[test]
fn tuple() {
    code_output_test(
        "Pagination",
        r##"
            [
                {
                    "pages": 1,
                    "items": 3
                },
                [
                    {
                        "name": "John"
                    },
                    {
                        "name": "James"
                    },
                    {
                        "name": "Jake"
                    }
                ]
            ]
        "##,
        r##"
            #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
            pub struct Pagination {
                pub pages: Decimal,
                pub items: Decimal,
            }

            #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
            pub struct Pagination2 {
                pub name : String,
            }
        "##,
    );
}

#[test]
fn rename() {
    code_output_test(
        "Renamed",
        r##"
            {
                "type": 5
            }
        "##,
        r##"
            #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
            pub struct Renamed {
                #[serde(rename = "type")]
                pub type_field: Decimal,
            }
        "##,
    );
}
