use std::borrow::Cow;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub children: Children,
    #[prop_or_default]
    pub primary: bool,
    #[prop_or_default]
    pub success: bool,
    #[prop_or_default]
    pub warning: bool,
    #[prop_or_default]
    pub error: bool,
    #[prop_or_default]
    pub disabled: bool,
    pub class: Option<Cow<'static, str>>,
    pub style: Option<AttrValue>,
}
#[function_component(Button)]
pub fn button(
    ButtonProps {
        children,
        class,
        primary,
        success,
        warning,
        error,
        disabled,
        style,
    }: &ButtonProps,
) -> Html {
    let class = classes!(
        class,
        "nes-btn",
        if *primary { Some("is-primary") } else { None },
        if *success { Some("is-success") } else { None },
        if *warning { Some("is-warning") } else { None },
        if *error { Some("is-error") } else { None },
        if *disabled { Some("is-disabled") } else { None },
    );

    html! {
        <button class={class} disabled={*disabled} style={style.clone()}>
            {children.clone()}
        </button>
    }
}
