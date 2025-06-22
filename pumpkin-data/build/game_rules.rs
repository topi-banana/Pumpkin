use std::{collections::HashMap, fs};

use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_json::Value;

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=../assets/game_rules.json");

    let game_rules: HashMap<String, Value> =
        serde_json::from_str(&fs::read_to_string("../assets/game_rules.json").unwrap())
            .expect("Failed to parse game_rules.json");

    let mut variants = TokenStream::new();
    let mut defaults = TokenStream::new();

    for (raw_name, raw_value) in game_rules.iter() {
        let name = format_ident!("{}", raw_name.to_snake_case());

        match raw_value {
            Value::Bool(b) => {
                variants.extend(quote! {
                    #[serde(rename = #raw_name)]
                    #[serde(with = "as_string")]
                    pub #name: bool,
                });
                defaults.extend(quote! {
                    #name: #b,
                });
            }
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    variants.extend(quote! {
                        #[serde(rename = #raw_name)]
                        #[serde(with = "as_string")]
                        pub #name: i64,
                    });
                    defaults.extend(quote! {
                        #name: #i,
                    });
                } else {
                    panic!("Expected integer for rule '{}'", raw_name);
                }
            }
            _ => panic!("Unsupported value type for key '{}'", raw_name),
        }
    }

    quote! {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
        pub struct GameRules {
            #variants
        }

        impl Default for GameRules {
            fn default() -> Self {
                Self {
                    #defaults
                }
            }
        }

        mod as_string {
            use serde::{Serializer, Deserializer, Deserialize};
            use std::fmt::Display;
            use std::str::FromStr;

            pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
            where
                T: Display,
                S: Serializer,
            {
                serializer.serialize_str(&value.to_string())
            }

            pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
            where
                T: FromStr,
                D: Deserializer<'de>,
                <T as FromStr>::Err: std::fmt::Display,
            {
                let s = String::deserialize(deserializer)?;
                s.parse::<T>().map_err(serde::de::Error::custom)
            }
        }
    }
}
