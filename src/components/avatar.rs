use std::borrow::Cow;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AvatarProps {
    #[prop_or_default]
    pub small: bool,
    #[prop_or_default]
    pub medium: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub rounded: bool,
    pub src: AttrValue,
    pub class: Option<Cow<'static, str>>,
}

#[function_component(Avatar)]
pub fn avatar(
    AvatarProps {
        src,
        small,
        medium,
        large,
        rounded,
        class,
    }: &AvatarProps,
) -> Html {
    html! {
        <img
            src={src.clone()}
            class={classes!(
                class,
                "nes-avatar",
                if *small { Some("is-small") } else { None },
                if *medium { Some("is-medium") } else { None },
                if *large { Some("is-large") } else { None },
                if *rounded { Some("is-rounded") } else { None },
            )}
        />
    }
}
