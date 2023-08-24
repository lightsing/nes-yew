use std::borrow::Cow;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ListProps {
    #[prop_or_default]
    pub solid: bool,
    pub class: Option<Cow<'static, str>>,
    pub children: Children,
    pub style: Option<AttrValue>,
}

#[function_component(List)]
pub fn list(
    ListProps {
        solid,
        class,
        children,
        style,
    }: &ListProps,
) -> Html {
    html! {
        <ul
            class={classes!(
                class,
                "nes-list",
                if !*solid { Some("is-circle") } else { None },
                if *solid { Some("is-disc") } else { None },
            )}
            style={style.clone()}
        >
            {children.clone()}
        </ul>
    }
}
