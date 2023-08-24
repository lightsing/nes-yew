use std::borrow::Cow;
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub icon: IconKind,
    #[prop_or_default]
    pub small: bool,
    #[prop_or_default]
    pub medium: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub empty: bool,
    #[prop_or_default]
    pub transparent: bool,
    #[prop_or_default]
    pub half: bool,
    pub class: Option<Cow<'static, str>>,
}

#[derive(Copy, Clone, Eq, PartialEq, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum IconKind {
    Heart,
    Star,
    Like,
    Twitter,
    Facebook,
    Github,
    Youtube,
    Google,
    Medium,
    Twitch,
    Reddit,
    Whatsapp,
    Gmail,
    Linkedin,
    Close,
    Trophy,
}

impl IconKind {
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as IntoEnumIterator>::iter()
    }
}

#[function_component(Icon)]
pub fn icon(
    IconProps {
        icon,
        small,
        medium,
        large,
        empty,
        transparent,
        half,
        class,
    }: &IconProps,
) -> Html {
    let icon: &'static str = icon.into();
    html! {
        <i
            class={classes!(
                class,
                "nes-icon",
                icon,
                if *small { Some("is-small") } else { None },
                if *medium { Some("is-medium") } else { None },
                if *large { Some("is-large") } else { None },
                if *empty { Some("is-transparent") } else { None },
                if *transparent { Some("is-empty") } else { None },
                if *half { Some("is-half") } else { None },
            )}
        />
    }
}
