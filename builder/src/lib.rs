use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index,
};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    let builder_string = Ident::new(&format!("{name}Builder"), Span::call_site());

    println!("hey imma macro lookitme");

    let stuff = quote! {
        impl #name {
            pub fn builder() -> #builder_string {
                #builder_string {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }

        #[derive(Debug)]
        pub struct #builder_string {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

    };

    TokenStream::from(stuff)
}
