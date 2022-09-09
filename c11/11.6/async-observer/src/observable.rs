use std::{future::Future, pin::Pin};

pub trait Observable {
    type Observer;
    fn update<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>>;
    fn attach(&mut self, observer: Self::Observer);
    fn detach(&mut self, observer: Self::Observer);
}
