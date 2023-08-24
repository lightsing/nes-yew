use std::borrow::Cow;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProgressProps {
    pub value: u32,
    pub max: u32,
    #[prop_or_default]
    pub primary: bool,
    #[prop_or_default]
    pub success: bool,
    #[prop_or_default]
    pub warning: bool,
    #[prop_or_default]
    pub error: bool,
    #[prop_or_default]
    pub pattern: bool,
    pub class: Option<Cow<'static, str>>,
    pub style: Option<AttrValue>,
}

#[function_component(Progress)]
pub fn progress(
    ProgressProps {
        value,
        max,
        primary,
        success,
        warning,
        error,
        pattern,
        class,
        style,
    }: &ProgressProps,
) -> Html {
    html! {
        <progress
            value={AttrValue::from(format!("{value}"))}
            max={AttrValue::from(format!("{max}"))}
            class={classes!(
                class,
                "nes-progress",
                if *primary { Some("is-primary") } else { None },
                if *success { Some("is-success") } else { None },
                if *warning { Some("is-warning") } else { None },
                if *error { Some("is-error") } else { None },
                if *pattern { Some("is-pattern") } else { None },
            )}
            style={style.clone()}
        />
    }
}
