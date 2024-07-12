use yew::prelude::*;
use yew_agent::reactor::use_reactor_subscription;

use crate::websocket::WebsocketReactor;

#[function_component(Logs)]
pub fn logs() -> Html {
    let reactor = use_reactor_subscription::<WebsocketReactor>();

    html! {
        <main class="app-body">
            <nav class="app-body-navigation">
                <div class="sidebar">
                    <div class="menu-item title">
                        <span class="menu-text">{"Status"}</span>
                    </div>
                    <div class="submenu">
                        <div class="menu-item info">
                            <input type="checkbox" id="info" />
                            <label for="info">
                                <span class="status-icon"></span>
                                <span class="menu-text">{"Info"}</span>
                            </label>
                        </div>
                        <div class="menu-item warn">
                            <input type="checkbox" id="warn" />
                            <label for="warn">
                                <span class="status-icon"></span>
                                <span class="menu-text">{"Warn"}</span>
                            </label>
                        </div>
                        <div class="menu-item error">
                            <input type="checkbox" id="error" />
                            <label for="error">
                                <span class="status-icon"></span>
                                <span class="menu-text">{"Error"}</span>
                            </label>
                        </div>
                    </div>
                </div>
            </nav>
            <section class="app-body-main-content">
                <div class="logs-table-container">
                    <table class="logs-table">
                        <thead>
                            <tr>
                                <th>{"Timestamp"}</th>
                                <th>{"Level"}</th>
                                <th>{"Message"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {
                                reactor.iter().last().cloned().map(|value|{
                                    let v = (*value).clone();

                                    v.into_iter().map(|v| {
                                        html!{
                                            <tr class="info">
                                                <td>{"2024-07-06 12:00:00"}</td>
                                                <td>{"INFO"}</td>
                                                <td>{v}</td>
                                            </tr>
                                        }
                                    }).collect::<Html>()
                                })
                            }
                            <tr class="info">
                                <td>{"2024-07-06 12:00:00"}</td>
                                <td>{"INFO"}</td>
                                <td>{"Application started successfully."}</td>
                            </tr>
                            <tr class="warn">
                                <td>{"2024-07-06 12:05:00"}</td>
                                <td>{"WARN"}</td>
                                <td>{"Deprecated API call detected."}</td>
                            </tr>
                            <tr class="error">
                                <td>{"2024-07-06 12:10:00"}</td>
                                <td>{"ERROR"}</td>
                                <td>{"Failed to connect to the database."}</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </section>
            <aside class="app-body-sidebar"></aside>
        </main>
    }
}
