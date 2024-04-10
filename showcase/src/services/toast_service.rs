use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;

#[derive(PartialEq)]
pub struct ToastMessage {
    pub message: String,
    pub toast_type: String,
    pub class: String,
}

#[derive(Default, PartialEq)]
pub struct ToastService {
    pub toasts: Vec<ToastMessage>,
}

#[derive(PartialEq, Clone)]
pub struct ToastServiceContext(pub Rc<RefCell<ToastService>>);

#[hook]
pub fn use_toast() -> Rc<RefCell<ToastService>> {
    let context = use_context::<ToastServiceContext>().expect("no ctx found");
    context.0.clone()
}