extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::Data;

#[proc_macro_derive(Check, attributes(check))]
pub fn derive_check_config(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let struct_identifier = &input.ident;

    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

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
                                        let mut check_result = ::config_checker::__check_config(&self.#id, depth+1);
                                        check_result.__push_stack(format!("Field `{}` of `{}`", stringify!(#id), stringify!(#struct_identifier)));
                                        ret.__concatenate(check_result);
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
                                let mut check_result = ::config_checker::__check_config(o, depth+1);
                                check_result.__push_stack(format!("Field `{}` of `{}`", stringify!(#id), stringify!(#struct_identifier)));
                                ret.__concatenate(check_result);
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
        impl #impl_generics ::config_checker::ConfigCheckable for #struct_identifier #ty_generics #where_clause {
            fn check(&self) -> ::config_checker::CheckResult {
                self.__tree_check(0)
            }

            fn __tree_check(&self, depth: usize) -> ::config_checker::CheckResult {
                use colored::Colorize;
                // use ::config_checker::*;
                let depth_space = vec!["| "; depth].join("");
                let mut ret = ::config_checker::CheckResult::new();
                #implementation;

                let mut check_result = ::config_checker::__CheckBranching::<Self>::call_do_check(self);
                check_result.__push_stack(format!("Type `{}`", stringify!(#struct_identifier)));
                ret.__concatenate(check_result);
                ret
            }
        }
    }
    .into()
}
