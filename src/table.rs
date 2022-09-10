use chrono::{self, TimeZone};
use reqwasm::http::Request;
use serde::Deserialize;
use yew::{html, Component, Context, Html};
use yew_router::prelude::*;
use std::fmt;

use crate::Route;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct TableData {
    id: usize,
    user: String,

    #[serde(rename = "userID")]
    user_id: usize,

    map: String,

    #[serde(rename = "mapID")]
    map_id: usize,
    time: f64,
    checkpoint: usize,
    date: i64,
}

enum Column {
    ID,
    User,
    UserId,
    Map,
    MapId,
    Time,
    Checkpoint,
    Date,
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Column::*;
        match self {
            ID => write!(f, "ID"),
            User => write!(f, "User"),
            UserId => write!(f, "User ID"),
            Map => write!(f, "Map"),
            MapId => write!(f, "Map ID"),
            Time => write!(f, "Time"),
            Checkpoint => write!(f, "Checkpoints"),
            Date => write!(f, "Date")
        }
    }
}

impl Column {
    fn is_right_align(&self) -> bool {
        use Column::*;
        match self {
            ID => true,
            User => false,
            UserId => true,
            Map => false,
            MapId => true,
            Time => true,
            Checkpoint => true,
            Date => true,
        }
    }
}

pub enum TableMessage {
    Response(Result<Vec<TableData>, String>),
}

pub struct Table {
    columns: Vec<Column>,
    data: Option<Vec<TableData>>,
}

impl Component for Table {
    type Message = TableMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let new = Self { 
            columns: vec![
                Column::Date,
                Column::Map,
                Column::User,
                Column::Time,
            ],
            data: None };

        let callback = ctx.link().callback(|data| TableMessage::Response(data));
    
        wasm_bindgen_futures::spawn_local(async move {
            let res = Request::get("https://api.gorillakz.com/api/runs?max=5").send().await.unwrap();
            
            let res = res.json::<Vec<TableData>>().await.unwrap();
            callback.clone().emit(Ok(res));
        });

        new
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use TableMessage::*;
        match msg {
            Response(data) => {
                self.data = data.ok();
                self.data.is_some()
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if let Some(data) = &self.data {
            let header: Html = self
                .columns
                .iter()
                .map(|x| {
                    html! {
                        <th class={if x.is_right_align() {"right"} else {""}}>{x}</th>
                    }
                })
                .collect();

            let rows: Html = data.iter().map(|x| {
                html! {
                    <tr>
                    {
                        self.columns.iter().map(|y| {
                            html! {
                                <td class={if y.is_right_align() {"right"} else {""}}> {
                                    match *y {
                                        Column::ID => html! {x.id},
                                        Column::User => html! { <Link<Route> to={Route::User {id: x.user_id}}>{x.user.clone()}</Link<Route>> },
                                        Column::UserId => html! {x.user_id},
                                        Column::Map => html! { <Link<Route> to={Route::Map {id: x.map_id}}>{x.map.clone()}</Link<Route>> },
                                        Column::MapId => html! {x.map_id},
                                        Column::Time => html! {format!("{:.3}", x.time)},
                                        Column::Checkpoint => html! {x.checkpoint},
                                        Column::Date => html! {chrono::Utc.timestamp_millis(x.date).format("%Y/%m/%d")},
                                    }
                                } </td>
                            }
                        }).collect::<Html>()
                    }
                    </tr>
                }
            }).collect();

            html! {
                <div class="data-table">
                    <table class="dark round">
                        <tr>
                            {header}
                        </tr>
                        {rows}
                    </table>
                </div>
            }
        } else {
            html! {
                <h1>{"Loading..."}</h1>
            }
        }
    }
}
