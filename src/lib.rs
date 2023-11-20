/// Application widget.
pub mod app;

/// Application views.
pub mod views;

/// Localization manager.
pub mod localization;

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
i18n!();