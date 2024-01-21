use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, yew::Properties)]
struct View {
    vec: Vec<(defs::Id, defs::Todo)>,
}

#[function_component(AppMain)]
fn app_main(view: &View) -> Html {
    let html: Html = view
        .vec
        .iter()
        .map(|(_id, todo)| {
            if todo.0 {
                format!("o {:?}", todo.1)
            } else {
                format!("x {:?}", todo.1)
            }
        })
        .collect();
    html! {
        <>
        {html}
        </>
    }
}
