extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Ident};
use convert_case::{Case, Casing};

#[proc_macro_derive(EnumToFunction)]
pub fn enum_to_function_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the enum
    let name = &input.ident;

    // Generate function calls based on the enum variants
    let functions = if let Data::Enum(data_enum) = &input.data {
        data_enum.variants.iter().map(|variant| {
            let variant_ident = &variant.ident;
            let function_name = Ident::new(&variant_ident.to_string().to_case(Case::Snake), variant_ident.span());

            let params = if let Fields::Unnamed(fields_unnamed) = &variant.fields {
                fields_unnamed.unnamed.iter().enumerate().map(|(i, _)| {
                    let param_name = Ident::new(&format!("param{}", i), variant_ident.span());
                    quote! { #param_name }
                }).collect::<Vec<_>>()
            } else {
                vec![]
            };

            quote! {
                #name::#variant_ident(#(#params),*) => #function_name(#(#params),*),
            }
        }).collect::<Vec<_>>()
    } else {
        vec![]
    };

    // Generate the implementation of the execute method
    let expanded = quote! {
        impl #name {
            pub fn execute(&self) {
                match *self {
                    #(#functions)*
                }
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}