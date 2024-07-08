use yew::prelude::*;

#[function_component]
pub fn AppHeader() -> Html {
    html! {
      <header class="header">
          <div class="header-content responsive-wrapper">
              <div class="header-logo">
                  <img src="data:image/svg+xml;utf8,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20width='200'%20height='200'%20viewBox='0%200%20200%20200'%3E%3Ccircle%20cx='100'%20cy='100'%20r='90'%20stroke='%234fc3f7'%20stroke-width='10'%20fill='none'/%3E%3Ccircle%20cx='100'%20cy='100'%20r='60'%20stroke='%23ffca28'%20stroke-width='10'%20fill='none'/%3E%3Ccircle%20cx='100'%20cy='100'%20r='10'%20fill='%23f44336'/%3E%3Cpath%20d='M100,10%20L120,40'%20stroke='%234fc3f7'%20stroke-width='5'/%3E%3Cpath%20d='M100,10%20L80,40'%20stroke='%234fc3f7'%20stroke-width='5'/%3E%3Cpath%20d='M10,100%20L40,120'%20stroke='%23ffca28'%20stroke-width='5'/%3E%3Cpath%20d='M10,100%20L40,80'%20stroke='%23ffca28'%20stroke-width='5'/%3E%3Cpath%20d='M100,190%20L120,160'%20stroke='%234fc3f7'%20stroke-width='5'/%3E%3Cpath%20d='M100,190%20L80,160'%20stroke='%234fc3f7'%20stroke-width='5'/%3E%3Cpath%20d='M190,100%20L160,120'%20stroke='%23ffca28'%20stroke-width='5'/%3E%3Cpath%20d='M190,100%20L160,80'%20stroke='%23ffca28'%20stroke-width='5'/%3E%3C/svg%3E" />
              </div>
              <div class="header-navigation">
                  <nav class="header-navigation-links">
                      <a href="#"> {"Logs"} </a>
                      <a href="#"> {"Traces"} </a>
                      <a href="#"> {"Metrics"} </a>
                  </nav>
              </div>
          </div>
      </header>
    }
}
