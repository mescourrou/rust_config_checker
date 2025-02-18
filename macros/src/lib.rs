extern crate proc_macro;

use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::{Group, TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens};
use syn::{Data, Ident};

#[proc_macro_derive(Check, attributes(check, convert))]
pub fn derive_check_config(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let struct_identifier = &input.ident;

    let mut implementation = TokenStream2::new();
    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            for field in fields {
                let id = field.ident.as_ref().unwrap().to_token_stream().clone();
                let mut accessible_id = id.clone();
                for attr in &field.attrs {
                    if let Some(attr_id) = &attr.meta.path().get_ident() {
                        match attr_id.to_string().as_str() {
                            "convert" => {
                                if let Ok(l) = &attr.meta.require_list() {
                                    let extension = l.tokens.clone();
                                    accessible_id.extend(quote! {.#extension()});
                                } else {
                                    panic!("convert should be a list ()");
                                }
                            }
                            "check" => {
                                if let Ok(l) = &attr.meta.require_list() {
                                    let expr = l.tokens.clone();
                                    let cond =
                                        evaluate_expr(&expr, &accessible_id, struct_identifier)
                                            .first()
                                            .expect("Error")
                                            .clone();
                                    implementation.extend(quote! {
                                        if !(#cond) {
                                        // if !(false) {
                                            // println!(stringify!(#cond));
                                            println!("{} {depth_space}In field `{}` of `{}`", "ERROR:".red(), stringify!(#id), stringify!(#struct_identifier));
                                            ret = false;
                                        }
                                    });
                                } else if let Ok(_) = &attr.meta.require_path_only() {
                                    implementation.extend(quote! {
                                        if !::config_checker::__check_config(&self.#id, depth+1) {
                                            ret = false;
                                            println!("{} {depth_space}  {} From field `{}` of `{}`", "NOTE: ".blue(), "\u{21b3}", stringify!(#id), stringify!(#struct_identifier));
                                        }
                                    });
                                } else {
                                    panic!("`check` should be a path (for sub-struct check) or a list (as a function)");
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
                if !variant.fields.is_empty() {
                    arms.extend(quote! {
                        #struct_identifier::#id(o) => {
                            if !::config_checker::__check_config(o, depth+1) {
                                ret = false;
                                println!("{} {depth_space}  {} From field `{}` of `{}`", "NOTE: ".blue(), "\u{21b3}", stringify!(#id), stringify!(#struct_identifier));
                            }
                        },
                    });
                }
            }
            implementation.extend(quote!{match &self {
                #arms
                _ => {}
            };});
        },
        _ => unimplemented!(),
    }

    quote! {
        #[automatically_derived]
        impl ::config_checker::ConfigCheckable for #struct_identifier {
            fn check(&self) -> bool {
                self.__check(0)
            }

            fn __check(&self, depth: usize) -> bool {
                use colored::Colorize;
                // use ::config_checker::*;
                let depth_space = String::from_utf8(vec![b' '; depth*2]).unwrap();
                let mut ret = true;
                #implementation;
                ret
            }
        }
    }
    .into()
}

fn get_operator(s: &TokenStream2) -> TokenStream2 {
    match s.to_string().as_str() {
        "gt" => TokenStream2::from_str(">"),
        "ge" => TokenStream2::from_str(">="),
        "lt" => TokenStream2::from_str("<"),
        "le" => TokenStream2::from_str("<="),
        "eq" => TokenStream2::from_str("=="),
        "ne" => TokenStream2::from_str("!="),
        "and" => TokenStream2::from_str("&&"),
        "or" => TokenStream2::from_str("||"),
        &_ => panic!("Should not be called for other operators ({})", s),
    }
    .unwrap()
}

fn evaluate_expr(
    expr: &TokenStream2,
    field_id: &TokenStream2,
    struct_identifier: &Ident,
) -> Vec<TokenStream2> {
    let mut cond = vec![TokenStream2::new()];
    let mut ident = TokenStream2::new();
    let mut point_before = false;
    let mut commas: u8 = 0;
    for node in expr.clone().into_iter() {
        match &node {
            TokenTree::Group(g) => {
                cond.last_mut().unwrap().extend(evaluate_ident(
                    &ident.clone(),
                    g,
                    field_id,
                    struct_identifier,
                ));
                ident = TokenStream2::new();
            }
            TokenTree::Ident(i) => {
                if point_before {
                    if !ident.is_empty() {
                        ident.extend(quote! {.#i});
                    }
                } else if commas == 2 {
                    if !ident.is_empty() {
                        ident.extend(quote! {::#i});
                    }
                } else {
                    ident = i.to_token_stream();
                }
                point_before = false;
                commas = 0;
            }
            TokenTree::Literal(lit) => {
                cond.last_mut().unwrap().extend(quote! {#lit});
            }
            TokenTree::Punct(p) => {
                match p.as_char() {
                    '.' => {
                        point_before = true;
                    }
                    ',' => {
                        cond.last_mut().unwrap().extend(ident);
                        cond.push(TokenStream2::new());
                        ident = TokenStream2::new();
                    }
                    ':' => {
                        commas += 1;
                        if commas > 2 {
                            panic!("::: is not valid");
                        }
                    }
                    _ => {}
                };
            }
        }
    }
    if !ident.is_empty() {
        cond.last_mut().unwrap().extend(ident.to_token_stream());
    }

    cond
}

fn evaluate_ident(
    ident: &TokenStream2,
    group: &Group,
    field_id: &TokenStream2,
    struct_identifier: &Ident,
) -> Result<TokenStream2, ()> {
    match ident.to_string().as_str() {
        // Comparison operators
        "ge" | "gt" | "le" | "lt" | "eq" | "ne" => {
            let op = get_operator(&ident);
            let r = group.stream();
            let r = evaluate_expr(&r, field_id, struct_identifier);

            assert!(
                r.len() > 0,
                "{} should have at least 1 parameter",
                ident.to_string()
            );
            let lhs;
            let rhs;
            if r.len() == 1 {
                lhs = quote! { self.#field_id };
                rhs = r[0].clone();
            } else {
                lhs = r[0].clone();
                rhs = r[1].clone();
            }
            let cond = quote! {#lhs #op #rhs};
            Ok(cond)
        }
        // List operators (replacing the delimiter)
        "and" | "or" => {
            let r = group.stream();
            let r = evaluate_expr(&r, field_id, struct_identifier);
            let mut cond = TokenStream2::new();
            let op = get_operator(&ident);
            for c in r {
                if !cond.is_empty() {
                    cond.extend(quote! { #op });
                }
                cond.extend(quote! {(#c)});
            }
            Ok(cond)
        }
        "inside" => {
            let r = group.stream();
            let cond = quote! { vec![#r].contains(&self.#field_id) };

            Ok(cond)
        }
        "if" => {
            let r = group.stream();
            let r = evaluate_expr(&r, field_id, struct_identifier);
            assert!(
                r.len() <= 3,
                "If operator should be maximum with 3 parameters if(cond, if_yes [, if_no])"
            );
            assert!(
                r.len() >= 2,
                "If operator should be minimum with 2 parameters if(cond, if_yes [, if_no])"
            );
            let mut cond = TokenStream2::new();
            let if_cond = &r[0];
            let if_true = &r[1];
            cond.extend(quote! { (#if_cond && #if_true)});
            if r.len() > 2 {
                let if_false = &r[2];
                cond.extend(quote! { || #if_false });
            }
            Ok(quote! {(#cond)})
        }
        "is_enum" => {
            let r = group.stream();
            let r = evaluate_expr(&r, field_id, struct_identifier);
            assert!(r.len() >= 1, "is_enum operator should have 1 or 2 parameters: is_enum([field to test,] Enum::Value)");
            assert!(r.len() <= 2, "is_enum operator should have 1 or 2 parameters: is_enum([field to test,] Enum::Value)");
            let lhs;
            let rhs;
            if r.len() == 1 {
                rhs = quote! { self.#field_id };
                lhs = r[0].clone();
            } else {
                rhs = r[0].clone();
                lhs = r[1].clone();
            }
            let cond = quote! {(match #rhs { #lhs => true, _=> false})};
            Ok(cond)
        }
        &_ => {
            Ok(quote! {#ident #group}) // For enum(_)
        }
    }
}
