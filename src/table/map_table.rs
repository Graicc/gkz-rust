use serde::Deserialize;
use std::fmt;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{table::*, Route};

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct MapTableData {
    pub id: usize,
    pub name: String,
    #[serde(rename = "runCount")]
    pub run_count: usize,
}

#[derive(PartialEq)]
#[allow(unused)]
pub enum MapColumn {
    ID,
    Name,
    RunCount,
}

impl fmt::Display for MapColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MapColumn::*;
        match self {
            ID => write!(f, "ID"),
            Name => write!(f, "Map"),
            RunCount => write!(f, "Runs"),
        }
    }
}

impl Column for MapColumn {
    fn is_right_align(&self) -> bool {
        use MapColumn::*;
        match self {
            ID => true,
            Name => false,
            RunCount => true,
        }
    }
}

impl TableData for MapTableData {
    type Column = MapColumn;

    fn get_column(&self, column: &Self::Column) -> Html {
        match *column {
            Self::Column::ID => html! {self.id},
            Self::Column::Name => {
                html! { <Link<Route> to={Route::Map {id: self.id}}>{self.name.clone()}</Link<Route>> }
            }
            Self::Column::RunCount => html! {self.run_count},
        }
    }
}
