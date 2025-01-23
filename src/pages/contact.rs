use yew::prelude::*;

use crate::components::{Footer, NavBar};

#[function_component(ContactPage)]
pub fn contact_page() -> Html {

    html! {
        <div>
        <NavBar/>
        <div class="container" style="
            background-image: url('static/verbier_mountains.jpg');
            max-width: 100%;
            background-size: cover;
            background-position: center bottom;
            height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            position: relative;
        ">
        <div class="box" style="
            max-width: 50%;
            padding: 3rem;
        ">
            <h1 class="title-bar title is-size-1" style="
                margin-bottom: 3rem;
            ">
            {"Contact Me"}
            </h1>
            <div class="is-size-3">
                <a href="https://github.com/TorBorve" target="_blank" class="icon-link" style="margin: 0 1rem;">
                            <i class="fab fa-github fa-2x"></i>
                        </a>
                <a href="https://linkedin.com/in/torborverasmussen" target="_blank" class="icon-link" style="margin: 0 1rem;">
                    <i class="fab fa-linkedin fa-2x"></i>
                </a>
                <a href="mailto:torbras@stud.ntnu.no" class="icon-link" style="margin: 0 1rem;">
                    <i class="fas fa-envelope fa-2x"></i>
                </a>
            </div>
        </div>
        </div>
        <Footer/>
        </div>
    }
}