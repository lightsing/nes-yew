use std::borrow::Cow;
use strum::{EnumIter, IntoEnumIterator};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SpriteProps {
    pub sprite: SpriteKind,
    pub class: Option<Cow<'static, str>>,
    pub style: Option<AttrValue>,
}

#[derive(Copy, Clone, PartialEq, EnumIter)]
pub enum SpriteKind {
    Octocat,
    Mario,
    Ash,
    Pokeball,
    Bulbasaur,
    Charmander,
    Squirtle,
    Smartphone,
    Phone,
    Kirby,
    Bcrikko,
}

impl SpriteKind {
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as IntoEnumIterator>::iter()
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            SpriteKind::Octocat => "nes-octocat",
            SpriteKind::Mario => "nes-mario",
            SpriteKind::Ash => "nes-ash",
            SpriteKind::Pokeball => "nes-pokeball",
            SpriteKind::Bulbasaur => "nes-bulbasaur",
            SpriteKind::Charmander => "nes-charmander",
            SpriteKind::Squirtle => "nes-squirtle",
            SpriteKind::Smartphone => "nes-smartphone",
            SpriteKind::Phone => "nes-phone",
            SpriteKind::Kirby => "nes-kirby",
            SpriteKind::Bcrikko => "nes-bcrikko",
        }
    }
}

#[function_component(Sprite)]
pub fn sprite(
    SpriteProps {
        sprite,
        class,
        style,
    }: &SpriteProps,
) -> Html {
    html! {
        <i
            class={classes!(class, sprite.to_str())}
            style={style.clone()}
        />
    }
}
