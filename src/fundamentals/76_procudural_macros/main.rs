/*
Your task is to create a procedural macro named derive_describe that can be used to generate the Describe trait implementation for any struct. The generated describe method should return a string representation of the struct, including its name and all its fields with their respective values.

For example, applying the macro to a struct like:

#[derive(Describe)]
struct Point {
    x: i32,
    y: i32,
}

Should generate a Describe implementation such that:

let p = Point { x: 1, y: 2 };
assert_eq!(p.describe(), "Point { x: 1, y: 2 }");

Requirements

    The derive_describe macro should:
        Generate the Describe trait implementation for the struct it is applied to.
        Support structs with named fields. (Tuple structs or unit structs are not required for this challenge.)

    The describe method should return a properly formatted string, including the struct name and fields with their values.
*/

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                let field_prints = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! {
                        format!("{}: {:?}", stringify!(#name), &self.#name)
                    }
                });

                quote! {
                    impl Describe for #name {
                        fn describe(&self) -> String {
                            let fields_str = vec![#(#field_prints),*].join(", ");
                            format!("{} {{ {} }}", stringify!(#name), fields_str)
                        }
                    }
                }
            }
            _ => panic!("Describe macro only supports structs with named fields"),
        },
        _ => panic!("Describe macro only supports structs"),
    };

    TokenStream::from(expanded)
}

// Example Test
//#[test]
//fn test_example() {
//    #[derive(Describe)]
//    struct Person {
//        name: String,
//        age: u32,
//    }
//
//    let person = Person {
//        name: "Alice".to_string(),
//        age: 30,
//    };
//
//    assert_eq!(person.describe(), "Person { name: \"Alice\", age: 30 }");
//}
