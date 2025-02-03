use yew::prelude::*;

use crate::components::{Footer, NavBar};

#[function_component(NotFound)]
fn not_found() -> Html {
    html! {
        <div style="
            background-image: url('static/hardanger_stream_clip.jpg');
            background-size: cover;
            background-position: center 45%;
            height: 100vh;
            display: flex;
            justify-content: center;
            align-itmes: center;
            position: relative;
        ">  
            <div style="
                position: absolute;
                letf: 50%;
                bottom: 47.3%;
            ">
            <div class="has-text-centered">
                <h1 class="title is-1"
                    style="
                        text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.7);
                        -webkit-text-stroke: 1px rgba(0, 0, 0, 0.7);
                    "
                >{ "404 - Page Not Found" }</h1>
            </div>
        </div>
                </div>
    }
}

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <div>
            <NavBar/>
            <NotFound/>
            <Footer/>
        </div>
    }
}
