mod content;
mod header;

use content::AppContent;
use header::AppHeader;
use log::info;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class="app">
            <AppHeader />
            <AppContent />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
