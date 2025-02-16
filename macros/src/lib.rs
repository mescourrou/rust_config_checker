extern crate proc_macro;

use std::{mem, str::FromStr};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{token::Token, Data, Ident, LitStr, Token};

#[proc_macro_derive(Check, attributes(check, ge, gt, le, lt, inside, or))]
pub fn derive_check_config(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    
    let struct_identifier = &input.ident;

    let mut implementation = quote! {
        let mut fields: Vec<String> = Vec::new();
    };
    match &input.data {
        Data::Struct(syn::DataStruct { fields, ..}) => {
            for field in fields {
                let id = field.ident.as_ref().unwrap();
                for attr in &field.attrs {
                    // attr.
                    if let Some(attr_id) = &attr.meta.path().get_ident() {
                        match attr_id.to_string().as_str() {
                            "check" => {
                                implementation.extend(quote! {
                                    if !::config_checker::check_config(&self.#id) {
                                        ret = false;
                                        println!("{} {} From field `{}` of `{}`", "NOTE: ".blue(), "\u{21b3}", stringify!(#id), stringify!(#struct_identifier));
                                    }
                                });
                            },
                            "ge" | "gt" | "le" | "lt" | "eq" | "ne" => {
                                let op = get_operator(&attr_id);
                                let cond;
                                let r;
                                if let Ok(l) = &attr.meta.require_list() {
                                    r = l.tokens.clone();
                                    cond = quote! {self.#id #op #r};
                                } else {
                                    panic!("{} should be a list (=)", attr_id.to_string());
                                }
                                implementation.extend(quote! {
                                    if #cond {
                                        println!("{} Field `{}` of `{}` should be {} `{}`", "ERROR:".red(), stringify!(#id), stringify!(#struct_identifier), stringify!(#op), stringify!(#r));
                                        ret = false;
                                    }
                                });
                            },
                            "inside" => {
                                let r = &attr.meta.require_list().expect("in attribute should be a list").tokens;
                                implementation.extend(quote! {
                                    // println!(stringify!(#r));
                                    let v = vec![#r];
                                    if !v.contains(&self.#id.as_str()) {
                                        println!("{} Field `{}` of `{}` be equal to one of the values in [{}]", "ERROR:".red(), stringify!(#id), stringify!(#struct_identifier), stringify!(#r));
                                        ret = false;
                                    }
                                });
                            },
                            "or" => {
                                let r = &attr.meta.require_list().expect("in attribute should be a list").tokens;
                                
                            }
                            
                            &_ => {},
                        }
                    }
                }

            }

        },
        _ => unimplemented!()
    }

    quote! {
        #[automatically_derived]
        impl ::config_checker::ConfigCheckable for #struct_identifier {
            fn check(&self) -> bool {
                use colored::Colorize;
                let mut ret = true;
                #implementation
                ret
            }
        }
    }.into()
}


fn get_operator(s: &Ident) -> TokenStream2 {
    match s.to_string().as_str() {
        "gt" => TokenStream2::from_str(">"),
        "ge" => TokenStream2::from_str(">="),
        "lt" => TokenStream2::from_str("<"),
        "le" => TokenStream2::from_str("<="),
        "eq" => TokenStream2::from_str("=="),
        "ne" => TokenStream2::from_str("!="),
        &_ => panic!("Should not be called for other operators"),
    }.unwrap()
}