use tokio::net::TcpListener;
use tokio::sync::oneshot;

use once_cell::sync::Lazy;
use std::future::Future;
use tokio_util::context::TokioContext;
use std::sync::mpsc;
use std::sync::Arc;

mod executers {
    use super::*;

    // pub struct TaskSender {
    //     tx: mpsc::Sender<()>,
    //     rx: mpsc::Receiver<()>,
    // }

    // impl TaskSender {
    //     pub fn new(tx: mpsc::Sender<()>) -> Self {
    //         let (tx, rx) = mpsc::channel();

    //     }

    //     pub fn push<F>(&self, f: F)
    //     where
    //         F: Future<Output = ()> + Send + 'static,
    //     {
    //         self.tx.send(f);
    //     }

    //     pub fn execute(&self) {
    //         let task = self.rx.lock().unwrap().recv().unwrap();
    //         task();
    //     }
    // }


    pub fn spawn(f: impl Future<Output = ()> + Send + 'static) {
        EXECUTOR.spawn(f);
    }

    struct ThreadPool {
        inner: futures::executor::ThreadPool,
        rt: tokio::runtime::Runtime,
    }

    static EXECUTOR: Lazy<ThreadPool> = Lazy::new(|| {
        // Spawn tokio runtime on a single background thread
        // enabling IO and timers.
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        let inner = futures::executor::ThreadPool::builder().create().unwrap();

        ThreadPool { inner, rt }
    });

    impl ThreadPool {
        fn spawn(&self, f: impl Future<Output = ()> + Send + 'static) {
            let handle = self.rt.handle().clone();
            self.inner.spawn_ok(TokioContext::new(f, handle));
        }
    }
}



#[test]
fn test_multi_tasks() {
    let (tx, rx) = oneshot::channel();

    executers::spawn(async move {
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        println!("addr: {:?}", listener.local_addr());
        tx.send(()).unwrap();
    });

    futures::executor::block_on(rx).unwrap();
}
