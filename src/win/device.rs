use super::*;

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum Device {
    Laptop4K,
    LaptopL,
    LaptopS,
    Tablet,
    MobileL,
    MobileM,
    MobileS
}

#[inline]
pub fn use_device() -> Signal<Option<Result<Device>>> {
    let inner_w: Signal<_> = use_inner_w();
    let mut ret: Signal<Option<Result<Device>>> = use_signal(|| None);

    if let Some(w) = inner_w() {
        match w {
            Ok(w) => match w {
                w if w >= 2560.0 => ret.set(Some(Ok(Device::Laptop4K))),
                w if w >= 1440.0 => ret.set(Some(Ok(Device::LaptopL))),
                w if w >= 1024.0 => ret.set(Some(Ok(Device::LaptopS))),
                w if w >= 768.0 => ret.set(Some(Ok(Device::Tablet))),
                w if w >= 425.0 => ret.set(Some(Ok(Device::MobileL))),
                w if w >= 375.0 => ret.set(Some(Ok(Device::MobileM))),
                _ => ret.set(Some(Ok(Device::MobileS)))
            },
            Err(e) => ret.set(Some(Err(e)))
        };
    } else {
        ret.set(None);
    }

    ret
}