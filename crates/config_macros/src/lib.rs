//use serde::Deserialize;
//use std::path::{Path, PathBuf};
//
//#[derive(Clone, Debug, Deserialize)]
//pub struct Config {}
//
//impl Config {
//    pub fn load_config(path: PathBuf) -> Result<Config, String> {
//        let p: &Path = path.as_ref();
//        let config_yaml = std::fs::read_to_string(p).map_err(|err| match err {
//            e @ std::io::Error { .. } if e.kind() == std::io::ErrorKind::NotFound => {
//                "Config file not found".to_string()
//            }
//            _ => err.to_string(),
//        })?;
//
//        let config: Config = serde_yaml::from_str(&config_yaml).map_err(|e| e.to_string())?;
//        Ok(config)
//    }
//}

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(LoadConfig)]
pub fn derive_load_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    let expanded = quote! {
        impl #struct_name {
            pub fn load_config(path: std::path::PathBuf) -> Result<#struct_name, String> {
                use std::path::Path;
                let p: &Path = path.as_ref();

                let config_yaml = std::fs::read_to_string(p).map_err(|err| match err {
                    e @ std::io::Error { .. } if e.kind() == std::io::ErrorKind::NotFound => {
                        "Config file not found".to_string()
                    }
                    _ => err.to_string(),
                })?;

                let config: #struct_name = serde_yaml::from_str(&config_yaml)
                    .map_err(|e| e.to_string())?;

                Ok(config)
            }
        }
    };

    TokenStream::from(expanded)
}
