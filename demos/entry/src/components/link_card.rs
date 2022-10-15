use log::error;
use shared::Route;
use web_sys::HtmlVideoElement;
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
    #[prop_or(AttrValue::from("lazy"))]
    pub img_loading: AttrValue,
    #[prop_or(AttrValue::from(""))]
    pub vid_src: AttrValue,
    #[prop_or(AttrValue::from(""))]
    pub img_alt: AttrValue,
}

#[function_component(LinkCard)]
pub fn link_card(props: &LinkCardProps) -> Html {
    let video_ref = use_node_ref();
    let handle_mouse_enter = {
        let video_ref = video_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let video: Option<HtmlVideoElement> = video_ref.cast();
            if let Some(video) = video {
                if let Err(err) = video.play() {
                    error!("Error trying to play video element: {:?}", err);
                }
            }
        })
    };

    let handle_mouse_leave = {
        let video_ref = video_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let video: Option<HtmlVideoElement> = video_ref.cast();
            if let Some(video) = video {
                if let Err(err) = video.pause() {
                    error!("Error trying to pause video element: {:?}", err);
                }
            }
        })
    };

    html! {
        <div
            class={classes!("link-card", props.classes.clone())}
            onmouseenter={handle_mouse_enter}
            onmouseleave={handle_mouse_leave}
        >
            <Link<Route> to={props.route}>
                <p class="title">{props.title.clone()}</p>
                {if !props.img_src.is_empty() {
                    html!{
                        <img
                            src={props.img_src.clone()}
                            alt={props.img_alt.clone()}
                            loading={props.img_loading.clone()}
                        />
                    }
                } else {
                    html!{}
                }}
                {if !props.vid_src.is_empty() {
                    html!{
                        <video
                            src={props.vid_src.clone()}
                            controls={false}
                            ref={video_ref}
                            muted={true}
                            loop={true}
                            autoplay={true}
                            playsinline={true}
                        />
                    }
                } else {
                    html!{}
                }}
            </Link<Route>>
        </div>
    }
}
