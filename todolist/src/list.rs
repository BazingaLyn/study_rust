use crate::format::Todo;
use crate::string::StringExt;
use std::error;
use std::io::BufRead;

const TODO: &str = "\u{2610}";
const DONE: &str = "\u{2611}";


struct List(Todo);


impl List {
    fn serialize(&self, index: u32) -> String {
        if self.0.done && self.0.time.is_some() {
            format!(
                "{} {:03}: {} ({:.1})\n",
                DONE,
                index,
                self.0.task,
                self.0.time.unwrap_or(0.0)
            )
                .cyan()
        } else if self.0.done {
            format!("{} {:03}: {}\n", DONE, index, self.0.task).cyan()
        } else if !self.0.done && self.0.time.is_some() {
            format!(
                "{} {:03}: {} ({:.1})\n",
                TODO,
                index,
                self.0.task,
                self.0.time.unwrap_or(0.0)
            )
        } else {
            format!("{} {:03}: {}\n", TODO, index, self.0.task)
        }
            .bold()
    }
}

pub(crate) fn list<R: BufRead>(reader: &mut R) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
    let mut w = String::new();

    let index = 0;
    let mut l = String::new();
    while reader.read_line(&mut l)? > 0 {
        let todo =
    }
}