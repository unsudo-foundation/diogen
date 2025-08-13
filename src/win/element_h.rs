use super::*;

pub fn use_element_h(id: &'static str) -> Signal<Option<Result<f64>>> {
    let ret: Signal<Option<Result<f64>>> = use_signal(|| None);
    let drop_token: Signal<Option<DropToken>> = use_signal(|| None);

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
            let new: f64 = element.get_bounding_client_rect().height();
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
        let mut ret: Signal<_> = ret.to_owned();
        let mut drop_token: Signal<_> = drop_token.to_owned();
        move || {
            match on_animation_frame({
                let mut update = update.to_owned();
                move || {
                    update();
                }
            }) {
                Ok(new_drop_token) => {
                    drop_token.set(None);
                    drop_token.set(Some(new_drop_token));
                },
                Err(e) => {
                    ret.set(Some(Err(Error::EventListenerAttachmentFailure(e))));
                }
            }
        }
    });

    use_drop({
        let mut drop_token: Signal<_> = drop_token.to_owned();
        move || {
            drop_token.set(None);
        }
    });

    ret
}