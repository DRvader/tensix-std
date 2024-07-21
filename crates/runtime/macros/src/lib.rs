#![deny(warnings)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate core;
extern crate proc_macro2;
#[macro_use]
extern crate syn;

use proc_macro2::Span;
use syn::{parse, spanned::Spanned, FnArg, ItemFn, PathArguments, ReturnType, Type, Visibility};

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    // check the function arguments
    if f.sig.inputs.len() > 0 {
        return parse::Error::new(
            f.sig.inputs.last().unwrap().span(),
            "`#[entry]` function has too many arguments",
        )
        .to_compile_error()
        .into();
    }
    for arg in &f.sig.inputs {
        match arg {
            FnArg::Receiver(_) => {
                return parse::Error::new(arg.span(), "invalid argument")
                    .to_compile_error()
                    .into();
            }
            FnArg::Typed(t) => {
                if !is_simple_type(&t.ty, "usize") {
                    return parse::Error::new(t.ty.span(), "argument type must be usize")
                        .to_compile_error()
                        .into();
                }
            }
        }
    }

    let never_exit = match f.sig.output {
        ReturnType::Default => false,
        ReturnType::Type(_, ref ty) => matches!(**ty, Type::Never(_)),
    };

    let return_u32 = match f.sig.output {
        ReturnType::Default => false,
        ReturnType::Type(_, ref ty) => {
            if let Type::Path(ty) = ty.as_ref() {
                if ty.qself.is_none() && ty.path.segments.len() == 1 {
                    let segment = &ty.path.segments[0];
                    segment.ident == "u32"
                } else {
                    false
                }
            } else {
                false
            }
        }
    };

    let empty_rt = match f.sig.output {
        ReturnType::Default => true,
        ReturnType::Type(..) => false,
    };

    // check the function signature
    let valid_signature = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && (return_u32 || never_exit | empty_rt);

    if !valid_signature {
        return parse::Error::new(
            f.span(),
            "`#[entry]` function must have signature `[unsafe] fn() [-> (! | u32)]`",
        )
        .to_compile_error()
        .into();
    }

    let arg = parse_macro_input!(args as syn::Ident);

    let arg = arg.to_string();

    match arg.as_str() {
        "brisc" | "ncrisc" | "trisc0" | "trisc1" | "trisc2" => {}
        err => {
            return parse::Error::new(Span::call_site(), format!("This attribute requires an argument to be one of (brisc, ncrisc, trisc0, trisc1, trisc2); found {err}"))
                .to_compile_error()
                .into();
        }
    }

    // XXX should we blacklist other attributes?
    let attrs = f.attrs;
    let unsafety = f.sig.unsafety;
    let args = f.sig.inputs;
    let stmts = f.block.stmts;

    let function_name = format!("{arg}_main");
    let mangled_function_name =
        syn::Ident::new(&format!("__tensix_rt__{function_name}"), f.sig.ident.span());
    let complete_name = syn::Ident::new(&format!("complete_{}", arg.as_str()), f.sig.ident.span());
    let no_exit_name = syn::Ident::new(&format!("no_exit_{}", arg.as_str()), f.sig.ident.span());
    let set_pc_name = syn::Ident::new(
        &format!("set_postcode_{}", arg.as_str()),
        f.sig.ident.span(),
    );

    let (extern_def, func) = if never_exit {
        (
            quote! {
                fn #no_exit_name();
            },
            quote! {
                #no_exit_name();
                #(#stmts)*
            },
        )
    } else if return_u32 {
        (
            quote! {
                fn #set_pc_name(pc: u32);
                fn #complete_name();
            },
            quote! {
                let output = (|| {
                    #(#stmts)*
                })();
                #set_pc_name(output);
                #complete_name();
                loop {}
            },
        )
    } else {
        (
            quote! {
                fn #complete_name();
            },
            quote! {
                #(#stmts)*
                #complete_name();
                loop {}
            },
        )
    };

    quote!(
        #[allow(non_snake_case)]
        #[export_name = #function_name]
        #(#attrs)*
        pub #unsafety fn #mangled_function_name(#args) -> ! {
            extern "Rust" {
                #extern_def
            }
            #func
        }
    )
    .into()
}

#[allow(unused)]
fn is_simple_type(ty: &Type, name: &str) -> bool {
    if let Type::Path(p) = ty {
        if p.qself.is_none() && p.path.leading_colon.is_none() && p.path.segments.len() == 1 {
            let segment = p.path.segments.first().unwrap();
            if segment.ident == name && segment.arguments == PathArguments::None {
                return true;
            }
        }
    }
    false
}
