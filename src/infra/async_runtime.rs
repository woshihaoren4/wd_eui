use std::future::Future;
use std::pin::Pin;

pub struct Runtime {
    handle: Vec<Pin<Box<dyn Future<Output = ()> + Send + Sync>>>,
    rt: tokio::runtime::Runtime,
}

impl Runtime {
    pub fn new() -> Self {
        let handle = vec![];
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("async runtime build failed");
        Self { handle, rt }
    }
    pub fn add_task<F: Future<Output = ()> + Send + Sync + 'static>(mut self, task: F) -> Self {
        self.handle.push(Box::pin(task));
        self
    }
    pub fn run(self) {
        std::thread::spawn(move || {
            let Self { handle, rt } = self;

            rt.block_on(async move {
                for i in handle {
                    tokio::spawn(i);
                }
                if let Err(err) = tokio::signal::ctrl_c().await {
                    println!("shutdown error:{}", err);
                }
            });
            rt.shutdown_background();
        });
    }
}
