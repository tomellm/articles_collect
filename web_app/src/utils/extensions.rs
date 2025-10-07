use leptos::{prelude::*, server_fn::ServerFn};

pub type MultiactionLastSubSignal<S> = Signal<Option<ArcSubmission<S, Result<(), ServerFnError>>>>;

pub trait ServerMultiActionExtensions<S>
where
    S: ServerFn<Output = (), Error = ServerFnError> + Sync + 'static,
    S::Output: Sync + 'static,
{
    fn last_submission_signal(&self) -> MultiactionLastSubSignal<S>;
}

impl<S> ServerMultiActionExtensions<S> for ServerMultiAction<S>
where
    S: ServerFn<Output = (), Error = ServerFnError> + Sync + 'static,
    S::Output: Sync + 'static,
{
    fn last_submission_signal(&self) -> MultiactionLastSubSignal<S> {
        let action = *self;
        Signal::derive(move || action.submissions().read().last().map(|sub| sub.to_owned()))
    }
}

pub trait MultiactionLastSubSignalExtensions<S>
where
    S: ServerFn<Output = (), Error = ServerFnError> + Sync + 'static,
    S::Output: Sync + 'static,
{
    fn pending(&self) -> Signal<bool>;
    fn state(&self) -> Signal<Option<Result<(), ServerFnError>>>;
}

impl<S> MultiactionLastSubSignalExtensions<S> for MultiactionLastSubSignal<S>
where
    S: ServerFn<Output = (), Error = ServerFnError> + Sync + 'static,
    S::Output: Sync + 'static,
{
    fn pending(&self) -> Signal<bool> {
        let submission = *self;
        Signal::derive(move || submission.get().map(|s| s.pending().get()).unwrap_or(false))
    }

    fn state(&self) -> Signal<Option<Result<(), ServerFnError>>> {
        let submission = *self;
        Signal::derive(move || submission.get().and_then(|s| s.value().get()))
    }
}
