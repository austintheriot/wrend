use shared::Route;
use yew::{prelude::*, virtual_dom::AttrValue};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LinkCardProps {
    pub route: Route,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or(AttrValue::from(""))]
    pub title: AttrValue,
    #[prop_or(AttrValue::from(""))]
    pub img_src: AttrValue,
    #[prop_or(AttrValue::from(""))]
    pub vid_src: AttrValue,
    #[prop_or(AttrValue::from(""))]
    pub img_alt: AttrValue,
}

#[function_component(LinkCard)]
pub fn link_card(props: &LinkCardProps) -> Html {
    html! {
        <Link<Route> to={props.route} classes={classes!("link-card", props.classes.clone())}>
            <p class="title">{props.title.clone()}</p>
            {if !props.img_src.is_empty() {
                html!{
                    <img src={props.img_src.clone()} alt={props.img_alt.clone()} />
                }
            } else {
                html!{}
            }}
            {if !props.vid_src.is_empty() {
                html!{
                    <video src={props.vid_src.clone()} autoplay={true} controls={false} />
                }
            } else {
                html!{}
            }}
        </Link<Route>>
    }
}
