use ::dioxus::prelude::*;

#[cfg(feature = "bind")]
pub mod bind;

pub mod win;

::modwire::expose!(
    pub url
);