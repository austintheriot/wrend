use yew::{prelude::*, virtual_dom::AttrValue};

#[derive(PartialEq, Clone)]
pub enum ButtonVariant {
    Unpressed,
    Pressed,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        ButtonVariant::Unpressed
    }
}

impl From<ButtonVariant> for Classes {
    fn from(button_variant: ButtonVariant) -> Self {
        match button_variant {
            ButtonVariant::Unpressed => Classes::from("unpressed"),
            ButtonVariant::Pressed => Classes::from("pressed"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(AttrValue::from("button"))]
    pub button_type: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or(AttrValue::from(""))]
    pub aria_label: AttrValue,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
      <button
        class={classes!("button", props.variant.clone(), props.class.clone())}
        type={props.button_type.clone()}
        onclick={&props.onclick}
        disabled={props.disabled}
        aria-label={props.aria_label.clone()}
      >
        {for props.children.iter()}
      </button>
    }
}
