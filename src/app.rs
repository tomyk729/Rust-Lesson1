
use yew::prelude::*;
use crate::components::profile_card::ProfileCard;

#[function_component(App)]
pub fn app() -> Html {

    html! {
      <>
      <div class="grid grid-cols-3 gap-4">
        <ProfileCard title={"Alexa"} image_src={"images/alexa.png"}/>
        <ProfileCard title={"Siri"} image_src={"images/siri.png"}/>
        <ProfileCard title={"Cortana"} image_src={"images/cortana.png"}/>
        </div>

      </>
      }
}
