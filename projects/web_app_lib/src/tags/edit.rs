// use database::tags_query;
use leptos::prelude::*;

// use crate::keycloak::AuthClient;

#[component]
pub fn EditTags() -> impl IntoView {
    // let add_articles = ServerMultiAction::<AddArticles>::new();
    // let links = RwSignal::new(String::new());
    //
    // let submission = add_articles.last_submission_signal();
    // let is_pending = submission.pending();
    // let state = submission.state();
    //
    // Effect::watch(
    //     state,
    //     move |val, _, _| {
    //         if let Some(Ok(())) = val {
    //             links.set(String::new())
    //         }
    //     },
    //     false,
    // );
    //
    // view! {
    //     <CenterColumn>
    //         <ExpectAuth>
    //             <MultiActionForm action=add_articles>
    //                 { move || match is_pending.get() {
    //                     true => CenteredLoader().into_any(),
    //                     false => AddForm(AddFormProps { links}).into_any(),
    //                 }}
    //             </MultiActionForm>
    //         </ExpectAuth>
    //     </CenterColumn>
    // }
    view! {
        <div> "hello" </div>
    }
}

// #[server(
//     client = AuthClient
// )]
// async fn add_tag(name: String, description: String) -> Result<(), ServerFnError> {
//     use crate::ServerState;
//
//     if name.is_empty() {
//         return Err(ServerFnError::Args(String::from("name is empty")));
//     }
//     if description.is_empty() {
//         return Err(ServerFnError::Args(String::from("description is empty")));
//     }
//
//     let state = expect_context::<ServerState>();
//
//     if tags_query::name_exists(&state.db, &name).await? {
//         return Err(ServerFnError::Args(format!(
//             "the name '{name}' is already used"
//         )));
//     }
//
//     let articles = file_contents
//         .lines()
//         .map(|line| {
//             let line = String::from(line);
//             let title = get_title_from_url(line.clone());
//             Article::from_parts(title, line)
//         })
//         .collect();
//
//     Ok(articles_query::insert_many(articles, &state.db).await?)
// }
