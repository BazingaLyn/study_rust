const PARSE_ERROR: &str = "failed to parse";

pub(crate) struct Todo {
    pub done: bool,
    pub task: String,
    pub time: Option<f32>,
}

