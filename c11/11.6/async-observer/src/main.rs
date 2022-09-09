use std::{future::Future, pin::Pin, sync::Arc};

use observable::Observable;
use observer::Observer;
use subject::Subject;

mod observable;
mod observer;
mod subject;

struct MyObserver {
    name: String,
}

impl MyObserver {
    fn new(name: &str) -> Arc<Self> {
        Arc::new(Self { name: name.into() })
    }
}
impl Observer for MyObserver {
    type Subject = Subject;
    type Output = ();
    fn observe<'a>(
        &'a self,
        subject: &'a Self::Subject,
    ) -> Pin<Box<dyn Future<Output = Self::Output> + 'a + Send>> {
        Box::pin(async {
            println!(
                "observed subject with state={:?} in {}",
                subject.state(),
                self.name
            );
        })
    }
}

#[tokio::main]
async fn main() {
    let mut subject = Subject::new("some subject state");

    let observer1 = MyObserver::new("observer1");
    let observer2 = MyObserver::new("observer2");

    subject.attach(observer1.clone());
    subject.attach(observer2.clone());

    // ... do something here ...

    subject.update().await;
}
