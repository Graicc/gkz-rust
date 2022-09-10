use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(serde::Serialize)]
struct FittyProps {
    #[serde(rename = "maxSize")]
    max_size: i32,
}

#[wasm_bindgen]
extern "C" {
    fn fitty(s: &str, p: &JsValue);
}

#[derive(Properties, PartialEq)]
pub struct MonkeyHeadingProps {
    pub title: String,
}

#[function_component(MonkeyHeading)]
pub fn monkey_heading(MonkeyHeadingProps { title }: &MonkeyHeadingProps) -> Html {
    let fitty_callback = |_| {
        let props = FittyProps { max_size: 128 };
        fitty(".fitty-resize", &JsValue::from_serde(&props).unwrap())
    };

    html! {
        <>
            <img src="img/monke.svg" class="monke-img" onload={fitty_callback}/>
            <div width="100%">
                <h1 class="fitty-resize">{title}</h1>
            </div>
        </>
    }
}
