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

    fn create(_ctx: &Context<Self>) -> Self {
        Self { blurred: false }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::Clicked);
        let onclick2 = ctx.link().callback(|_| Msg::Clicked);

        let c = if self.blurred { "bg blur-bg" } else { "bg" };
        let v = if self.blurred {
            "header-lnk hide"
        } else {
            "header-lnk"
        };

        let d = if self.blurred { "menu" } else { "hide menu" };
        html! {
        <header >
            <div class={c} onclick={onclick2}></div>
            <div class={d}>

                    <a href="https://github.com/KivalM"><i class="fa-brands fa-github"></i>{"KivalM"}</a>
                    <a href="https://linkedin.com/in/kivalm"><i class="fa-brands fa-linkedin"></i>{"Kival Mahadew"}</a>
                    <a href="https://www.instagram.com/uwukival/"><i class="fa-brands fa-instagram"></i>{"uwukival"}</a>

            </div>
            <div class={v} {onclick}>
                <div class="title">{"Kival Mahadew"}</div>
                <button class="header-btn">{"see.more"}</button>
            </div>
        </header>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked => {
                self.blurred = !self.blurred;
            }
        }
        true
    }
}
