use chrono::{self, TimeZone};
use serde::Deserialize;
use std::fmt;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{table::*, Route};

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct RunTableData {
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

#[derive(PartialEq)]
#[allow(unused)]
pub enum RunColumn {
    ID,
    User,
    UserId,
    Map,
    MapId,
    Time,
    Checkpoint,
    Date,
}

impl fmt::Display for RunColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RunColumn::*;
        match self {
            ID => write!(f, "ID"),
            User => write!(f, "User"),
            UserId => write!(f, "User ID"),
            Map => write!(f, "Map"),
            MapId => write!(f, "Map ID"),
            Time => write!(f, "Time"),
            Checkpoint => write!(f, "Checkpoints"),
            Date => write!(f, "Date"),
        }
    }
}

impl Column for RunColumn {
    fn is_right_align(&self) -> bool {
        use RunColumn::*;
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

impl TableData for RunTableData {
    type Column = RunColumn;

    fn get_column(&self, column: &Self::Column) -> Html {
        match *column {
            Self::Column::ID => html! {self.id},
            Self::Column::User => {
                html! { <Link<Route> to={Route::User {id: self.user_id}}>{self.user.clone()}</Link<Route>> }
            }
            Self::Column::UserId => html! {self.user_id},
            Self::Column::Map => {
                html! { <Link<Route> to={Route::Map {id: self.map_id}}>{self.map.clone()}</Link<Route>> }
            }
            Self::Column::MapId => html! {self.map_id},
            Self::Column::Time => html! {format!("{:.3}", self.time)},
            Self::Column::Checkpoint => html! {self.checkpoint},
            Self::Column::Date => {
                let time = chrono::Utc.timestamp_millis(self.date);
                html! { <span title={time.to_rfc2822()}> {time.format("%Y/%m/%d")} </span>}
            }
        }
    }
}
