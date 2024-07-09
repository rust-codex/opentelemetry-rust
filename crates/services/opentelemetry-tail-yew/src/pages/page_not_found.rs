use yew::prelude::*;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    html! {
        <main style="font-family: Arial, sans-serif; text-align: center; padding: 50px;">
            <h1 style="font-size: 48px; color: #666;">{"404"}</h1>
            <p style="font-size: 18px; color: #666;">{"Oops! The page you are looking for cannot be found."}</p>
            <a href="/" style="display: inline-block; margin-top: 20px; padding: 10px 20px; font-size: 16px; color: #fff; background-color: #007BFF; text-decoration: none; border-radius: 4px;">{"Go to Homepage"}</a>
        </main>
    }
}
