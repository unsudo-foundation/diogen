use wasm_bindgen::JsCast;

use super::*;

#[cfg(feature = "binding")]
#[cfg(feature = "win-binding")]
::modwire::expose!(
    pub win_binding
);

::modwire::expose!(
    pub cursor_client_x
    pub cursor_client_y
    pub cursor_offset_x
    pub cursor_offset_y
    pub cursor_page_x
    pub cursor_page_y
    pub cursor_screen_x
    pub cursor_screen_y
    pub device
    pub element_w
    pub element_h
    pub inner_h
    pub inner_w
    pub outer_h
    pub outer_w
);

static MOUSE_EVENTS: [&str; 6] = [
    "mousemove", 
    "mousedown", 
    "mouseup",
    "click",
    "dblclick",
    "contextmenu"
];

pub type Result<T> = ::std::result::Result<T, Error>;

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    PropertyAccessFailure(::wasm_bindgen::JsValue),
    EventListenerAttachmentFailure(::wasm_bindgen::JsValue),
    MissingWindow,
    MissingDocument,
    MissingDocumentElement,
    UnsupportedConvertionToF64
}

type DropTokenSlot = Option<::std::rc::Rc<::std::cell::RefCell<Box<dyn FnMut() + 'static>>>>;

#[repr(transparent)]
pub struct DropToken(DropTokenSlot);

impl DropToken {
    pub fn from_static_dyn_fn_mut_box(on_event: Box<dyn FnMut() + 'static>) -> Self {
        Self({
            let ret: ::std::cell::RefCell<_> = ::std::cell::RefCell::new(on_event);
            let ret: ::std::rc::Rc<_> = ::std::rc::Rc::new(ret);
            let ret: Option<_> = Some(ret);
            ret
        })
    }

    pub fn from_dyn_fn_mut_closure(on_event: ::wasm_bindgen::closure::Closure<dyn FnMut()>) -> Self {
        Self({
            let ret: ::std::cell::RefCell<_> = ::std::cell::RefCell::new(on_event);
            let ret: ::std::rc::Rc<_> = ::std::rc::Rc::new(ret);
            let ret: Box<dyn FnMut()> = Box::new({
                let ptr: ::std::rc::Rc<_> = ret.to_owned();
                move || {
                    let c: ::std::cell::RefMut<_> = ptr.borrow_mut();
                    let c: &::js_sys::Function = c.as_ref().unchecked_ref();
                    if c.call0(&::wasm_bindgen::JsValue::NULL).is_err() {
                        ::web_sys::console::log_1(&r#"
                            [ALERT] Cleanup procedure aborted. Residual closure may persist in memory. Investigate reference counts and reentrancy conditions.
                        "#.into());
                    }
                }
            });
            let ret: ::std::cell::RefCell<_> = ::std::cell::RefCell::new(ret);
            let ret: ::std::rc::Rc<_> = ::std::rc::Rc::new(ret);
            let ret: Option<_> = Some(ret);
            ret
        })
    }
}

impl ToOwned for DropToken {
    type Owned = Self;

    fn to_owned(&self) -> Self::Owned {
        if let Some(c) = &self.0 {
            let c: ::std::rc::Rc<_> = c.to_owned();
            Self(Some(c))
        } else {
            Self(None)
        }
    }
}

impl Drop for DropToken {
    fn drop(&mut self) {
        let Some(ref c) = self.0 else {
            return
        };
        let Ok(mut c) = c.try_borrow_mut() else {
            ::web_sys::console::error_1(&"[FAULT]: Cleanup lock unavailable. Disposal skipped. Possible leak. Check for reentrancy.".into());
            return
        };
        c();
    }
}

fn on<A, B>(events: Vec<&'static str>, event_handler: A) -> Result<DropToken>
where
    A: 'static,
    A: FnMut(B),
    B: 'static,
    B: ::wasm_bindgen::convert::FromWasmAbi {
    use ::wasm_bindgen::prelude::*;
    use ::std::rc;
    use ::std::cell;
    let win: ::web_sys::Window = ::web_sys::window().ok_or(Error::MissingWindow)?;
    let closure: rc::Rc<_> = {
        let ret: Box<_> = Box::new(event_handler) as Box<dyn FnMut(_)>;
        let ret: Closure<_> = Closure::wrap(ret);
        let ret: Option<_> = Some(ret);
        let ret: cell::RefCell<_> = cell::RefCell::new(ret);
        let ret: rc::Rc<_> = rc::Rc::new(ret);
        ret
    };
    {
        let events: Vec<_> = events.to_owned();
        let closure: cell::RefMut<_> = closure.borrow_mut();
        let Some(closure) = closure.as_ref() else {
            return Err(Error::EventListenerAttachmentFailure(JsValue::from("")))
        };
        let closure: &::js_sys::Function = closure.as_ref().unchecked_ref();
        for event in events {
            win.add_event_listener_with_callback(event, closure).map_err(Error::EventListenerAttachmentFailure)?;
        }
    };
    Ok(DropToken::from_static_dyn_fn_mut_box(Box::new({
        let closure: rc::Rc<_> = closure.to_owned();
        let events: Vec<_> = events.to_owned();
        move || {
            let mut closure: cell::RefMut<_> = closure.borrow_mut();
            {
                let Some(closure) = closure.as_ref() else {
                    return
                };
                let closure: &::js_sys::Function = closure.as_ref().unchecked_ref();
                for event in events.iter() {
                    win.remove_event_listener_with_callback(event, closure).expect("");
                }
            };
            *closure = None;
        }
    })))
}