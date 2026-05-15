use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericArgument, PathArguments, Type};

#[proc_macro_derive(ImmutableUpdate)]
pub fn derive_immutable_update(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let methods = if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            fields
                .named
                .iter()
                .flat_map(|field| {
                    let field_name = &field.ident.as_ref().unwrap();
                    let field_type = &field.ty;

                    let setter_name =
                        syn::Ident::new(&format!("with_{}", field_name), field_name.span());
                    let getter_name = field_name; // Getter has same name as field

                    let getter = quote! {
                        pub fn #getter_name(&self) -> &#field_type {
                            &self.#field_name
                        }
                    };

                    let setter = if let Some(inner_type) = extract_rc_inner_type(field_type) {
                        // Check if field is Rc<T>
                        if is_string_type(&inner_type) {
                            // For Rc<String>, accept impl Into<String>
                            quote! {
                                pub fn #setter_name(&self, value: impl Into<String>) -> Self {
                                    Self {
                                        #field_name: std::rc::Rc::new(value.into()),
                                        ..self.clone()
                                    }
                                }
                            }
                        } else {
                            // For other Rc<T>, accept T
                            quote! {
                                pub fn #setter_name(&self, value: #inner_type) -> Self {
                                    Self {
                                        #field_name: std::rc::Rc::new(value),
                                        ..self.clone()
                                    }
                                }
                            }
                        }
                    } else if let Some(inner_type) = extract_arc_inner_type(field_type) {
                        // Check if field is Arc<T>
                        if is_string_type(&inner_type) {
                            // For Arc<String>, accept impl Into<String>
                            quote! {
                                pub fn #setter_name(&self, value: impl Into<String>) -> Self {
                                    Self {
                                        #field_name: std::sync::Arc::new(value.into()),
                                        ..self.clone()
                                    }
                                }
                            }
                        } else {
                            // For other Arc<T>, accept T
                            quote! {
                                pub fn #setter_name(&self, value: #inner_type) -> Self {
                                    Self {
                                        #field_name: std::sync::Arc::new(value),
                                        ..self.clone()
                                    }
                                }
                            }
                        }
                    } else {
                        // For non-Rc fields, accept the type directly
                        quote! {
                            pub fn #setter_name(&self, value: #field_type) -> Self {
                                Self {
                                    #field_name: value,
                                    ..self.clone()
                                }
                            }
                        }
                    };

                    vec![getter, setter]
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let expanded = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            pub fn to_rc(self) -> std::rc::Rc<Self> {
                std::rc::Rc::new(self)
            }

            pub fn to_arc(self) -> std::sync::Arc<Self> {
                std::sync::Arc::new(self)
            }

            #(#methods)*
        }
    };

    TokenStream::from(expanded)
}

// Helper functions
fn extract_rc_inner_type(ty: &Type) -> Option<&Type> {
    extract_smart_pointer_inner_type(ty, "Rc")
}

fn extract_arc_inner_type(ty: &Type) -> Option<&Type> {
    extract_smart_pointer_inner_type(ty, "Arc")
}

fn extract_smart_pointer_inner_type<'a>(ty: &'a Type, wrapper: &str) -> Option<&'a Type> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == wrapper {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                        return Some(inner_type);
                    }
                }
            }
        }
    }
    None
}

fn is_string_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "String";
        }
    }
    false
}
