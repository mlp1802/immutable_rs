use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericArgument, PathArguments, Type};

#[proc_macro_derive(ImmutableUpdate)]
pub fn derive_immutable_update(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

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
        impl #struct_name {
            #(#methods)*
        }
    };

    TokenStream::from(expanded)
}

// Helper functions
fn extract_rc_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Rc" {
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
