mod date;
mod percentage;

use date::*;
use lib::components::InputResult;
use percentage::*;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let form_state: UseStateHandle<InputResult<Percentage>> = use_state(|| InputResult::Empty);
    let form_date: UseStateHandle<InputResult<String>> = use_state(|| InputResult::Empty);

    let button_click = {
        let form_state = form_state.clone();
        Callback::from(move |_e: MouseEvent| form_state.set(InputResult::Empty))
    };
    let date_click = {
        let form_date = form_date.clone();
        Callback::from(move |_e: MouseEvent| form_date.set(InputResult::Empty))
    };

    let on_date = {
        let form_date = form_date.clone();

        Callback::from(move |res: InputResult<String>| form_date.set(res))
    };

    let on_percentage = {
        let form_state = form_state.clone();

        Callback::from(move |res: InputResult<Percentage>| form_state.set(res))
    };

    let percentage_display = match form_state.get_parsing_error() {
        None => html!(),
        Some(err) => html!(<p class="text-red-600" > {err.get_error()} </p>),
    };
    let date_display = match form_date.get_parsing_error() {
        None => html!(),
        Some(err) => html!(<p class="text-red-600" > {err.get_error()} </p>),
    };

    html! {
        <div class="bg-slate-400 w-screen h-screen flex flex-col items-center justify-center">
            <InputPercentage
                percentage={(*form_state).clone()}
                on_input={on_percentage}
                placeholder={"t'as rien mis gros"}
            />

            {percentage_display}
            <button
                onclick={button_click}
                class="bg-blue-400 px-4 py-2 rounded-md " >
                {"click to set value of your state"}
            </button>
            <InputDate
                date={(*form_date).clone()}
                on_input={on_date}
            />
            {date_display}
            <button
                onclick={date_click}
                class="bg-blue-400 px-4 py-2 rounded-md " >
                {"date reset"}
            </button>

        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
