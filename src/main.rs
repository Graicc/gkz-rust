use yew::prelude::*;
use yew_router::prelude::*;

mod table;
use table::*;

mod heading;
use heading::MonkeyHeading;

mod user;
use user::User;

mod map;
use map::Map;

const BASE_URL: &str = "https://api.gorillakz.com/api/";

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/users")]
    Users,
    #[at("/user/:id")]
    User { id: usize },
    #[at("/maps")]
    Maps,
    #[at("/map/:id")]
    Map { id: usize },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: &Route) -> Html {
    use map_table::*;
    use run_table::*;
    use user_table::*;

    match route {
        Route::Home => html! {
            <>
                <MonkeyHeading title="GorillaKZ" />
                <h3>{"Recent Times"}</h3>
                <Table<RunTableData> columns={vec![RunColumn::Date, RunColumn::Map, RunColumn::User, RunColumn::Time]} url={"runs?max=5"} />
            </>
        },
        Route::Users => html! {
            <>
                <MonkeyHeading title="Users" />
                <Table<UserTableData> columns={vec![UserColumn::RunCount, UserColumn::Name, UserColumn::Created]} url={"users?obsolete=true"} />
            </>
        },
        Route::User { id } => html! {
            <User id={*id}/>
        },
        Route::Maps => html! {
            <>
                <MonkeyHeading title="Maps" />
                <Table<MapTableData> columns={vec![MapColumn::RunCount, MapColumn::Name]} url={"maps"} />
            </>
        },
        Route::Map { id } => html! {
            <Map id={*id}/>
        },
        Route::NotFound => html! {
            <>
                <MonkeyHeading title="Page not found :(" />
                <h3>
                    <Link<Route> to={Route::Home}>{"Go home"}</Link<Route>>
                </h3>
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
                        <Link<Route> to={Route::Maps}>{"Maps"}</Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Users}>{"Users"}</Link<Route>>
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
                    <li><Link<Route> to={Route::Maps}>{"Maps"}</Link<Route>></li>
                    <li><Link<Route> to={Route::Users}>{"Users"}</Link<Route>></li>
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
        <div id="app" class="light page">
            <BrowserRouter>
                <Nav />
                <main>
                    <article class="content">
                        <Switch<Route> render={Switch::render(switch)} />
                    </article>
                </main>
                <br />
                <Footer />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
