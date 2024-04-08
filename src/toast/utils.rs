use std::any::Any;
use time::Duration;
use uuid::Uuid;
use yew::{classes, Callback, Classes, Html, MouseEvent};

pub trait Notifiable: Any {
    fn id(&self) -> Uuid;
    fn apply_tick(&mut self, time: Duration);
    fn is_alive(&self) -> bool;
    fn is_paused(&self) -> bool;
    fn mouse_in(&mut self);
    fn mouse_out(&mut self);
}

pub trait NotifiableComponentFactory<T: Notifiable> {
    fn component(
        &self,
        toast: T,
        onclick: Callback<MouseEvent>,
        onenter: Callback<MouseEvent>,
        onleave: Callback<MouseEvent>,
    ) -> Html;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ToastType {
    #[default]
    Info,
    Warn,
    Error,
}

impl From<&str> for ToastType {
    fn from(data: &str) -> Self {
        match data {
            "info" => Self::Info,
            "warn" => Self::Warn,
            "error" => Self::Error,
            _ => Self::Info,
        }
    }
}

impl From<&ToastType> for Classes {
    fn from(toast_type: &ToastType) -> Self {
        match toast_type {
            ToastType::Info => classes!("info"),
            ToastType::Warn => classes!("warn"),
            ToastType::Error => classes!("error"),
        }
    }
}

