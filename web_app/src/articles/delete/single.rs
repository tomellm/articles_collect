use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use tracing::info;
use uuid::Uuid;

use crate::{
    articles::delete::delete_article,
    utils::{
        busy_container::BusyAction,
        dialog::{DialogSignal, DialogState},
    },
};

/// This function creats an action that will open the dialog and give the option
/// to delete an article with the selected uuid.
///
/// Meaning the following steps for usage are:
/// - dispatch returned action when user clicks
/// - return value is another action whos
pub fn open_delete_dialog_action(navigate_to: Option<String>) -> DeleteDialogAction {
    let navigate_to = navigate_to.unwrap_or("/".into());

    DeleteDialogAction::new(navigate_to)
}

#[derive(Clone, Copy)]
pub struct DeleteDialogAction {
    is_open: RwSignal<bool>,
    dialog_action: Action<Uuid, ()>,
    _delete_action: Action<Uuid, ()>,
    _last_err: RwSignal<Option<String>>,
}

impl DeleteDialogAction {
    pub fn new(navigate_to: String) -> Self {
        let dialog = expect_context::<DialogSignal>();
        let is_open = RwSignal::new(false);
        let last_err = RwSignal::new(None);

        let delete_action = Action::new(move |uuid: &Uuid| {
            let uuid = *uuid;
            let navigate_to = navigate_to.clone();
            let navigate = use_navigate();
            async move {
                match delete_article(uuid).await {
                    Ok(_) => {
                        is_open.set(false);
                        navigate(&navigate_to, Default::default());
                        last_err.set(None);
                    }
                    Err(err) => {
                        is_open.set(false);
                        last_err.set(Some(err.to_string()));
                    }
                }
            }
        });

        let dialog_action = Action::new(move |uuid: &Uuid| {
            let uuid = *uuid;
            async move {
                dialog.open(DialogState::yes_no(
                    move || {
                        delete_action.dispatch(uuid);
                    },
                    move || {
                        info!("close....");
                        is_open.set(false);
                    },
                    "Delete Item?",
                    "Do you really want to delete this Item?",
                ));
            }
        });

        Self {
            is_open,
            dialog_action,
            _delete_action: delete_action,
            _last_err: last_err,
        }
    }

    pub fn open_dialog(&self, uuid: Uuid) {
        self.is_open.set(true);
        self.dialog_action.dispatch(uuid);
    }
}

impl BusyAction for DeleteDialogAction {
    fn is_busy(&self) -> bool {
        *self.is_open.read()
    }

    fn busy_text(&self) -> String {
        String::from("Waiting for User input....")
    }
}
