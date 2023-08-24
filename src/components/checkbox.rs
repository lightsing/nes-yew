use std::borrow::Cow;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CheckboxProps {
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub on_select: Callback<()>,
    pub class: Option<Cow<'static, str>>,
}

#[function_component(Checkbox)]
pub fn checkbox(
    CheckboxProps {
        label,
        on_select,
        checked,
        class,
    }: &CheckboxProps,
) -> Html {
    let on_select = on_select.clone();
    html! {
        <div>
            <label class="nes-checkbox-parent">
                <input
                    type="checkbox"
                    class={classes!(class, "nes-checkbox")}
                    checked={*checked}
                    onchange={Callback::from(move |_| on_select.emit(()))}
                />
                {label.as_ref().map(|l| html!{<span>{l}</span>})}
            </label>
        </div>
    }
}
