use yew::{prelude::*, props};

#[derive(Properties, PartialEq)]
pub struct Parameters{
    pub title: String,
    pub image_src: String
}

#[function_component(ProfileCard)]
pub fn profile_card(props: &Parameters) -> Html {


    html!{
        <>
        <div class="grid grid-rows-2">

            <div class="rounded-lg border border-sky-500 ">
                <h3>{props.title.clone()}</h3>
                <img src={props.image_src.clone()} class="scale-75"  alt="picture" />
            </div>

        </div>
        </>
    }
}