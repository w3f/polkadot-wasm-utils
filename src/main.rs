use routes::vesting::VestingComponent;
use yew::prelude::*;
use yew_router::prelude::*;

mod routes;
mod services;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/vesting")]
    Vesting,
    #[not_found]
    #[at("/")]
    Home,
}

fn main() {
    yew::Renderer::<VestingApp>::new().render();
}

struct VestingApp;

impl Component for VestingApp {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        VestingApp
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <HashRouter>
                <Switch<Route> render={switch} />
            </HashRouter>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <VestingComponent/> },
        Route::Vesting => {
            html! {
            <div>
                <h1>{"Polkadot Utilities by W3F TechEd team"}</h1>
                <a href="/vesting"> <button>{"Unlock vested balance"}</button></a>
            </div> }
        }
    }
}
