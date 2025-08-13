use super::*;

pub fn use_cursor_screen_x() -> Signal<Option<Result<f64>>> {
    let ret: Signal<Option<Result<f64>>> = use_signal(|| None);
    let drop_token: Signal<Option<DropToken>> = use_signal(|| None);

    use_effect({
        let mut ret: Signal<_> = ret.to_owned();
        let mut drop_token: Signal<_> = drop_token.to_owned();
        move || {
            match on(MOUSE_EVENTS.into(), move |mouse_event: ::web_sys::MouseEvent| {
                let new: f64 = mouse_event.screen_x() as f64;
                ret.set(Some(Ok(new)));
            }) {
                Ok(new_drop_token) => {
                    drop_token.set(None);
                    drop_token.set(Some(new_drop_token));
                },
                Err(e) => {
                    ret.set(Some(Err(e)));
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