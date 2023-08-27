use std::borrow::Cow;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    pub children: Children,
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub dark: bool,
    #[prop_or_default]
    pub rounded: bool,
    #[prop_or_default]
    pub centered: bool,
    pub class: Option<Cow<'static, str>>,
    pub style: Option<AttrValue>,
}

#[function_component(Container)]
pub fn container(
    ContainerProps {
        children,
        class,
        dark,
        rounded,
        centered,
        title,
        style,
    }: &ContainerProps,
) -> Html {
    let class = classes!(
        class,
        "nes-container",
        title.as_ref().map(|_| "with-title"),
        if *dark { Some("is-dark") } else { None },
        if *rounded { Some("is-rounded") } else { None },
        if *centered { Some("is-centered") } else { None },
    );

    html! {
        <section class={class} style={style.clone()}>
            {title.as_ref().map(|t| html!{<h2 class="title">{t}</h2>})}
            {children.clone()}
        </section>
    }
}
