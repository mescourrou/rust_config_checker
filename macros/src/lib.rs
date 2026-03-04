extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::Data;

#[proc_macro_derive(Check, attributes(check))]
pub fn derive_check_config(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let struct_identifier = &input.ident;

    let mut implementation = TokenStream2::new();
    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            for field in fields {
                let id = field.ident.as_ref().unwrap().to_token_stream().clone();
                // let mut accessible_id = id.clone();
                for attr in &field.attrs {
                    if let Some(attr_id) = &attr.meta.path().get_ident() {
                        match attr_id.to_string().as_str() {
                            // "convert" => {
                            //     if let Ok(l) = &attr.meta.require_list() {
                            //         let extension = l.tokens.clone();
                            //         accessible_id.extend(quote! {.#extension()});
                            //     } else {
                            //         panic!("convert should be a list ()");
                            //     }
                            // }
                            "check" => {
                                if let Ok(_) = &attr.meta.require_path_only() {
                                    implementation.extend(quote! {
                                        if let Err(e) = ::config_checker::__check_config(&self.#id, depth+1) {
                                            if ret.is_ok() {
                                                ret = Err(String::new());
                                            }
                                            ret = Err(ret.err().unwrap() + format!("{} {depth_space}From field `{}` of `{}`:\n{e}", "NOTE :".blue(), stringify!(#id), stringify!(#struct_identifier)).as_str());
                                        }
                                    });
                                } else {
                                    panic!("`check` should be a path (for sub-struct check)");
                                }
                            }
                            &_ => {}
                        }
                    }
                }
            }
        },
        Data::Enum(syn::DataEnum { variants, ..}) => {
            let mut arms = TokenStream2::new();
            for variant in variants {
                let id = variant.ident.clone();
                // Only check inside enum variant when #[check] attribute is present (path-only)
                let mut should_check = false;
                for attr in &variant.attrs {
                    if let Some(attr_id) = &attr.meta.path().get_ident() {
                        if attr_id.to_string().as_str() == "check" {
                            if let Ok(_) = &attr.meta.require_path_only() {
                                should_check = true;
                            } else {
                                panic!("`check` on enum variant should be a path-only attribute");
                            }
                        }
                    }
                }

                if should_check {
                    if !variant.fields.is_empty() {
                        arms.extend(quote! {
                            #struct_identifier::#id(o) => {
                                if let Err(e) = ::config_checker::__check_config(o, depth+1) { 
                                    if ret.is_ok() {
                                        ret = Err(String::new());
                                    }
                                    ret = Err(ret.err().unwrap() + format!("{} {depth_space}From field `{}` of `{}`:\n{e}", "NOTE :".blue(), stringify!(#id), stringify!(#struct_identifier)).as_str());
                                }
                            },
                        });
                    }
                }
            }
            // Only generate match statement if there are arms to match
            if !arms.is_empty() {
                implementation.extend(quote!{match &self {
                    #arms
                    _ => {}
                };});
            }
        },
        _ => unimplemented!(),
    }

    quote! {
        #[automatically_derived]
        impl ::config_checker::ConfigCheckable for #struct_identifier {
            fn check(&self) -> Result<(), String> {
                self.__tree_check(0)
            }

            fn __tree_check(&self, depth: usize) -> Result<(), String> {
                use colored::Colorize;
                // use ::config_checker::*;
                let depth_space = vec!["| "; depth].join("");
                let mut ret = Ok(());
                #implementation;

                if let Err(e) = ::config_checker::__CheckBranching::<Self>::call_do_check(self) {
                    if ret.is_ok() {
                        ret = Err(String::new());
                    }
                    for (i, err) in e.into_iter().enumerate() {
                        ret = Err(ret.err().unwrap() + format!("{} {depth_space}{err}\n", "ERROR:".red()).as_str());
                    }
                }
                ret
            }
        }
    }
    .into()
}
