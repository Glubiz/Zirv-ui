use yew::{html, Callback, Html, MouseEvent};

use super::utils::NotifiableComponentFactory;
use super::component::ToastComponent;
use super::Toast;

#[derive(Clone, PartialEq, Default)]
pub struct ToastFactory;

impl NotifiableComponentFactory<Toast> for ToastFactory {
    fn component(
        &self,
        toast: Toast,
        onclick: Callback<MouseEvent>,
        onenter: Callback<MouseEvent>,
        onleave: Callback<MouseEvent>,
    ) -> Html {
        html! {
            <ToastComponent {toast} {onclick} {onenter} {onleave} />
        }
    }
}
