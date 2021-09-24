//! # Blizzard Engine Macros
//!
//! This crate is for derivative macros for the blizzard engine, to reduce code duplication and make the engine easier for users.

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{Data, DeriveInput, Fields};

/// Get the type of values in a hash map
macro_rules! hash_map_value_type {
    ($x:expr) => {{
        let s = $x
            .to_string()
            .replace("HashMap", "")
            .replace("<", "")
            .replace(">", "")
            .replace("u32,", "")
            .replace(" ", "");

        let s = format!("struct Component {{ field : {}, }}", s);
        let s: &str = &s[..];

        let h: DeriveInput = syn::parse_str(s).expect("Cannot create a meta-struct");
        h
    }};
}

#[proc_macro_derive(ComponentRegistry)]
/// Macro for generating a component registry
pub fn component_registry_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Could not parse code");
    impl_component_registry(&ast)
}

/// Try creating a ComponentRegistry from derivation
fn impl_component_registry(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut gen = None;
    let data = &ast.data;
    if let Data::Struct(data) = data {
        if let Fields::Named(fields) = &data.fields {
            if let Some(field) = fields.named.first() {
                let component_type = &field.ty;
                let component_type = quote!(#component_type);
                let component = hash_map_value_type!(component_type);
                let data = &component.data;
                if let Data::Struct(data) = data {
                    if let Fields::Named(fields) = &data.fields {
                        if let Some(field) = fields.named.first() {
                            let component_type = &field.ty;
                            gen = Some(quote! {
                                    impl ComponentRegistry<#component_type> for #name {
                                        fn new() -> Self {
                                            Self {
                                                components: HashMap::new(),
                                            }
                                        }
                                        fn add(&mut self, entity: u32, component: #component_type) {
                                            self.components.insert(entity, component);
                                        }
                                        fn add_many(&mut self, entities: &Vec<u32>, component: #component_type) {
                                            for entity in entities.iter() {
                                                self.components.insert(*entity, component);
                                            }
                                        }
                                        fn remove(&mut self, entity: u32) {
                                            self.components.remove(&entity);
                                        }
                                        fn get(&self, entity: u32) -> Option<&#component_type> {
                                            self.components.get(&entity)
                                        }
                                    }
                            });
                        }
                    }
                }
            }
        }
    }
    return match gen {
        Some(gen) => gen.into(),
        None => panic!("Cannot derive into a ComponentRegistry"),
    };
}
