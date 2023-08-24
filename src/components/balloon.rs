use std::borrow::Cow;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BalloonProps {
    #[prop_or_default]
    pub from_left: bool,
    #[prop_or_default]
    pub from_right: bool,
    pub class: Option<Cow<'static, str>>,
    pub style: Option<AttrValue>,
    pub children: Children,
}

#[function_component(Balloon)]
pub fn balloon(
    BalloonProps {
        from_left,
        from_right,
        class,
        children,
        style,
    }: &BalloonProps,
) -> Html {
    html! {
        <div
            class={classes!(
                class,
                "nes-balloon",
                if *from_left { Some("from-left") } else { None },
                if *from_right { Some("from-right") } else { None },
            )}
            style={style.clone()}
        >
            {children.clone()}
        </div>
    }
}
