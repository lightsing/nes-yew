use std::borrow::Cow;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RadiosProps {
    pub options: Vec<RadioOption>,
    pub selected_value: AttrValue,
    pub on_value_change: Callback<AttrValue>,
    pub class: Option<Cow<'static, str>>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct RadioOption {
    pub value: AttrValue,
    pub label: AttrValue,
}

#[function_component(Radios)]
pub fn radios(
    RadiosProps {
        options,
        selected_value,
        on_value_change,
        class,
    }: &RadiosProps,
) -> Html {
    options
        .iter()
        .cloned()
        .map(|option| {
            let on_value_change = on_value_change.clone();
            let class = class.clone();
            let value = option.value.clone();
            html! {
                <label
                    class="nes-radio-group"
                    onclick={Callback::from(move |_| on_value_change.emit(value.clone()))}
                >
                    <input
                        type="radio"
                        class={classes!("nes-radio", class)}
                        value={option.value.clone()}
                        checked={option.value == *selected_value}
                        onchange={Callback::from(|_| {})}
                    />
                    <span>{ option.label.clone() }</span>
                </label>
            }
        })
        .collect()
}
