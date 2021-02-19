use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex, RwLock},
    task::{Context, Poll, Waker},
};

pub type JsTasksMap = Arc<RwLock<HashMap<String, Arc<Mutex<JsTaskSharedState>>>>>;

#[derive(Debug)]
pub struct JsTask {
    pub id: String,
    pub state: Arc<Mutex<JsTaskSharedState>>,
}

#[derive(Debug)]
pub struct JsTaskSharedState {
    pub completed: bool,
    pub data: Option<String>,
    pub waker: Option<Waker>,
}

impl Future for JsTask {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        if state.completed {
            return Poll::Ready(());
        }
        state.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

impl JsTask {
    pub fn new(id: String) -> Self {
        let state = Arc::new(Mutex::new(JsTaskSharedState {
            completed: false,
            waker: None,
            data: None,
        }));

        JsTask { id, state }
    }
}
