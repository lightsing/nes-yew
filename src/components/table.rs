use std::borrow::Cow;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TableProps {
    #[prop_or_default]
    pub bordered: bool,
    #[prop_or_default]
    pub centered: bool,
    #[prop_or_default]
    pub dark: bool,
    pub class: Option<Cow<'static, str>>,
    pub children: Children,
    pub style: Option<AttrValue>,
}

#[function_component(Table)]
pub fn table(
    TableProps {
        bordered,
        centered,
        dark,
        children,
        class,
        style,
    }: &TableProps,
) -> Html {
    html! {
        <table
            class={classes!(
                class,
                "nes-table",
                if *bordered { Some("is-bordered") } else { None },
                if *centered { Some("is-centered") } else { None },
                if *dark { Some("is-dark") } else { None },
            )}
            style={style.clone()}
        >
            {children.clone()}
        </table>
    }
}
