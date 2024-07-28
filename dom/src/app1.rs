use stdweb::web::ClickEvent;
use stdweb::web::{document, window, IEventTarget};

pub fn main() {
    let btn = document().query_selector("#app1 button").unwrap().
        btn.add_event_listener( move |_: ClickEvent| {
            window().alert("assalamu alaykum, world!");
}
