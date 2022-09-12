use reqwasm::http::Request;
use yew::prelude::*;

use crate::heading::MonkeyHeading;
use crate::table;
use crate::BASE_URL;

use table::map_table::MapTableData;

#[derive(Properties, PartialEq)]
pub struct MapProps {
    pub id: usize,
}

#[function_component(Map)]
pub fn map(props: &MapProps) -> Html {
    use table::run_table::{RunColumn::*, RunTableData};
    use table::Table;

    let id = props.id;

    let map = use_state(|| None);
    {
        let map = map.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let data = async {
                    let res = Request::get(&format!("{BASE_URL}map/{id}")).send().await?;
                
                    res.json::<MapTableData>().await
                }.await;

                map.set(Some(data));
            });
            || ()
        }, ())
    }

    let title = match map.as_ref() {
        Some(Ok(data)) => html! {
            <MonkeyHeading title={data.name.to_uppercase()} />
        },
        Some(Err(_)) => html! {
            <p>{"An error occoured fetching the map."}<br />{"Please try again later"}</p>
        },
        None => html! {
            <MonkeyHeading title="Loading..." />
        }
    };

    html! {
        <>
            {title}
            <Table<RunTableData> columns={vec![Date, User, Time]} url={format!("runs?map={id}")} />
        </>
    }
}
