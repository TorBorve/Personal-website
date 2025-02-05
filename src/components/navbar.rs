use yew::prelude::*;

use gloo_storage::{LocalStorage, Storage};

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    let theme = use_state(|| {
        LocalStorage::get::<String>("theme")
            .unwrap_or_else(|_| "system".to_string())
    });

    let set_theme = {
        let theme = theme.clone();
        Callback::from(move |new_theme: String| {
            LocalStorage::set("theme", &new_theme)
                .expect("Failed to set theme");

            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .document_element()
                .unwrap()
                .set_attribute("data-theme", &new_theme)
                .unwrap();

            theme.set(new_theme);
        })
    };

    // Set at the beginning
    use_effect_with((*theme).clone(), move |theme| {
        web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .document_element()
                .unwrap()
                .set_attribute("data-theme", &theme)
                .unwrap();
    }); 

    let sun_icon = html! {
        <span class="icon is-sun">
            <i class="fas fa-sun" aria-hidden="true"></i>
        </span>
    };
    let moon_icon = html! {
        <span class="icon is-moon">
            <i class="fas fa-moon" aria-hidden="true"></i>
        </span>
    };
    let system_icon = html! {
        <span class="icon is-system">
            <i class="fas fa-desktop" aria-hidden="true"></i>
        </span>
    };

    let current_theme_icon = {
        if *theme == "light" {
            sun_icon.clone()
        } else if *theme == "dark" {
            moon_icon.clone()
        } else {
            system_icon.clone()
        }
    };

    html! {
        <nav class="navbar is-size-4">
            <div class="navbar-brand">
                <a class="navbar-item" href="/">{"TBR"}</a>
            </div>
            <div class="navbar-menu">
                <div class="navbar-end">
                    <a class="navbar-item" href="/#about">{"About"}</a>
                    <a class="navbar-item" href="/projects">{"Projects"}</a>
                    <a class="navbar-item" href="/contact">{"Contact"}</a>
                    <div class="navbar-item has-dropdown is-hoverable mr-4 ml-4">
                        <div class="navbar-link is-arrowless">
                            {current_theme_icon}
                        </div>
                        <div class="navbar-dropdown is-size-4 is-right">
                        <a class="navbar-item is-sun" onclick={set_theme.reform(|_| {
                            "light".to_string()
                        })}>
                            {sun_icon}
                            <span>{"Light"}</span>
                        </a>

                        <a class="navbar-item" onclick={set_theme.reform(|_| {
                            "dark".to_string()
                        })}>
                            {moon_icon}
                            <span>{"Dark"}</span>
                        </a>

                        <a class="navbar-item" onclick={set_theme.reform(|_| {
                            "system".to_string()
                        })}>
                            {system_icon}
                            <span>{"System"}</span>
                        </a>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}
