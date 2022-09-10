use yew::prelude::*;
use yew_router::prelude::*;

mod table;
use table::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/user/:id")]
    Post { id: usize },
    #[at("/map/:id")]
    Map { id: usize },
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
            <>
                <h1>{"GorillaKZ"}</h1>
                <Table />
            </>
        },
        Route::Post { id } => html! {
            <>
                <h1>{"GorillaKZ"}</h1>
                <h1>{id}</h1>
                <Table />
            </>
        },
        Route::Map { id } => html! {
            <>
                <h1>{"GorillaKZ"}</h1>
                <h1>{id}</h1>
                <Table />
            </>
        },
    }
}

const DISCORD_URL: &str = "https://discord.gg/NcNK7NeqYm";
const GITHUB_URL: &str = "https://github.com/Graicc/GorillaKZ";

#[function_component(Nav)]
fn nav() -> Html {
    html! {
        <nav class="dark">
            <div class="content nav-block">
                <div class="nav-left">
                    <a href="/">
                        <img src="GKZLogo.png" width="64px"/>
                    </a>
                </div>
                <ul class="nav-right">
                    <li>
                        <a href="/maps">{"Maps"}</a>
                    </li>
                    <li>
                        <a href="/users">{"Users"}</a>
                    </li>
                    <li>
                        <a href={DISCORD_URL}>{"Discord"}</a>
                    </li>
                </ul>
            </div>
        </nav>
    }
}

#[function_component(Footer)]
fn footer() -> Html {
    use chrono::prelude::*;
    html! {
        <footer class="dark">
            <div class="content nav-block">
                <ul>
                    <li><b><a href="/">{"GorillaKZ"}</a></b></li>
                    <li><a href={GITHUB_URL}>{"Github"}</a></li>
                    <li><a href={DISCORD_URL}>{"Discord"}</a></li>
                </ul>
                <ul>
                    <li><a href="/">{"Home"}</a></li>
                    <li><a href="/maps">{"Maps"}</a></li>
                    <li><a href="/users">{"Users"}</a></li>
                </ul>
                <ul>
                    <li>{"Made with 🍌 by "}<b><a href="https://github.com/Graicc">{"Graic"}</a></b></li>
                    <li>{format!("©{} Graic", Utc::now().year())}</li>
                    <li>{"For legal inquires, contact legal@gorillakz.com"}</li>
                </ul>
            </div>
        </footer>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="light">
            <div id="app">
                <div class="page">
                    <Nav />
                    <main>
                        <article class="content">
                            <BrowserRouter>
                                <Switch<Route> render={Switch::render(switch)} />
                            </BrowserRouter>
                        </article>
                    </main>
                    <br />
                    <Footer />
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
