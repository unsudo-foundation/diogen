use ::dioxus::prelude::*;

#[cfg(feature = "binding")]
pub mod binding;

pub mod win;

::modwire::expose!(
    pub url
);