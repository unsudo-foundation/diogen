use ::dioxus::prelude::*;

pub mod layout;
pub mod win;

#[cfg(feature = "binding")]
pub mod binding;

::modwire::expose!(
    pub url
);