mod web;

use crate::web::{Request, RequestBuilder};

#[derive(Debug)]
struct Task {
    title: String,
    done: bool,
    desc: Option<String>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            done: false,
            desc: None,
        }
    }
}

impl Task {
    fn new(title: impl Into<String>) -> Task {
        Task {
            title: title.into(),
            done: false,
            desc: None,
        }
    }
}

fn main() -> Result<(), String>{
    let task1 = Task {
        title: "Hello world".to_string(),
        done: false,
        desc: Some("What's up".to_string()),
    };

    let task2 = Task::new("Build a robot");
    let task3 = Task::default();

    let task4: Option<Task> = None;
    println!("{:#?}", task4.unwrap_or_default());

    let task5 = Task {
        done: true,
        ..Default::default()
    };

    println!("{:#?}", task1);
    println!("{:#?}", task2);
    println!("{:#?}", task3);
    println!("{:#?}", task5);

    let mut req_builder = RequestBuilder::new().url("https://google.com")
        .method("GET")
        .header("token", "dsafdf22-2342sadf-afdsaf");
    let req_builder = req_builder.header("ds", "dafsd");
    let req = req_builder.clone().build()?;
    println!("{req:#?}");

    let req = req_builder.header("T", "v").build()?;
    println!("{req:#?}");
    

    Ok(())
}
