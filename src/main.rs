use routes::vesting::VestingComponent;
use crate::routes::on_demand::OnDemandComponent;
use yew::prelude::*;
use yew_router::prelude::*;

mod routes;
mod services;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/vesting")]
    Vesting,
    #[at("/ondemand")]
    OnDemand,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

mod pages {
    use super::*;

    #[function_component(Home)]
    pub fn home() -> Html {
        html! {
            <div>  
                <h1>{"Polkadot Utilities by W3F TechEd team"}</h1>
                <HashRouter>
                    <Link<Route> to={Route::Vesting}>
                        <button>{"Vesting"}</button>
                    </Link<Route>>
                </HashRouter>
                <HashRouter>
                    <Link<Route> to={Route::OnDemand}>
                        <button>{"On-demand Parachain"}</button>
                    </Link<Route>>
            </HashRouter>
            </div> 
        }
    }

    #[function_component(Vesting)]
    pub fn vesting() -> Html {
         html! { <VestingComponent/> }
    }

    #[function_component(OnDemand)]
    pub fn vesting() -> Html {
         html! { <OnDemandComponent/> }
    }

    #[function_component(NotFound)]
    pub fn not_found() -> Html {
        html! {
            <div class="container">
                <h1 class="title">{ "404" }</h1>
                <p>{ "This is the 404 page." }</p>
            </div>
        }
    }
}


fn main() {
    yew::Renderer::<VestingApp>::new().render();
}

#[function_component(VestingApp)]
pub fn vesting_app() -> Html {
    html! {
        <HashRouter>
            <div class="navbar">
                <Link<Route> classes={classes!("button")} to={Route::Home}>
                    { "Home" }
                </Link<Route>>
                { " | " }
                <Link<Route> classes={classes!("button")} to={Route::Vesting}>
                    { "Vesting" }
                </Link<Route>>
                { " | " }
                <Link<Route> classes={classes!("button")} to={Route::OnDemand}>
                    { "On-demand" }
                </Link<Route>>
            </div>

            <main>
                <Switch<Route> render={switch} />
            </main>
            <footer class="footer">
            </footer>
        </HashRouter>
    }
}

fn switch(routes: Route) -> Html {

    match routes {
        Route::Home => html! { <pages::Home /> },
        Route::Vesting => html! { <pages::Vesting /> },
        Route::OnDemand => html! { <pages::OnDemand /> },
        Route::NotFound => html! { <pages::NotFound /> },
    }
}