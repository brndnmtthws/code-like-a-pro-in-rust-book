use std::{future::Future, pin::Pin};

pub trait Observer: Send + Sync {
    type Subject;
    type Output;
    fn observe<'a>(
        &'a self,
        subject: &'a Self::Subject,
    ) -> Pin<Box<dyn Future<Output = Self::Output> + 'a + Send>>;
}
