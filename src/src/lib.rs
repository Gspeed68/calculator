use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    yew::Renderer::<App>::new().render();
    Ok(())
}

#[derive(Clone, PartialEq)]
struct Calculation {
    num1: f32,
    num2: f32,
    operation: String,
    result: f32,
}

/// The main calculator component
#[function_component(Calculator)]
pub fn calculator() -> Html {
    // State for the calculator
    let num1 = use_state(|| String::new());
    let num2 = use_state(|| String::new());
    let operation = use_state(|| String::from("+"));
    let result = use_state(|| String::new());
    let history = use_state(|| Vec::<Calculation>::new());

    // Callback for number input
    let on_num1_change = {
        let num1 = num1.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            num1.set(input.value());
        })
    };

    let on_num2_change = {
        let num2 = num2.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            num2.set(input.value());
        })
    };

    // Callback for operation selection
    let on_operation_change = {
        let operation = operation.clone();
        Callback::from(move |e: Event| {
            let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
            operation.set(select.value());
        })
    };

    // Callback for clearing inputs
    let on_clear = {
        let num1 = num1.clone();
        let num2 = num2.clone();
        let result = result.clone();
        Callback::from(move |_| {
            num1.set(String::new());
            num2.set(String::new());
            result.set(String::new());
        })
    };

    // Callback for calculation
    let on_calculate = {
        let num1 = num1.clone();
        let num2 = num2.clone();
        let operation = operation.clone();
        let result = result.clone();
        let history = history.clone();
        Callback::from(move |_| {
            if let (Ok(n1), Ok(n2)) = (num1.parse::<f32>(), num2.parse::<f32>()) {
                let res = match operation.as_str() {
                    "+" => n1 + n2,
                    "-" => n1 - n2,
                    "*" => n1 * n2,
                    "/" => {
                        if n2 == 0.0 {
                            result.set("Error: Division by zero".to_string());
                            return;
                        }
                        n1 / n2
                    },
                    _ => {
                        result.set("Invalid operation".to_string());
                        return;
                    }
                };
                result.set(res.to_string());
                
                // Add to history
                let mut new_history = (*history).clone();
                new_history.push(Calculation {
                    num1: n1,
                    num2: n2,
                    operation: (*operation).clone(),
                    result: res,
                });
                history.set(new_history);

                // Clear input boxes
                num1.set(String::new());
                num2.set(String::new());
            } else {
                result.set("Invalid numbers".to_string());
            }
        })
    };

    html! {
        <div class="calculator">
            <h1>{ "Rust Calculator" }</h1>
            <div class="input-group">
                <input
                    type="text"
                    placeholder="First number"
                    value={(*num1).clone()}
                    onchange={on_num1_change}
                />
                <select
                    value={(*operation).clone()}
                    onchange={on_operation_change}
                >
                    <option value="+">{ "+" }</option>
                    <option value="-">{ "-" }</option>
                    <option value="*">{ "*" }</option>
                    <option value="/">{ "/" }</option>
                </select>
                <input
                    type="text"
                    placeholder="Second number"
                    value={(*num2).clone()}
                    onchange={on_num2_change}
                />
            </div>
            <div class="button-group">
                <button onclick={on_calculate}>{ "Calculate" }</button>
                <button onclick={on_clear} class="clear-button">{ "Clear" }</button>
            </div>
            <div class="result">
                { if !result.is_empty() {
                    html! { <h2 class="result-text">{ format!("Result: {}", *result) }</h2> }
                } else {
                    html! { <></> }
                }}
            </div>
            <div class="history">
                <h3>{ "History" }</h3>
                { for history.iter().rev().map(|calc| {
                    html! {
                        <div class="history-item">
                            { format!("{} {} {} = {}", calc.num1, calc.operation, calc.num2, calc.result) }
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

/// The main entry point for the Yew application
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="app">
            <Calculator />
        </div>
    }
} 