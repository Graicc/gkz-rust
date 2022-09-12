use reqwasm::http::Request;
use serde::de::DeserializeOwned;
use yew::{html, Component, Context, Html, Properties, virtual_dom::AttrValue, Callback};
use std::fmt;

use crate::BASE_URL;

pub mod map_table;
pub mod run_table;
pub mod user_table;

pub trait Column: fmt::Display + PartialEq {
    fn is_right_align(&self) -> bool;
}

pub trait TableData {
    type Column: Column;

    fn get_column(&self, column: &Self::Column) -> Html;
}

pub enum TableMessage<T> where T: TableData {
    Get,
    Response(Result<Vec<T>, reqwasm::Error>),
}

#[derive(Properties, PartialEq)]
pub struct TableProps<T> where T: Column {
    pub columns: Vec<T>,
    pub url: AttrValue,
}

pub struct Table<T> where T: TableData {
    // columns: Vec<Column>,
    data: Option<Result<Vec<T>, reqwasm::Error>>,
    url: AttrValue,
    get_callback: Callback<()>,
}

impl<T> Component for Table<T> where T: TableData + DeserializeOwned + 'static {
    type Message = TableMessage<T>;
    type Properties = TableProps<T::Column>;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let url = props.url.clone();

        let callback = ctx.link().callback(|_| TableMessage::Get);

        let new = Self { data: None, url: url.clone(), get_callback: callback };

        new.get_callback.emit(());

        new
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use TableMessage::*;
        match msg {
            Get => {
                let callback = ctx.link().callback(|data| TableMessage::Response(data));
                let url = ctx.props().url.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let message = async {
                        let res = Request::get(&format!("{BASE_URL}{url}")).send().await?;
                    
                        res.json::<Vec<T>>().await
                    }.await;

                    callback.clone().emit(message);
                });
                false
            },
            Response(data) => {
                self.data = Some(data);
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props();
        let url = props.url.clone();
        if url != self.url {
            self.url = url.clone();

            self.get_callback.emit(());
        }

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let props = _ctx.props();
        let columns = &props.columns;

        match &self.data {
            Some(Ok(data)) => {
                let header: Html = columns
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
                            columns.iter().map(|y| {
                                html! {
                                    <td class={if y.is_right_align() {"right"} else {""}}> {
                                        x.get_column(y)
                                    } </td>
                                }
                            }).collect::<Html>()
                        }
                        </tr>
                    }
                }).collect();

                html! {
                    <div class="data-table">
                        // <p>{format!("{BASE_URL}{}", self.url)}</p>
                        <table class="dark round">
                            <tr>
                                {header}
                            </tr>
                            {rows}
                        </table>
                    </div>
                }
            },
            Some(Err(_)) => {
                html! {
                    <div class="dark round">
                        <p>{"A error occoured fetching the table."}<br />{"Please try again later"}</p>
                    </div>
                }
            },
            None => {
                html! {
                    <h1>{"Loading..."}</h1>
                }
            }
        }
    }
}
