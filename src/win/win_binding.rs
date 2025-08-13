use ::wasm_bindgen::prelude::*;
use ::std::time;

pub type Cancel = Closure<dyn FnMut()>;

#[inline]
pub fn on_animation_frame<T>(mut event_handler: T) -> Result<Cancel, JsValue>
where
    T: FnMut() + 'static {
    let event_handler_closure: Closure<_> = Closure::wrap(Box::new(move || {
        event_handler();
    }) as Box<dyn FnMut()>);
    let event_handler_closure_ref: &::js_sys::Function = event_handler_closure.as_ref().unchecked_ref();
    let cancel: ::js_sys::Function = js::on_animation_frame(event_handler_closure_ref)?;
    let cancel_closure: Closure<_> = Closure::wrap(Box::new(move || {
        cancel.call0(&JsValue::NULL).ok();
    }) as Box<dyn FnMut()>);
    event_handler_closure.forget();
    Ok(cancel_closure)
}

#[inline]
pub fn on_timeout<T>(duration: time::Duration, mut event_handler: T) -> Result<Cancel, JsValue> 
where 
    T: FnMut() + 'static {
    let ms: u128 = duration.as_millis();
    let ms: f64 = ms as f64;
    let ms: JsValue = ms.into();
    let event_handler_closure: Closure<_> = Closure::wrap(Box::new(move || {
        event_handler();
    }) as Box<dyn FnMut()>);
    let event_handler_closure_ref: &::js_sys::Function = event_handler_closure.as_ref().unchecked_ref();
    let cancel: ::js_sys::Function = js::on_timeout(ms, event_handler_closure_ref)?;
    let cancel_closure: Closure<_> = Closure::wrap(Box::new(move || {
        cancel.call0(&JsValue::NULL).ok();
    }) as Box<dyn FnMut()>);
    event_handler_closure.forget();
    Ok(cancel_closure)
}

#[inline]
pub fn on_interval<T>(duration: time::Duration, mut event_handler: T) -> Result<Cancel, JsValue>
where
    T: FnMut() + 'static {
    let ms: u128 = duration.as_millis();
    let ms: f64 = ms as f64;
    let ms: JsValue = ms.into();
    let event_handler_closure: Closure<_> = Closure::wrap(Box::new(move || {
        event_handler();
    }) as Box<dyn FnMut()>);
    let event_handler_closure_ref: &::js_sys::Function = event_handler_closure.as_ref().unchecked_ref();
    let cancel: ::js_sys::Function = js::on_interval(ms, event_handler_closure_ref)?;
    let cancel_closure: Closure<_> = Closure::wrap(Box::new(move || {
        cancel.call0(&JsValue::NULL).ok();
    }) as Box<dyn FnMut()>);
    event_handler_closure.forget();
    Ok(cancel_closure)
}

mod js {
    use super::*;
    
    #[wasm_bindgen(module = "/target/js/bind.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn on_animation_frame(event_handler: &::js_sys::Function) -> Result<::js_sys::Function, JsValue>;

        #[wasm_bindgen(catch)]
        pub fn on_timeout(ms: JsValue, event_handler: &::js_sys::Function) -> Result<::js_sys::Function, JsValue>;

        #[wasm_bindgen(catch)]
        pub fn on_interval(ms: JsValue, event_handler: &::js_sys::Function) -> Result<::js_sys::Function, JsValue>;
    }
}