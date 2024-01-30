use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Fields};

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
                    fn token_literal(&self) -> std::rc::Rc<String> {
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
                    fn token_literal(&self) -> std::rc::Rc<String> {
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
                    fn token_literal(&self) -> std::rc::Rc<String> {
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

#[proc_macro_attribute]
pub fn ast_node_with_try_from(args: TokenStream, input: TokenStream) -> TokenStream {
    let ipt = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ipt.ident;
    // let attr_args = syn::parse_macro_input!(args as syn::Ident);

    let s = quote! {
        #ipt
        impl TryFrom<Box<&dyn Expression>> for #name {
            type Error = String;

            fn try_from(value: Box<&dyn Expression>) -> std::result::Result<Self, Self::Error> {
                let x = value.as_any();
                if x.is::<Self>() {
                    let x = x.downcast_ref::<Self>().unwrap();
                    return Ok(x.clone());
                }
                if x.is::<ExpressionStatement>() {
                    if let Some(ExpressionStatement {
                        expression: Some(expression),
                        ..
                    }) = x.downcast_ref::<ExpressionStatement>()
                    {
                        if let Ok(r) = Self::try_from(Box::new((*expression).clone().get_expression())) {
                            return Ok(r);
                        }
                    }
                }
                Err(format!("Cannot cast {:?} to {}", value, stringify!(#name)))
            }
        }
    };
    TokenStream::from(s)
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
    // let attr_args = syn::parse_macro_input!(args as syn::Ident);
    let s = quote! {
        #ipt
        impl TryFrom<Rc<dyn Object>> for #name {
            type Error = String;

            fn try_from(value: std::rc::Rc<dyn Object>) -> std::result::Result<Self, Self::Error> {
                if let Some(v) = value.as_any().downcast_ref::<#name>() {
                    return Ok((*v).clone());
                }
                Err(format!("cannot convert {} into {}", &value, stringify!(#name)))
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

#[proc_macro_derive(FromU8)]
pub fn from_u8_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident; // 获取枚举的名称

    let variants = match input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("FromU8 can only be derived for enums"),
    };

    let mut arms = Vec::new();
    let mut discriminant = 0u8;

    for variant in variants {
        let variant_ident = &variant.ident;

        // 检查是否有显式指定的判别值
        if let Fields::Unit = variant.fields {
            if let Some((_, expr)) = &variant.discriminant {
                // 尝试将表达式解析为整数
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(lit_int),
                    ..
                }) = expr
                {
                    discriminant = lit_int.base10_parse().unwrap();
                }
            }
        } else {
            panic!("FromU8 can only be derived for enums with unit variants");
        }

        arms.push(quote! {
            #discriminant => Self::#variant_ident,
        });

        discriminant += 1;
    }

    let expanded = quote! {
        impl From<u8> for #name {
            fn from(value: u8) -> Self {
                match value {
                    #(#arms)*
                    _ => panic!("Invalid OpCode value: {}", value),
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ForAstExpression)]
pub fn for_ast_expression_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident; // 获取枚举的名称

    let variants = match input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("FromU8 can only be derived for enums"),
    };

    let mut arms = Vec::new();

    for variant in variants {
        let variant_ident = &variant.ident;

        arms.push(quote! {
            Self::#variant_ident(a) => a,
        });
    }

    let expanded = quote! {
        impl #name {

            pub fn as_any(&self) -> &dyn std::any::Any {
                match self {
                    #(#arms)*
                }
            }

            pub fn get_expression(&self) -> &dyn Expression {
                match self {
                    #(#arms)*
                }
            }

            pub fn upcast(&self) -> &dyn Node {
                match self {
                    #(#arms)*
                }
            }
        }

    };

    TokenStream::from(expanded)
}
