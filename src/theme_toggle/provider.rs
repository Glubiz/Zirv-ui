use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum Theme {
    Light,
    Dark,
}

struct ThemeContext {
    theme: Theme,
    toggle_theme: Callback<()>,
}

impl ThemeContext {
    fn new(theme: Theme, toggle_theme: Callback<()>) -> Self {
        Self { theme, toggle_theme }
    }
}

// Context provider component
#[derive(Properties, PartialEq)]
pub struct ThemeProviderProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct ThemeProvider {
    theme: Theme,
    link: ComponentLink<Self>,
}

enum Msg {
    ToggleTheme,
}

impl Component for ThemeProvider {
    type Message = Msg;
    type Properties = ThemeProviderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            theme: Theme::Dark,
            link: _ctx.link().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleTheme => {
                self.theme = match self.theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let theme_context = ThemeContext::new(self.theme.clone(), self.link.callback(|_| Msg::ToggleTheme));
        html! {
            <ContextProvider<ThemeContext> context={theme_context}>
                { for ctx.props().children.iter() }
            </ContextProvider<ThemeContext>>
        }
    }
}