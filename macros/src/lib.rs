extern crate proc_macro;

use std::mem;

use proc_macro::TokenStream;
use quote::{quote};
use syn::{Data, LitStr};

#[proc_macro_derive(Check, attributes(check, ge, gt, le, lt, inside))]
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
                                        println!(stringify!({} {} From field #id of #struct_identifier), "NOTE: ".blue(), "\u{21b3}");
                                    }
                                });
                            },
                            "ge" => {
                                let r = &attr.meta.require_name_value().expect("ge attribute should be name value").value;
                                implementation.extend(quote! {
                                    if self.#id < #r {
                                        println!("{} Field `{}` of `{}` should be greater or equal to `{}`", "ERROR:".red(), stringify!(#id), stringify!(#struct_identifier), stringify!(#r));
                                        ret = false;
                                    }
                                });
                            },
                            "gt" => {
                                let r = &attr.meta.require_name_value().expect("gt attribute should be name value").value;
                                implementation.extend(quote! {
                                    if self.#id <= #r {
                                        println!("{} Field `{}` of `{}` should be greater than `{}`", "ERROR:".red(), stringify!(#id), stringify!(#struct_identifier), stringify!(#r));
                                        ret = false;
                                    }
                                });
                            },
                            "le" => {
                                let r = &attr.meta.require_name_value().expect("le attribute should be name value").value;
                                implementation.extend(quote! {
                                    if self.#id > #r {
                                        println!("{} Field `{}` of `{}` should be less or equal to `{}`", "ERROR:".red(), stringify!(#id), stringify!(#struct_identifier), stringify!(#r));
                                        ret = false;
                                    }
                                });
                            },
                            "lt" => {
                                let r = &attr.meta.require_name_value().expect("lt attribute should be name value").value;
                                implementation.extend(quote! {
                                    if self.#id >= #r {
                                        println!("{} Field `{}` of `{}` should be less than `{}`", "ERROR:".red(), stringify!(#id), stringify!(#struct_identifier), stringify!(#r));
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