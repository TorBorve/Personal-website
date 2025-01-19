use yew::prelude::*;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;


#[function_component(NavBar)]
fn nav_bar() -> Html {
    html! {
        <nav class="navbar is-size-3">
            <div class="navbar-brand">
                <a class="navbar-item" href="#">{"TBR"}</a>
            </div>
            <div class="navbar-menu">
                <div class="navbar-end">
                    <a class="navbar-item" href="#about">{"About"}</a>
                    <a class="navbar-item" href="#projects">{"Projects"}</a>
                    <a class="navbar-item" href="#contact">{"Contact"}</a>
                </div>
            </div>
        </nav>
    }
}

pub enum PlotMsg {
    Redraw,
    Nothing,
    TogglePlot,
}

#[derive(Properties, PartialEq)]
pub struct PlotProps {

}

pub struct Plot {
    canvas: NodeRef,
    is_plot1: bool,
}

impl Component for Plot {
    type Message = PlotMsg;
    type Properties = PlotProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(PlotMsg::Redraw);
        Plot {
            canvas: NodeRef::default(),
            is_plot1: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PlotMsg::Redraw => {
                let element : HtmlCanvasElement = self.canvas.cast().unwrap();

                let rect = element.get_bounding_client_rect();
                element.set_height(rect.height() as u32);
                element.set_width(rect.width() as u32);

                let backend = CanvasBackend::with_canvas_object(element).unwrap();

                let drawing_area = backend.into_drawing_area();
                // drawing_area.fill(&RGBColor(200,200,200)).unwrap();
                drawing_area.fill(&RGBAColor(255, 255, 255, 0.3)).unwrap();
                            
                let mut chart = ChartBuilder::on(&drawing_area)
                    .caption("y=x^2", ("sans-serif", 14).into_font())
                    .margin(5)
                    .x_label_area_size(30)
                    .y_label_area_size(30)
                    .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32).unwrap();
                
                chart.configure_mesh().draw().unwrap();

                let num_points = 1000; 
                if self.is_plot1 {

                    chart
                    .draw_series(LineSeries::new(
                        (-1*num_points/2..=1*num_points/2).map(|x| x as f32 / (num_points as f32/2.0)).map(|x| (x, x * x)),
                        &RED,
                    )).unwrap()
                    .label("y = x^2")
                    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
                } else {
                    chart
                    .draw_series(LineSeries::new(
                        (-1*num_points/2..=1*num_points/2).map(|x| x as f32 / (num_points as f32/2.0)).map(|x| (x, 0.5*((8.0*x).sin()) + 0.5)),
                        &RED,
                    )).unwrap()
                    .label("y = sin(x)")
                    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
                }
                false
            },
            PlotMsg::TogglePlot => {
                self.is_plot1 = !self.is_plot1;
                ctx.link().send_message(PlotMsg::Redraw);
                true
            }
            PlotMsg::Nothing => true,
        }
    }   

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| PlotMsg::TogglePlot)}>
                    {"Toggle Plot"}
                </button>
                <canvas ref = {self.canvas.clone()}/>
            </div>
        }
    }
}



#[function_component]
fn App() -> Html {
    // let counter = use_state(|| 0);
    // let onclick = {
    //     let counter = counter.clone();
    //     move |_| {
    //         let value = *counter + 1;
    //         counter.set(value);
    //     }
    // };

    html! {
        // <div>
        // </div>
        // <div>
        <section class="section" style="width: 100vw">
        // <div class="container">
        //     // <h1 class="title">
        //     // {"Hello World"}
        //     // </h1>
        //     // <p class="subtitle">
        //     // {"My first website with <strong>Bulma</strong>"}
        //     // </p>
        //     <h3>
        //      {"Red?"}
        //      </h3>
        // </div>      
        <NavBar/>
        <div class="container" style="
            background-image: url('static/tor_verbier.svg');
            background-position: 50% 80%;
            height: 100vh;
            // width: 1900px;
            max-width: 100%;
            background-size: cover;
            overflow: hidden;
            // align-items: centre;
            // font-family: Arial, sans-serif; 
            // text-align: center; 
            // margin: 2rem;
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
                <p href="#more" class="subtitle is-size-1">
                    {"More about me"}
                </p>
            </div>
        </div>
        // <div>
        //     <img src="static/portrait.jpg" alt="Picture of Tor" style="max-width: 200px" />
        //     <button class="button is-primary" {onclick}> {"+1"}</button>
        //     <p> {*counter} </p>
        //     <Plot/>
        //     <h2>{ "Projects" }</h2>
        //     <ul style="list-style: none; padding: 0;">
        //         <li><a href="https://github.com/torborve" target="_blank">{ "GitHub" }</a></li>
        //         <li><a href="https://linkedin.com/in/torborve" target="_blank">{ "LinkedIn" }</a></li>
        //     </ul>
        //     <footer class="footer">
        //         <div class="content has-text-centered">
        //             { "© 2025 Tor Børve Rasmussen" }
        //         </div>
        //     </footer>
        // </div>
        // </div>
        </section>
    }
}



fn main() {
    yew::Renderer::<App>::new().render();
}
