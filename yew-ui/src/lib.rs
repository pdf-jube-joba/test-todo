use yew::prelude::*;

struct TodoView {}

#[derive(Debug, Clone, PartialEq, yew::Properties)]
struct TodoProp {
    id: defs::Id,
    todo: defs::Todo,
}

enum TodoMsg {}

impl Component for TodoView {
    type Message = TodoMsg;
    type Properties = TodoProp;
    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let todo = ctx.props();
        if todo.todo.0 {
            html! {format!("o {:?}", todo.todo.1)}
        } else {
            html! {format!("x {:?}", todo.todo.1)}
        }
    }
}

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
