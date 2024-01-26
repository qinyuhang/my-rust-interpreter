use proc_macro::TokenStream;
use quote::quote;
use syn;

/// add derive(Debug, Clone) to struct
/// impl as_any for struct
///
/// this macro is inspired by swc project
#[proc_macro_attribute]
pub fn ast_node(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    assert!(!args.is_empty());

    let args_clone = args.clone();
    // let extend_type = syn::parse_macro_input!(args_clone);
    let ipt = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ipt.ident;

    let add_impl = match args_clone.to_string().as_str() {
        "Node" => {
            // println!("start decorate for Node");
            quote! {
                impl Node for #name {
                    fn token_literal(&self) -> String {
                        self.token.literal.clone()
                    }
                    fn as_any(&self) -> &dyn Any {
                        self
                    }
                }
            }
        }
        "Expression" => {
            // println!("start decorate for Expression");
            quote! {
                impl Node for #name {
                    fn token_literal(&self) -> String {
                        self.token.literal.clone()
                    }
                    fn as_any(&self) -> &dyn Any {
                        self
                    }
                }
                impl Expression for #name {
                    fn expression_node(&self) {
                        todo!()
                    }
                    fn upcast(&self) -> &dyn Node {
                        self
                    }
                }
            }
        }
        "Statement" => {
            // println!("start decorate for Statement");
            quote! {
                impl Node for #name {
                    fn token_literal(&self) -> String {
                        self.token.literal.clone()
                    }
                    fn as_any(&self) -> &dyn Any {
                        self
                    }
                }
                impl Expression for #name {
                    fn expression_node(&self) {
                        todo!()
                    }
                    fn upcast(&self) -> &dyn Node {
                        self
                    }
                }
                impl Statement for #name {
                    fn statement_node(&self) {
                        todo!()
                    }
                    fn upcast(&self) -> &dyn Node {
                        self
                    }
                }
            }
        }
        _ => {
            // println!("start decorate for {} {}", args_clone, syn::parse::<syn::DeriveInput>(args_clone.clone()).is_ok() );
            quote! {}
        }
    };
    // println!("add impl: {}", add_impl);
    let s = quote! {
        #[derive(Debug, Clone, Eq, PartialEq)]
        #ipt
        #add_impl
    };
    // println!("ast_node {} {}", args, name);
    // println!("after decorater ast_node {} {}", &s, &add_impl);

    proc_macro::TokenStream::from(s)
}

/// add derive(Debug, Clone) to struct
#[proc_macro_attribute]
pub fn object(args: TokenStream, input: TokenStream) -> TokenStream {
    // let args_clone = args.clone();
    let ipt = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ipt.ident;
    let attr_args = syn::parse_macro_input!(args as syn::Ident);

    let s = quote! {
        #[derive(Debug, Clone)]
        #ipt
        impl ObjectWithoutInspect for #name {
            fn _object_type(&self) -> ObjectType {
                #attr_args
            }
            fn _as_any(&self) -> &dyn Any {
                self
            }
        }
        impl Object for #name {}
        impl std::fmt::Display for  #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.inspect())
            }
        }
    };
    TokenStream::from(s)
}

#[proc_macro_attribute]
pub fn object_with_try_from(args: TokenStream, input: TokenStream) -> TokenStream {
    let ipt = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ipt.ident;
    let attr_args = syn::parse_macro_input!(args as syn::Ident);
    let s = quote! {
        #ipt
        impl TryFrom<Rc<dyn Object>> for #name {
            type Error = String;

            fn try_from(value: Rc<dyn Object>) -> Result<Self, Self::Error> {
                if let Some(v) = value.as_any().downcast_ref::<#name>() {
                    return Ok((*v).clone());
                }
                Err(format!("cannot convert {} into {}", &value, #attr_args))
            }
        }
    };
    TokenStream::from(s)
}
// #[proc_macro]
// macro_rules! hashmap {
//     ($( $key: expr => $val: expr ),*) => {{
//          let mut map = ::std::collections::HashMap::new();
//          $( map.insert($key, $val); )*
//          map
//     }}
// }
