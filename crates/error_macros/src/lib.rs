use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit};

//#[proc_macro_attribute]
#[proc_macro_derive(ErrorWithCode, attributes(code))]
pub fn error_with_codes(input: TokenStream) -> TokenStream {
    // Parse the input as a `DeriveInput`
    let input = parse_macro_input!(input as DeriveInput);

    // Ensure the input is an enum
    let enum_name = &input.ident;
    let data = match &input.data {
        syn::Data::Enum(data) => data,
        _ => {
            return syn::Error::new(Span::call_site(), "This macro can only be used on enums")
                .to_compile_error()
                .into();
        }
    };

    // Generate match arms for `error_code` and `error_message`
    let mut match_arms_code = Vec::new();
    let mut match_arms_message = Vec::new();

    for variant in &data.variants {
        let variant_name = &variant.ident;

        // Default error code
        let mut code = quote!(200);

        // Parse attributes like `#[code(404)]`
        for attr in &variant.attrs {
            if let Some(attr_meta_name) = attr.path().get_ident() {
                if attr_meta_name == "code" {
                    if let Ok(Lit::Int(attr_args)) = attr.parse_args() {
                        code = quote!(#attr_args);
                    }
                }
            }
        }

        // Generate match arms
        match_arms_code.push(quote! {
            Self::#variant_name { .. } => #code,
        });

        match_arms_message.push(quote! {
            Self::#variant_name { .. } => self.to_string(),
        });
    }

    // Generate the implementation
    let expanded = quote! {
        //#input

        impl #enum_name {
            pub fn error_code(&self) -> u16 {
                match self {
                    #(#match_arms_code)*
                }
            }

            pub fn error_message(&self) -> String {
                match self {
                    #(#match_arms_message)*
                }
            }
        }
    };

    expanded.into()
}
