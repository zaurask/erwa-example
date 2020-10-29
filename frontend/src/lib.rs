use mytodo::{JsonApiResponse, Task};
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::FetchedTasks(fetch_drills().await) });
    Model { tasks: vec![] }
}

async fn fetch_drills() -> fetch::Result<JsonApiResponse> {
    fetch("http://localhost:8000/tasks/")
        .await?
        .check_status()?
        .json()
        .await
}

// ------ ------
//     Model
// ------ ------

struct Model {
    tasks: Vec<Task>,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    FetchedTasks(fetch::Result<JsonApiResponse>),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedTasks(Ok(json_api_response)) => {
            model.tasks = json_api_response.data;
        }
        Msg::FetchedTasks(Err(fetch_error)) => {
            error!("Error fetching: ", fetch_error);
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    h1![
        "Tasks",
        ul![model.tasks.iter().map(|task| { li![&task.title] })],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::start("app", init, update, view);
}
