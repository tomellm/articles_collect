use std::ops::Deref;

use leptos::prelude::{Effect, Get, RwSignal, Set, Signal};
use leptos_use::use_window_size;

pub fn use_width() -> RwSignal<Width> {
    let width = RwSignal::new(Width::default());
    let window_size = use_window_size();
    Effect::new(move |_| {
        width.set(window_size.width.get().into());
    });
    width
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Width(f64);

impl From<f64> for Width {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl Deref for Width {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Width {
    fn default() -> Self {
        Self(1000.)
    }
}

pub trait TailwindScreenSizes {
    fn is_sm(&self) -> Signal<bool>;
    fn is_md(&self) -> Signal<bool>;
    fn is_lg(&self) -> Signal<bool>;
    fn is_xl(&self) -> Signal<bool>;
    fn is_2xl(&self) -> Signal<bool>;
}

impl TailwindScreenSizes for RwSignal<Width> {
    fn is_sm(&self) -> Signal<bool> {
        let width = *self;
        Signal::derive(move || *width.get() <= 640.0)
    }
    fn is_md(&self) -> Signal<bool> {
        let width = *self;
        Signal::derive(move || *width.get() <= 768.0)
    }
    fn is_lg(&self) -> Signal<bool> {
        let width = *self;
        Signal::derive(move || *width.get() <= 1024.0)
    }
    fn is_xl(&self) -> Signal<bool> {
        let width = *self;
        Signal::derive(move || *width.get() <= 1280.0)
    }
    fn is_2xl(&self) -> Signal<bool> {
        let width = *self;
        Signal::derive(move || *width.get() <= 1536.0)
    }
}
