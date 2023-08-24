use std::borrow::Cow;
use strum::{EnumIter, IntoEnumIterator};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ControllerIconProps {
    pub controller: ControllerKind,
    pub class: Option<Cow<'static, str>>,
    pub style: Option<AttrValue>,
}

#[derive(Copy, Clone, PartialEq, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum ControllerKind {
    Snes,
    SnesJp,
    Nes,
    NesJp,
}

impl ControllerKind {
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as IntoEnumIterator>::iter()
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ControllerKind::Snes => "snes-logo",
            ControllerKind::SnesJp => "snes-jp-logo",
            ControllerKind::Nes => "nes-logo",
            ControllerKind::NesJp => "nes-jp-logo",
        }
    }
}

#[function_component(ControllerIcon)]
pub fn controller_icon(
    ControllerIconProps {
        controller,
        class,
        style,
    }: &ControllerIconProps,
) -> Html {
    html! {
        <i
            class={classes!(class, controller.to_str())}
            style={style.clone()}
        />
    }
}
