use super::*;

pub fn use_inner_h() -> Signal<Option<Result<f64>>> {
    let ret: Signal<Option<Result<f64>>> = use_signal(|| None);
    let drop_token: Signal<Option<DropToken>> = use_signal(|| None);

    let update = {
        let mut ret: Signal<_> = ret.to_owned();
        move || {
            let Some(win) = ::web_sys::window() else {
                ret.set(Some(Err(Error::MissingWindow)));
                return
            };
            match win.inner_height() {
                Ok(new_h) => {
                    let Some(new_h) = new_h.as_f64() else {
                        ret.set(Some(Err(Error::UnsupportedConvertionToF64)));
                        return
                    };
                    ret.set(Some(Ok(new_h)));
                },
                Err(e) => ret.set(Some(Err(Error::PropertyAccessFailure(e))))
            };
        }
    };

    use_effect({
        let mut update = update.to_owned();
        move || {
            update();
        }
    });

    use_effect({
        let mut update = update.to_owned();
        let mut ret: Signal<_> = ret.to_owned();
        let mut drop_token: Signal<_> = drop_token.to_owned();
        move || {
            match on(vec!["resize"], move |_: ::web_sys::Event| {
                update();
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