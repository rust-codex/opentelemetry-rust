mod pages;

use pages::logs::Logs;
use pages::metrics::Metrics;
use pages::page_not_found::PageNotFound;
use pages::traces::Traces;
use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    // #[at("/logs")]
    // Logs,
    #[at("/")]
    Logs,
    #[at("/traces")]
    Traces,
    #[at("/metrics")]
    Metrics,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Logs => {
            html! { <Logs /> }
        }
        Route::Metrics => {
            html! { <Metrics /> }
        }
        Route::Traces => {
            html! { <Traces /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <div class="app">
                    <header class="header">
                        <div class="header-content responsive-wrapper">
                            <div class="header-logo">
                                <img src="data:image/svg+xml;utf8,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20width='200'%20height='200'%20viewBox='0%200%20200%20200'%3E%3Ccircle%20cx='100'%20cy='100'%20r='90'%20stroke='%234fc3f7'%20stroke-width='10'%20fill='none'/%3E%3Ccircle%20cx='100'%20cy='100'%20r='60'%20stroke='%23ffca28'%20stroke-width='10'%20fill='none'/%3E%3Ccircle%20cx='100'%20cy='100'%20r='10'%20fill='%23f44336'/%3E%3Cpath%20d='M100,10%20L120,40'%20stroke='%234fc3f7'%20stroke-width='5'/%3E%3Cpath%20d='M100,10%20L80,40'%20stroke='%234fc3f7'%20stroke-width='5'/%3E%3Cpath%20d='M10,100%20L40,120'%20stroke='%23ffca28'%20stroke-width='5'/%3E%3Cpath%20d='M10,100%20L40,80'%20stroke='%23ffca28'%20stroke-width='5'/%3E%3Cpath%20d='M100,190%20L120,160'%20stroke='%234fc3f7'%20stroke-width='5'/%3E%3Cpath%20d='M100,190%20L80,160'%20stroke='%234fc3f7'%20stroke-width='5'/%3E%3Cpath%20d='M190,100%20L160,120'%20stroke='%23ffca28'%20stroke-width='5'/%3E%3Cpath%20d='M190,100%20L160,80'%20stroke='%23ffca28'%20stroke-width='5'/%3E%3C/svg%3E" />
                            </div>
                            <div class="header-navigation">
                                { self.view_nav() }
                            </div>
                        </div>
                    </header>
                    <Switch<Route> render={switch} />
                </div>
            </BrowserRouter>
        }
    }
}

impl App {
    fn view_nav(&self) -> Html {
        html! {
            <nav class="header-navigation-links">
                <Link<Route> to={Route::Logs}>
                    {"Logs"}
                </Link<Route>>
                <Link<Route> to={Route::Traces}>
                    {"Traces"}
                </Link<Route>>
                <Link<Route> to={Route::Metrics}>
                    {"Metrics"}
                </Link<Route>>
            </nav>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
