use crate::routes::switch;
use crate::routes::Route;
use yew::{function_component, html};
use yew_router::BrowserRouter;
use yew_router::Switch;

mod pages;
mod routes;

#[function_component(Base)]
pub fn base() -> html::Html {
    html!(
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
    )
}

fn main() {
    yew::start_app::<Base>();
}
