use yew::prelude::*;

pub struct Home {
    blurred: bool,
}

pub enum Msg {
    Clicked,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { blurred: false }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::Clicked);

        let c = if self.blurred { "blur-bg" } else { "" };
        let v = if self.blurred {
            "hide header-lnk "
        } else {
            "header-lnk"
        };
        html! {
            <header {onclick} class={c}>
                <div class={v}   >
                    <div class="title">{"Kival Mahadew"}</div>
                    <button class="header-btn">{"see.more"}</button>
                 </div>
            </header>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked => {
                self.blurred = !self.blurred;
            }
        }
        true
    }
}
