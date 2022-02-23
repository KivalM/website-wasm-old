use yew::prelude::*;

pub struct Home;

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <header>
                <a href="#" class="header-lnk">
                <div class="title">{"Kival Mahadew"}</div>
                <button class="header-btn">{"see.more"}</button>
                </a>
            </header>

        }
    }
}
