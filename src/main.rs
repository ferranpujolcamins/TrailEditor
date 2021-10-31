use futures::executor::block_on;
use leaflet::LatLng;
use rfd::AsyncFileDialog;
use seed::{prelude::*, *};

use trail_editor::view::TopBar;

fn main() {
    App::start("app", init, update, view);
    let i: TopBar;
}

type Model = i32;
#[derive(Copy, Clone)]
enum Msg {
    Increment,
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    // Cannot initialize Leaflet until the map element has rendered.
    orders.after_next_render(init_map);
    orders.after_next_render(open_file_dialog);
    Model::default()
}

fn open_file_dialog(reader_info: RenderInfo) {
    let future = _open_file_dialog(reader_info);
    block_on(future);
}

async fn _open_file_dialog(_: RenderInfo) {
    let file = AsyncFileDialog::new()
        .add_filter("gpx", &["gpx"])
        .pick_file()
        .await;

    if let Some(file) = file {
        let data = file.read().await;
    }
}

fn init_map(_: RenderInfo) {
    let map = leaflet::Map::new("map", &JsValue::NULL);
    map.setView(&LatLng::new(63.5, 10.5), 5.0);
    leaflet::TileLayer::new(
        "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
        &JsValue::NULL,
    )
    .addTo(&map);
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        button![model, ev(Ev::Click, |_| Msg::Increment),],
        div![id!["map"]]
    ]
}
