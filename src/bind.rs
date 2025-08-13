/// Generates Rust bindings for JavaScript functions using `wasm_bindgen`.
///
/// The `bind!` macro simplifies the creation of `extern` declarations and safe Rust wrapper functions
/// that handle conversion between Rust and JavaScript types using `serde_wasm_bindgen`.
///
/// The macro supports both synchronous and asynchronous JavaScript functions, with or without return types.
///
/// ### Syntax Variants
///
/// #### Asynchronous with arguments and return value:
/// ```rust
/// bind!(async "/path/to/js/module.js"::function_name(arg1: Arg1Type, arg2: Arg2Type) -> ReturnType);
/// ```
///
/// #### Asynchronous with arguments and no return value:
/// ```rust
/// bind!(async "/path/to/js/module.js"::function_name(arg1: Arg1Type, arg2: Arg2Type));
/// ```
///
/// #### Asynchronous with no arguments and return value:
/// ```rust
/// bind!(async "/path/to/js/module.js"::function_name() -> ReturnType);
/// ```
///
/// #### Asynchronous with no arguments and no return value:
/// ```rust
/// bind!(async "/path/to/js/module.js"::function_name());
/// ```
///
/// #### Synchronous with arguments and return value:
/// ```rust
/// bind!("/path/to/js/module.js"::function_name(arg1: Arg1Type, arg2: Arg2Type) -> ReturnType);
/// ```
///
/// #### Synchronous with arguments and no return value:
/// ```rust
/// bind!("/path/to/js/module.js"::function_name(arg1: Arg1Type, arg2: Arg2Type));
/// ```
///
/// #### Synchronous with no arguments and return value:
/// ```rust
/// bind!("/path/to/js/module.js"::function_name() -> ReturnType);
/// ```
///
/// #### Synchronous with no arguments and no return value:
/// ```rust
/// bind!("/path/to/js/module.js"::function_name());
/// ```
///
/// ### Behavior
/// - Defines a hidden module to hold the `extern` block and JavaScript FFI.
/// - Uses `wasm_bindgen` to catch JS exceptions and wrap them in `Result`.
/// - Uses `serde_wasm_bindgen` to serialize/deserialize parameters and results.
///
/// ### Notes
/// - This macro assumes the JavaScript function being bound has an equivalent signature.
/// - Return values must be `serde`-serializable/deserializable.
/// - JS errors are returned as `JsValue` with stringified error messages.
///
/// ### Example
/// ```rust
/// bind!(async "/js/math.js"::add(a: u32, b: u32) -> u32);
/// let result = add(1, 2).await.unwrap();
/// ```
#[macro_export]
macro_rules! bind {
    (async $file_path:literal::$fn_ident:ident($($arg_ident:ident: $arg_ty:ty),*) -> $ret_ty:ty) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use ::wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub async fn $fn_ident($($arg_ident: JsValue),*) -> ::core::result::Result<JsValue, JsValue>;
                }
            }

            #[inline]
            pub async fn $fn_ident($($arg_ident: $arg_ty),*) -> ::core::result::Result<$ret_ty, ::wasm_bindgen::JsValue> {
                let ret: ::wasm_bindgen::JsValue = [< __ $fn_ident >]::$fn_ident($(::serde_wasm_bindgen::to_value(&$arg_ident).map_err(|e| {
                    ::wasm_bindgen::JsValue::from_str(&e.to_string())
                })?),*).await?;
                ::serde_wasm_bindgen::from_value(ret).map_err(|e| {
                    ::wasm_bindgen::JsValue::from_str(&e.to_string())
                })
            }
        );
    };
    (async $file_path:literal::$fn_ident:ident($($arg_ident:ident: $arg_ty:ty),*)) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub async fn $fn_ident($($arg_ident: JsValue),*) -> Result<JsValue, JsValue>;
                }
            }

            #[inline]
            pub async fn $fn_ident($($arg_ident: $arg_ty),*) -> Result<(), ::wasm_bindgen::JsValue> {
                [< __ $fn_ident >]::$fn_ident($(::serde_wasm_bindgen::to_value(&$arg_ident).map_err(|e| {
                    ::wasm_bindgen::JsValue::from_str(&e.to_string())
                })?),*).await?;
                Ok(())
            }
        );
    };
    (async $file_path:literal::$fn_ident:ident() -> $ret_ty:ty) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use ::wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub async fn $fn_ident() -> Result<JsValue, JsValue>;
                }
            }

            #[inline]
            pub async fn $fn_ident() -> ::core::result::Result<$ret_ty, ::wasm_bindgen::JsValue> {
                ::serde_wasm_bindgen::from_value([< __ $fn_ident >]::$fn_ident().await?).map_err(|e| {
                    ::wasm_bindgen::JsValue::from_str(&e.to_string())
                })
            }
        );
    };
    (async $file_path:literal::$fn_ident:ident()) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use ::wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub async fn $fn_ident() -> Result<JsValue, JsValue>;
                }
            }

            #[inline]
            pub async fn $fn_ident() -> Result<(), JsValue> {
                [< __ $fn_ident >]::$fn_ident().await?;
                Ok(())
            }
        );
    };
    ($file_path:literal::$fn_ident:ident($($arg_ident:ident: $arg_ty:ty),*) -> $ret_ty:ty) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use ::wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub fn $fn_ident($($arg_ident: JsValue),*) -> ::core::result::Result<JsValue, JsValue>;
                }
            }

            pub fn $fn_ident($($arg_ident: $arg_ty),*) -> ::core::result::Result<$ret_ty, ::wasm_bindgen::JsValue> {
                ::serde_wasm_bindgen::from_value([< __ $fn_ident >]::$fn_ident($(::serde_wasm_bindgen::to_value(&$arg_ident).map_err(|e| {
                    ::wasm_bindgen::JsValue::from_str(&e.to_string())
                })?),*)?).map_err(|e| {
                    ::wasm_bindgen::JsValue::from_str(&e.to_string())
                })
            }
        );
    };
    ($file_path:literal::$fn_ident:ident($($arg_ident:ident: $arg_ty:ty),*)) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use ::wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub fn $fn_ident($($arg_ident: JsValue),*) -> ::core::result::Result<JsValue, JsValue>;
                }
            }

            pub fn $fn_ident($($arg_ident: $arg_ty),*) -> ::core::result::Result<(), ::wasm_bindgen::JsValue> {
                [< __ $fn_ident >]::$fn_ident($(::serde_wasm_bindgen::to_value(&$arg_ident).map_err(|e| {
                    ::wasm_bindgen::JsValue::from_str(&e.to_string())
                })?),*)?;
                Ok(())
            }
        );
    };
    ($file_path:literal::$fn_ident:ident() -> $ret_ty:ty) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use ::wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub fn $fn_ident() -> Result<JsValue, JsValue>;
                }
            }

            #[inline]
            pub fn $fn_ident() -> Result<$ret_ty, ::wasm_bindgen::JsValue> {
                ::serde_wasm_bindgen::from_value([< __ $fn_ident >]::$fn_ident()?).map_err(|e| {
                    ::wasm_bindgen::JsValue::from_str(&e.to_string())
                })
            }
        );
    };
    ($file_path:literal::$fn_ident:ident()) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use ::wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub fn $fn_ident() -> Result<JsValue, JsValue>;
                }
            }

            #[inline]
            pub fn $fn_ident() -> Result<(), JsValue> {
                [< __ $fn_ident >]::$fn_ident()?;
                Ok(())
            }
        );
    };
}

/// Wraps Rust closures into `wasm_bindgen`-compatible `Closure` types.
///
/// This macro simplifies creating `wasm_bindgen::closure::Closure` from closures
/// of various arities and forms (with or without arguments, with or without return values).
///
/// All forms support both `move` and non-`move` closures.
///
/// ### Syntax Variants
///
/// #### No arguments, with return value:
/// ```rust
/// function!(|| -> ReturnType {
///     // body
/// });
/// ```
///
/// #### No arguments, no return value:
/// ```rust
/// function!(|| {
///     // body
/// });
/// ```
///
/// #### With arguments and return value:
/// ```rust
/// function!(|arg1: Type1, arg2: Type2| -> ReturnType {
///     // body
/// });
/// ```
///
/// #### With arguments and no return value:
/// ```rust
/// function!(|arg1: Type1, arg2: Type2| {
///     // body
/// });
/// ```
///
/// Each of the above can be prepended with `move`:
/// ```rust
/// function!(move || { ... });
/// function!(move |x: u8| -> u8 { x });
/// ```
///
/// ### Return
/// - Produces a `wasm_bindgen::closure::Closure` that can be passed to JS APIs.
///
/// ### Example
/// ```rust
/// let closure = function!(move |event: web_sys::Event| {
///     log::info!("Clicked!");
/// });
/// element.set_onclick(Some(closure.as_ref().unchecked_ref()));
/// closure.forget(); // To prevent it from being dropped
/// ```
///
/// ### Notes
/// - Use `.forget()` if you want to keep the closure alive for the lifetime of the page.
/// - Otherwise, it will be dropped once it goes out of scope, potentially causing JS callbacks to fail.
#[macro_export]
macro_rules! function {
    (move || -> $ret_ty:ty $block:block) => {
        ::wasm_bindgen::closure::Closure::wrap(::std::boxed::Box::new(move || -> $ret_ty {
            $block
        }) as ::std::boxed::Box<dyn ::core::ops::FnMut() -> $ret_ty>)
    };
    (move || $block:block) => {
        ::wasm_bindgen::closure::Closure::wrap(::std::boxed::Box::new(move || {
            $block
        }) as ::std::boxed::Box<dyn ::core::ops::FnMut()>)
    };
    (move |$($arg_ident:ident: $arg_ty:ty),*| -> $ret_ty:ty $block:block) => {
        ::wasm_bindgen::closure::Closure::wrap(::std::boxed::Box::new(move |$($arg_ident: $arg_ty),*| -> $ret_ty {
            $block
        }) as ::std::boxed::Box<dyn ::core::ops::FnMut($($arg_ty),*) -> $ret_ty>)
    };
    (move |$($arg_ident:ident: $arg_ty:ty),*| $block:block) => {
        ::wasm_bindgen::closure::Closure::wrap(::std::boxed::Box::new(move |$($arg_ident: $arg_ty),*| {
            $block
        }) as ::std::boxed::Box<dyn ::core::ops::FnMut($($arg_ty),*)>)
    };
    (|| -> $ret_ty:ty $block:block) => {
        ::wasm_bindgen::closure::Closure::wrap(::std::boxed::Box::new(|| -> $ret_ty {
            $block
        }) as ::std::boxed::Box<dyn ::core::ops::FnMut() -> $ret_ty>)
    };
    (|| $block:block) => {
        ::wasm_bindgen::closure::Closure::wrap(::std::boxed::Box::new(|| {
            $block
        }) as ::std::boxed::Box<dyn ::core::ops::FnMut()>)
    };
    (|$($arg_ident:ident: $arg_ty:ty),*| -> $ret_ty:ty $block:block) => {
        ::wasm_bindgen::closure::Closure::wrap(::std::boxed::Box::new(|$($arg_ident: $arg_ty),*| -> $ret_ty {
            $block
        }) as ::std::boxed::Box<dyn ::core::ops::FnMut($($arg_ty),*) -> $ret_ty>)
    };
    (|$($arg_ident:ident: $arg_ty:ty),*| $block:block) => {
        ::wasm_bindgen::closure::Closure::wrap(::std::boxed::Box::new(|$($arg_ident: $arg_ty),*| {
            $block
        }) as ::std::boxed::Box<dyn ::core::ops::FnMut($($arg_ty),*)>)
    };
}

#[cfg(test)]
#[allow(unused)]
mod sig_test {
    use super::*;
    
    fn success() {
        bind!(async "/src/bind.js"::test_0(a: u8, b: u8, c: u8) -> u8);
        bind!(async "/src/bind.js"::test_1(a: u8, b: u8, c: u8));
        bind!(async "/src/bind.js"::test_2() -> u8);
        bind!(async "/src/bind.js"::test_3());        
        bind!("/src/bind.js"::test_4(a: u8, b: u8, c: u8) -> u8);
        bind!("/src/bind.js"::test_5(a: u8, b: u8, c: u8));
        bind!("/src/bind.js"::test_6() -> u8);
        bind!("/src/bind.js"::test_7());

        function!(move || -> u8 {
            200
        });

        function!(move || {
            
        });

        function!(move |foo: u8| -> u8 {
            foo
        });

        function!(move |_foo: u8| {
            
        });

        function!(|| -> u8 {
            200
        });

        function!(|| {

        });

        function!(|foo: u8| -> u8 {
            foo
        });

        function!(|_foo: u8| {

        });
    }
}