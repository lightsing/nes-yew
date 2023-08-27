use std::borrow::Cow;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub label: Option<AttrValue>,
    pub placeholder: Option<AttrValue>,
    #[prop_or(AttrValue::from(""))]
    pub value: AttrValue,
    pub on_change: Option<Callback<HtmlInputElement>>,
    #[prop_or_default]
    pub success: bool,
    #[prop_or_default]
    pub warning: bool,
    #[prop_or_default]
    pub error: bool,
    #[prop_or_default]
    pub label_inline: bool,
    pub class: Option<Cow<'static, str>>,
    pub style: Option<AttrValue>,
}

#[function_component(TextInput)]
pub fn text_input(
    TextInputProps {
        label,
        placeholder,
        success,
        warning,
        error,
        value,
        on_change,
        label_inline,
        class,
        style,
    }: &TextInputProps,
) -> Html {
    let on_change = on_change.clone();
    html! {
        <div class={classes!("nes-field", if *label_inline { Some("is-inline") } else { None })}>
            {label.as_ref().map(|l| html!{<label for="name">{l}</label>})}
            <input
                type="text"
                value={value.clone()}
                onchange={Callback::from(move |e: Event| if let Some(on_change) = on_change.clone() { on_change.emit(e.target_dyn_into::<HtmlInputElement>().unwrap()) })}
                placeholder={placeholder.clone()}
                class={classes!(
                    class,
                    "nes-input",
                    if *success { Some("is-success") } else { None },
                    if *warning { Some("is-warning") } else { None },
                    if *error { Some("is-error") } else { None },
                )}
                style={style.clone()}
            />
        </div>
    }
}
