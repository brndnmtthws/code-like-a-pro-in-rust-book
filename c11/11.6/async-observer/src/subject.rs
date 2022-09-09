use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Weak},
};

use crate::{observable::Observable, observer::Observer};

pub struct Subject {
    observers: Vec<Weak<dyn Observer<Subject = Self, Output = ()>>>,
    state: String,
}

impl Subject {
    pub fn new(state: &str) -> Self {
        Self {
            observers: vec![],
            state: state.into(),
        }
    }

    pub fn state(&self) -> &str {
        self.state.as_ref()
    }
}

impl Observable for Subject {
    type Observer = Arc<dyn Observer<Subject = Self, Output = ()>>;
    fn update<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>> {
        let observers: Vec<_> =
            self.observers.iter().flat_map(|o| o.upgrade()).collect();

        Box::pin(async move {
            futures::future::join_all(
                observers.iter().map(|o| o.observe(self)),
            )
            .await;
        })
    }
    fn attach(&mut self, observer: Self::Observer) {
        self.observers.push(Arc::downgrade(&observer));
    }
    fn detach(&mut self, observer: Self::Observer) {
        self.observers
            .retain(|f| !f.ptr_eq(&Arc::downgrade(&observer)));
    }
}
