use serde_wasm_bindgen::to_value;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod fitty {
    use js_sys::Function;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_duck_type::*;
    use web_sys::Element;

    #[derive(serde::Serialize)]
    pub struct FittyProps {
        #[serde(rename = "maxSize")]
        pub max_size: i32,
    }

    #[wasm_bindgen]
    extern "C" {
        pub fn fitty(s: &str, p: &JsValue) -> Vec<FittyResult>;
        // #[wasm_bindgen(js_namespace = console)]
        // pub fn log(s: &JsValue);
        // #[wasm_bindgen(js_namespace = console, js_name = log)]
        // pub fn log_str(s: &str);
    }

    #[wasm_bindgen_duck_type]
    struct FittyResult {
        element: Element,
        fit: Function,
        unsubscribe: Function,
    }
}

use fitty::*;

#[derive(Properties, PartialEq)]
pub struct MonkeyHeadingProps {
    pub title: String,
}

pub enum MonkeyHeadingMessage {
    Set(Vec<FittyResult>),
}

pub struct MonkeyHeading {
    fitty_results: Vec<FittyResult>,
}

impl Component for MonkeyHeading {
    type Message = MonkeyHeadingMessage;

    type Properties = MonkeyHeadingProps;

    fn create(_ctx: &Context<Self>) -> Self {
        MonkeyHeading {
            fitty_results: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MonkeyHeadingMessage::Set(f) => {
                self.fitty_results = f;
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let MonkeyHeadingProps { title } = ctx.props();

        let set = ctx.link().callback(|x| MonkeyHeadingMessage::Set(x));

        let fitty_callback = move |_| {
            let props = FittyProps { max_size: 128 };

            let x = fitty(".fitty-resize", &to_value(&props).unwrap());

            set.emit(x);
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
}

impl Drop for MonkeyHeading {
    fn drop(&mut self) {
        self.fitty_results.iter().for_each(|res| {
            _ = res.unsubscribe().call0(&JsValue::NULL);
        });
    }
}
