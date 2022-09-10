use yew::prelude::*;
use yew_router::prelude::*;

mod table;
use table::*;

mod heading;
use heading::MonkeyHeading;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/user/:id")]
    User { id: usize },
    #[at("/map/:id")]
    Map { id: usize },
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
            <>
                <MonkeyHeading title="GorillaKZ" />
                <h3>{"Recent Times"}</h3>
                <Table />
            </>
        },
        Route::User { id } => html! {
            <>
                <MonkeyHeading title="GorillaKZ" />
                <h1>{id}</h1>
                <Table />
            </>
        },
        Route::Map { id } => html! {
            <>
                <MonkeyHeading title="GorillaKZ" />
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
                    <Link<Route> to={Route::Home}>
                        <img src="img/GKZLogo.png" alt="GKZ Logo" width="64px"/>
                    </Link<Route>>
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
                    <li><b><Link<Route> to={Route::Home}>{"GorillaKZ"}</Link<Route>></b></li>
                    <li><a href={GITHUB_URL}>{"Github"}</a></li>
                    <li><a href={DISCORD_URL}>{"Discord"}</a></li>
                </ul>
                <ul>
                    <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
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
