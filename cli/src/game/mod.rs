use serde::{Deserialize, Serialize};
use sudokube::model::Grid;

use crate::config::Config;

#[derive(Serialize, Deserialize)]
struct Game<const N: usize> {
    pub config: Option<Config>,
    pub grid: Grid<N>,
}
