use yew::prelude::*;

use crate::components::{NavBar, Footer};

#[function_component(ProjectsIntro)]
fn projects_intro() -> Html {
    
    html!{
        <section class="section">
            <div class="columns is-vcentered">
                <div class="column" style="width: 30%">
                // <h1 class="title">{"Help"}</h1>
                    <figure class="imgage">
                        <img src="static/tor_multimeter.jpg" style="border-radius: 10px;"/>
                    </figure>
                </div>
                <div class="column">
                    <div style="width: 70%; margin: 0 auto;">
                        <h1 class="title-bar title is-size-1">
                            {"My Projects"}
                        </h1>
                        <h2 class="subtitle is-size-3" style="margin-top: 2rem;">
                            // {"I love thinkering and expaning my knowledge. One of the ways I do this is through personal projects. 
                            // I have done serveral projects such as Embedded programming in Rust, Ubuntu Homeserver using Ansible, "}
                            {"I love "}<strong>{"tinkering"}</strong>{" and expanding my knowledge across a wide range of disciplines such as, "}<strong>{"mechanical"}</strong>
                            {" systems, "}<strong>{"electronics"}</strong>{", "}<strong>{"programming"}</strong>{", and more."}
                        </h2>
                        <h2 class="subtitle is-size-3">
                        {"For me, the best way to learn is by "}<strong>{"building stuff"}</strong>{". I've worked on projects like "}<strong>{"autonomous"}
                        </strong>{" cars, embedded "}<strong>{"Rust"}</strong>{" programming, balancing "}<strong>{"robot"}</strong>{", and making this "}<strong>{"website"}</strong>{"."}
                            // Check out some of my projects below!"}
                        </h2>
                        // <h2 class="subtitle is-size-3">
                        //     {"Beyond my academic pursuits, I am passionate about "}<strong>{"skiing and mountain hiking"} </strong>{", 
                        //     enjoying the nature and the outdoors."}
                        // </h2>

                        <div style="
                            display: flex;
                            justify-content: center;
                            margin-top: 3rem;
                        ">
                        <a href="/projects" class="button is-link is-size-4 is-uppercase">
                            {"See My Projects"}
                        </a>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[function_component(AboutMe)]
fn about_me() -> Html {

    html!{
        <section class="section">
            <div class="columns is-vcentered">
                <div class="column">
                    <div style="width: 70%; margin: 0 auto;">
                        <h1 class="title-bar title is-size-1">
                            {"About Me"}
                        </h1>
                        <h2 class="subtitle is-size-3" style="margin-top: 2rem;">
                            {"I'm Tor Børve Rasmussen, and I am currently studying 
                            "}<strong>{"Control Systems and Robotics"}</strong>{" at NTNU. 
                            I am in the final year of my master's program and working on my thesis about "}
                            <strong>{"motion control"}</strong>{"."}
                        </h2>
                        <h2 class="subtitle is-size-3">
                            {"Beyond my academic pursuits, I am passionate about "}<strong>{"skiing"}</strong>{" and "}<strong>{"mountain hiking"} </strong>{", 
                            enjoying the nature and the outdoors."}
                        </h2>

                        <div style="
                            display: flex;
                            justify-content: center;
                            margin-top: 3rem;
                        ">
                        <a href="/contact" class="button is-link is-size-4 is-uppercase">
                            {"Contact Me"}
                        </a>
                        </div>
                    </div>
                </div>
                <div class="column">
                    <figure class="image">
                        <img src="static/auc_passed_technical.jpg" style="border-radius: 10px;"/>
                    </figure>
                </div>
            </div>
        </section>
    }
}

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div>
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
                max-width: 25%;
                text-align: left;
                wrap-word: break-word;
            ">
                <h1 class="title is-size-1">{"Control Systems and Robotics Engineer from Norway"} </h1>
                <a class="button is-link is-size-4 is-uppercase" href="/#about">
                    {"More about me"}
                </a>
            </div>
        </div>
        <br/>
        <br/>
        <br/>
        <div id="about">
            <AboutMe/>
        </div>
        <br/>
        <br/>
        // <br/>
        <ProjectsIntro/>
        <Footer/>
        </div>
    }
}