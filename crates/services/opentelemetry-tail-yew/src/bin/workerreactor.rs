use opentelemetry_tail_yew::websocket::WebsocketReactor;
use yew_agent::Registrable;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    WebsocketReactor::registrar().register();
}
