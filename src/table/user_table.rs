use chrono::{self, TimeZone};
use serde::Deserialize;
use std::fmt;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{table::*, Route};

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct UserTableData {
    pub id: usize,
    pub name: String,
    #[serde(rename = "runCount")]
    pub run_count: usize,
    pub created: i64,
}

#[derive(PartialEq)]
#[allow(unused)]
pub enum UserColumn {
    ID,
    Name,
    RunCount,
    Created,
}

impl fmt::Display for UserColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UserColumn::*;
        match self {
            ID => write!(f, "ID"),
            Name => write!(f, "User"),
            RunCount => write!(f, "Runs"),
            Created => write!(f, "Joined"),
        }
    }
}

impl Column for UserColumn {
    fn is_right_align(&self) -> bool {
        use UserColumn::*;
        match self {
            ID => true,
            Name => false,
            RunCount => true,
            Created => true,
        }
    }
}

impl TableData for UserTableData {
    type Column = UserColumn;

    fn get_column(&self, column: &Self::Column) -> Html {
        match *column {
            Self::Column::ID => html! {self.id},
            Self::Column::Name => {
                html! { <Link<Route> to={Route::User {id: self.id}}>{self.name.clone()}</Link<Route>> }
            }
            Self::Column::RunCount => html! {self.run_count},
            Self::Column::Created => {
                let time = chrono::Utc.timestamp_millis(self.created);
                html! { <span title={time.to_rfc2822()}> {time.format("%Y/%m/%d")} </span>}
            }
        }
    }
}
