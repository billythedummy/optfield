use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, Index, ItemStruct};

use crate::args::Args;
use crate::fields;

pub fn generate(item: &ItemStruct, opt_item: &ItemStruct, args: &Args) -> TokenStream {
    if args.from {
        let item_name = &item.ident;

        let opt_name = &opt_item.ident;
        let opt_generics = &opt_item.generics;

        let fields = field_bindings(&item.fields, args);

        quote! {
            impl#opt_generics From<#item_name#opt_generics> for #opt_name#opt_generics {
                fn from(item: #item_name#opt_generics) -> #opt_name#opt_generics {
                    #opt_name {
                        #fields
                    }
                }
            }
        }
    } else {
        TokenStream::new()
    }
}

fn field_bindings(fields: &Fields, args: &Args) -> TokenStream {
    let mut tokens = TokenStream::new();

    for (i, field) in fields.iter().enumerate() {
        let field_name = match &field.ident {
            // means that original item is a tuple struct
            None => {
                let index = Index::from(i);

                quote!(#index)
            }
            Some(ident) => quote!(#ident),
        };

        let field_tokens = if fields::is_option(field) && !args.rewrap {
            quote! {
                #field_name: item.#field_name,
            }
        } else {
            quote! {
                #field_name: Some(item.#field_name),
            }
        };

        tokens.extend(field_tokens);
    }

    tokens
}
