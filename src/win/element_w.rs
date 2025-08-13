use super::*;

pub fn use_element_w(id: &'static str) -> Signal<Option<Result<f64>>> {
    use ::wasm_bindgen::prelude::*;
    let ret: Signal<Option<Result<f64>>> = use_signal(|| None);
    let cancel: Signal<Option<Closure<dyn FnMut()>>> = use_signal(|| None);

    let update = {
        let mut ret: Signal<_> = ret.to_owned();
        move || {
            let Some(win) = ::web_sys::window() else {
                ret.set(Some(Err(Error::MissingWindow)));
                return
            };
            let Some(doc) = win.document() else {
                ret.set(Some(Err(Error::MissingDocument)));
                return
            };
            let Some(element) = doc.get_element_by_id(id) else {
                ret.set(None);
                return
            };
            let new: f64 = element.get_bounding_client_rect().width();
            ret.set(Some(Ok(new)));
        }
    };

    use_effect({
        let mut update = update.to_owned();
        move || {
            update();
        }
    });

    use_effect({
        let update = update.to_owned();
        let mut cancel: Signal<_> = cancel.to_owned();
        let mut ret: Signal<_> = ret.to_owned();
        move || {
            match on_animation_frame({
                let mut update = update.to_owned();
                move || {
                    update();
                }
            }) {
                Ok(new_cancel) => {
                    let cancel =
                    if let Some(old_cancel) = cancel() {
                        old_cancel();
                    }
                    cancel.set(None);
                    cancel.set(Some(new_cancel));
                },
                Err(e) => {
                    ret.set(Some(Err(Error::EventListenerAttachmentFailure(e))));
                }
            }
        }
    });

    use_drop({
        let mut drop_token: Signal<_> = cancel.to_owned();
        move || {
            drop_token.set(None);
        }
    });

    ret
}