use monaco::yew::Editor;
use wasm_bindgen::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

struct App;
impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Editor />
        }
    }
}

#[wasm_bindgen(start)]
pub fn start_app() {
    monaco::init_environment();
    yew::start_app::<App>();
}