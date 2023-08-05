extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use paste::paste;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Expr, ItemStruct, Lit, Meta, Token};

use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use std::string::String;

type Metas = Punctuated<Meta, Token![,]>;

fn parse_lits_from_metas(metas: Metas) -> HashMap<String, Lit> {
    let parse_to_lit = |value_: &Expr| -> Lit {
        match value_ {
            Expr::Lit(value) => value.lit.clone(),
            _ => unreachable!(),
        }
    };

    let mut map = HashMap::new();
    for meta in metas {
        let name_value = meta.require_name_value().unwrap();
        let ident = name_value.path.get_ident().unwrap();
        let value = &name_value.value;

        // can be other expr?
        map.insert(ident.to_string(), parse_to_lit(value));
    }

    map
}

fn get_u32_from_lit(lit: &Lit) -> u32 {
    if let Lit::Int(l) = lit {
        l.base10_parse::<u32>().unwrap()
    } else {
        unreachable!()
    }
}

fn get_str_from_lit(lit: &Lit) -> String {
    if let Lit::Str(l) = lit {
        l.value()
    } else {
        unreachable!()
    }
}

macro_rules! base {
    ($real: tt) => {
        paste! {
            #[proc_macro_attribute]
            pub fn $real(attr: TokenStream, item: TokenStream) -> proc_macro::TokenStream {
                // processing declaration
                let item_struct: ItemStruct = syn::parse(item.clone()).unwrap();
                let ident = item_struct.ident;

                // processing trait
                let metas = Metas::parse_terminated.parse(attr).unwrap();

                // parse binding location
                let lits = parse_lits_from_metas(metas);
                let set: u32 = get_u32_from_lit(lits.get("set").unwrap());
                let binding: u32 = get_u32_from_lit(lits.get("binding").unwrap());

                let tkn_trait = proc_macro::TokenStream::from(quote! {
                    impl DescriptorStruct for #ident {
                        fn get_type(&self) -> VkDescriptorType {
                            VkDescriptorType::[<VK_DESCRIPTOR_TYPE_ $real:snake:upper>]
                        }

                        fn get_set(&self) -> u32 {
                            #set
                        }

                        fn get_binding(&self) -> u32 {
                            #binding
                        }
                    }
                });

                let mut tokens = proc_macro::TokenStream::new();
                tokens.extend(item);
                tokens.extend(tkn_trait);
                tokens
            }
        }
    };
}

// all type
base!(sampler);
base!(combined_image_sampler);
base!(sampled_image);
base!(storage_image);
base!(uniform_texel_buffer);
base!(storage_texel_buffer);
base!(uniform_buffer);
base!(storage_buffer);
base!(uniform_buffer_dynamic);
base!(storage_buffer_dynamic);
base!(input_attachment);
base!(inline_uniform_block);
base!(acceleration_structure_block);
// base!(acceleration_structure_khr);