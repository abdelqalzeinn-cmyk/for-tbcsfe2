extern crate proc_macro;

use darling::FromAttributes;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, GenericArgument, PathArguments, Type, parse_macro_input};

#[derive(Debug, Clone, FromAttributes, Default)]
#[darling(default, attributes(rw))]
struct Opts {
    gvcc: bool,
    min_gv: Option<u32>,
    max_gv: Option<u32>,
    en: Option<bool>,
    jp: Option<bool>,
    kr: Option<bool>,
    tw: Option<bool>,
    with: Option<String>,
}

#[derive(Debug, Copy, Clone, FromAttributes, Default)]
#[darling(default, attributes(rw))]
struct StructOpts {
    end_assert: Option<u32>,
}

impl Opts {
    fn is_empty(&self) -> bool {
        self.min_gv.is_none()
            && self.max_gv.is_none()
            && self.en.is_none()
            && self.jp.is_none()
            && self.kr.is_none()
            && self.tw.is_none()
    }
}

#[proc_macro_derive(Readable, attributes(rw))]
pub fn readable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let fields = match input.data {
        Data::Struct(ref data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Readable can only be derived for structs with named fields"),
        },
        _ => panic!("Readable can only be derived for structs"),
    };

    let struct_opts = StructOpts::from_attributes(&input.attrs).expect("Wrong options");

    let mut all_empty = true;

    let mut read_field_impls = Vec::with_capacity(fields.len());

    for field in fields {
        let opts = Opts::from_attributes(&field.attrs).expect("Wrong options");
        let field_name = &field.ident;
        let field_type = &field.ty;

        if !opts.is_empty() || opts.gvcc {
            all_empty = false;
        }

        let min_gv = opts.min_gv.unwrap_or(0);
        let max_gv = opts.max_gv.unwrap_or(u32::MAX);
        let en = opts.en.unwrap_or(true);
        let jp = opts.jp.unwrap_or(true);
        let kr = opts.kr.unwrap_or(true);
        let tw = opts.tw.unwrap_or(true);
        let gvcc = opts.gvcc;

        let val = if let Some(ref with) = opts.with {
            syn::parse_str::<Type>(with).unwrap()
        } else {
            field_type.clone()
        };

        let line = if gvcc {
            quote! {
                crate::stream::NewResultCtx::add_context(<#val as crate::stream::Readable>::read(reader, args), || format!("read {} for {}", stringify!(#field_name), stringify!(#name)))?.into()
            }
        } else {
            quote! {
                crate::stream::NewResultCtx::add_context(<#val as crate::stream::Readable>::read(reader, ()), || format!("read {} for {}", stringify!(#field_name), stringify!(#name)))?.into()
            }
        };

        let data = if opts.is_empty() {
            quote! {
                #field_name: #line,
            }
        } else {
            quote! {
                #field_name: {
                    if args.gv.0 >= #min_gv && args.gv.0 <= #max_gv && {
                        match args.cc {
                            crate::country_code::CountryCode::En => #en,
                            crate::country_code::CountryCode::Jp => #jp,
                            crate::country_code::CountryCode::Kr => #kr,
                            crate::country_code::CountryCode::Tw => #tw,
                        }
                    }
                    {
                        #line
                    }
                    else {
                        <#field_type>::default()
                    }
                },
            }
        };

        read_field_impls.push(data);
    }

    let assertable_toks = if let Some(assertable) = struct_opts.end_assert {
        quote! {
            let pos = std::io::Seek::stream_position(reader)?;
            let name = stringify!(#name);
            let assertable = #assertable;
            let value: u32 = crate::stream::NewResultCtx::add_context(crate::stream::ReadableNoOptions::read_no_opts(reader), || format!("read u32 for end assert for {name}"))?;


            if value != #assertable {
                return Err(crate::stream::StreamError::new(std::io::Error::other(format!("assertion error. expected: {assertable}, got {value} for {name}")), pos));
            }
        }
    } else {
        quote! {}
    };

    let expanded = if all_empty {
        quote! {
            impl crate::stream::Readable for #name {
                type Args<'a> = ();
                fn read<R: std::io::Read + std::io::Seek>(
                    reader: &mut R,
                    _args: Self::Args<'_>,
                ) -> crate::stream::StreamResult<Self> {
                    let val = Ok(Self {
                        #(#read_field_impls)*
                    });

                    #assertable_toks

                    val
                }
            }
        }
    } else {
        quote! {
            impl crate::stream::Readable for #name {
                type Args<'a> = crate::save::GVCC;
                fn read<R: std::io::Read + std::io::Seek>(
                    reader: &mut R,
                    args: Self::Args<'_>,
                ) -> crate::stream::StreamResult<Self> {
                    let val = Ok(Self {
                        #(#read_field_impls)*
                    });

                    #assertable_toks

                    val
                }
            }
        }
    };

    TokenStream::from(expanded)
}
#[proc_macro_derive(Writable, attributes(rw))]
pub fn writeable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match input.data {
        Data::Struct(ref data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Writeable can only be derived for structs with named fields"),
        },
        _ => panic!("Writeable can only be derived for structs"),
    };

    let mut all_empty = true;

    let struct_opts = StructOpts::from_attributes(&input.attrs).expect("Wrong options");

    let mut write_field_impls = Vec::with_capacity(fields.len());

    for field in fields {
        let opts = Opts::from_attributes(&field.attrs).expect("Wrong options");
        let field_name = &field.ident;
        let field_type = &field.ty;
        let min_gv = opts.min_gv.unwrap_or(0);
        let max_gv = opts.max_gv.unwrap_or(u32::MAX);
        let en = opts.en.unwrap_or(true);
        let jp = opts.jp.unwrap_or(true);
        let kr = opts.kr.unwrap_or(true);
        let tw = opts.tw.unwrap_or(true);
        let gvcc = opts.gvcc;

        if !opts.is_empty() || gvcc {
            all_empty = false;
        }

        let val = if let Some(ref with) = opts.with {
            syn::parse_str::<Type>(with).unwrap()
        } else {
            field_type.clone()
        };

        let line = if gvcc {
            quote! {
                crate::stream::NewResultCtx::add_context(<#val as crate::stream::Writable>::write(self.#field_name.into(), writer, args), || format!("write {} for {}", stringify!(#field_name), stringify!(#name)))?;
            }
        } else {
            quote! {
                crate::stream::NewResultCtx::add_context(<#val as crate::stream::Writable>::write(self.#field_name.into(), writer, ()), || format!("write {} for {}", stringify!(#field_name), stringify!(#name)))?;
            }
        };

        let data = if opts.is_empty() {
            quote! {
                #line
            }
        } else {
            quote! {
                if args.gv.0 >= #min_gv && args.gv.0 <= #max_gv && {
                    match args.cc {
                        crate::country_code::CountryCode::En => #en,
                        crate::country_code::CountryCode::Jp => #jp,
                        crate::country_code::CountryCode::Kr => #kr,
                        crate::country_code::CountryCode::Tw => #tw,
                    }
                }
                {
                    #line
                }
            }
        };

        write_field_impls.push(data);
    }

    let assertable_toks = if let Some(assertable) = struct_opts.end_assert {
        quote! {
            crate::stream::WritableNoOptions::write_no_opts(#assertable, writer)?;
        }
    } else {
        quote! {}
    };

    let expanded = if all_empty {
        quote! {
            impl crate::stream::Writable for #name {
                type Args<'a> = ();
                fn write<W: std::io::Write + std::io::Seek>(
                    self,
                    writer: &mut W,
                    _args: Self::Args<'_>,
                ) -> crate::stream::StreamResult<()> {
                    #(#write_field_impls)*
                    #assertable_toks
                    Ok(())
                }
            }
        }
    } else {
        quote! {
            impl crate::stream::Writable for #name {
                type Args<'a> = crate::save::GVCC;
                fn write<W: std::io::Write + std::io::Seek>(
                    self,
                    writer: &mut W,
                    args: Self::Args<'_>,
                ) -> crate::stream::StreamResult<()> {
                    #(#write_field_impls)*
                    #assertable_toks
                    Ok(())
                }
            }
        }
    };

    TokenStream::from(expanded)
}
