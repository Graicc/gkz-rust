use reqwasm::http::Request;
use yew::prelude::*;

use crate::heading::MonkeyHeading;
use crate::table;
use crate::BASE_URL;

use table::user_table::UserTableData;

#[derive(Properties, PartialEq)]
pub struct UserProps {
    pub id: usize,
}

#[function_component(User)]
pub fn user(props: &UserProps) -> Html {
    use table::run_table::{RunColumn::*, RunTableData};
    use table::Table;

    let id = props.id;

    let user = use_state(|| None);
    {
        let user = user.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let data = async {
                    let res = Request::get(&format!("{BASE_URL}user/{id}")).send().await?;
                
                    res.json::<UserTableData>().await
                }.await;

                user.set(Some(data));
            });
            || ()
        }, ())
    }

    let title = match user.as_ref() {
        Some(Ok(data)) => html! {
            <MonkeyHeading title={data.name.clone()} />
        },
        Some(Err(_)) => html! {
            <p>{"An error occoured fetching the user."}<br />{"Please try again later"}</p>
        },
        None => html! {
            <MonkeyHeading title="Loading..." />
        }
    };

    html! {
        <>
            {title}
            <Table<RunTableData> columns={vec![Date, Map, Time]} url={format!("runs?user={id}")} />
        </>
    }
}
