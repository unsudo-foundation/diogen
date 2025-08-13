use ::dioxus::prelude::*;

pub mod layout;
pub mod win;

#[cfg(feature = "binding")]
pub mod binding;

::modwire::expose!(
    pub url
);


// TODO Add to notes.
// Severity & Status Prefix
// [INFO]      Normal telemetry, expected events, or minor notes.
// [NOTE]      Doesn't require action but is worth highlighting.
// [WARN]      Something unexpected happened, but the system is still nominal (within safe limits).
// [CAUTION]   Approaching unsafe limits, operator attention required.
// [ALERT]     Problem detected that could escalate if not addressed.
// [ABORT]     Stop current operation, execute safety sequence.
// [FAIL]      Beyond nominal operation, fallback procedures required.
// [NOGO]      Not cleared to proceed.
// [NOMINAL]   Everything is performing within expected parameters.