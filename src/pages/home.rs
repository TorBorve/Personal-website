use yew::prelude::*;

use crate::components::{NavBar, Footer};

#[function_component(AboutMe)]
fn about_me() -> Html {

    html!{
        <section class="section">
            <div class="columns">
                <div class="column">
                    <div style="width: 70%; margin: 0 auto;">
                        <h1 class="title-bar title is-size-1">
                            {"About Me"}
                        </h1>
                        <h2 class="subtitle is-size-3" style="margin-top: 2rem;">
                            {"Hi, I'm Tor Børve Rasmussen, and I am currently studying 
                            "}<strong>{"Control Systems and Robotics"}</strong>{" at NTNU. 
                            I am in the final year of my master's program and working on my thesis about "}
                            <strong>{"motion control"}</strong>{"."}
                        </h2>
                        <h2 class="subtitle is-size-3">
                            {"Beyond my academic pursuits, I am passionate about skiing and mountain hiking, 
                            enjoying the nature and the outdoors."}
                        </h2>
                    </div>
                </div>
                <div class="column">
                    <figure class="image">
                        <img src="static/auc_passed_technical.jpg"/>
                    </figure>
                </div>
            </div>
        </section>
    }
}

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <section class="section">
        <NavBar/>
        <div class="container" style="
            background-image: url('static/tor_verbier.jpg');
            background-position: 50% 80%;
            height: 100vh;
            max-width: 100%;
            background-size: cover;
            // overflow: hidden;
            ">
            <div style="
                position: absolute;
                top: 30%;
                left: 50%;
                max-width: 30%;
                text-align: left;
                wrap-word: break-word;
            ">
                <h1 class="title is-size-1">{"Control Systems and Robotics Engineer based in Norway."} </h1>
                // <!-- More about me link -->
                <p class="subtitle is-size-1" href="#about">
                    {"More about me"}
                </p>
            </div>
        </div>
        <div id="about">
            <AboutMe/>
        </div>
        <Footer/>
        </section>
    }
}