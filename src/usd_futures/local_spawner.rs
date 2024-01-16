use {
    hyper::client::connect::Connect,
    std::{sync::Arc, thread::JoinHandle},
    super::Futures,
    tokio::{
        runtime::Builder, sync::mpsc, task::LocalSet,
    }
};

#[derive(Debug)]
pub enum Task {
    market_order(f32),
    shutdown,
}

pub struct LocalSpawner {
    send: mpsc::UnboundedSender<Task>,
    handle: JoinHandle<()>,
}

impl LocalSpawner
{
    pub fn new<T>(fu: Arc<Futures<T>>) -> Self
        where
            T: Connect + Clone + Send + Sync + 'static,
    {
        let (send, mut recv) = mpsc::unbounded_channel::<Task>();

        let rt = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let handle = std::thread::spawn(move || {
            let local = LocalSet::new();
            local.spawn_local(async move {

                while let Some(task) = recv.recv().await {
                    let fu = Arc::clone(&fu);
                    match task {
                        Task::market_order(qty) => {
                            tokio::task::spawn_local(async move {
                                fu.market_order(qty).await;
                            });
                        }
                        Task::shutdown => break,
                    }
                }
            });

            rt.block_on(local);
        });

        Self {
            send,
            handle
        }
    }

    pub fn spawn(&self, task: Task) {
        self.send.send(task).expect("Thread with LocalSet has shut down.");
    }

    pub fn join(mut self) {
        self.send.send(Task::shutdown);
        self.handle.join().expect("Couldn't join LocalSpawner thread");
    }
}

