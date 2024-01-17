use axum::async_trait;
use std::sync::{Arc, Mutex};

use crate::application::ports::input_port::InputPort;

#[derive(Clone)]
pub struct MyService {
    pub count: Arc<Mutex<u8>>,
}

impl std::fmt::Debug for MyService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let count = match self.count.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                let guard = poisoned.into_inner();
                guard
            }
        };

        f.debug_struct("MyService").field("count", &count).finish()
    }
}

#[async_trait]
impl InputPort for MyService {
    // async fn process(&self) -> Result<(), Box<dyn std::error::Error>> {
    //     self.increment().await;
    //     println!("{:?}", self);
    //     Ok(())
    // }
    async fn process(&self) {
        self.increment().await;
        println!("{:?}", self);
    }
}

impl MyService {
    pub fn new() -> Self {
        Self {
            count: Arc::new(Mutex::new(0)),
        }
    }

    pub(crate) async fn increment(&self) {
        let mut count = match self.count.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                let guard = poisoned.into_inner();
                println!("Thread recovered from mutex poisoning: {:?}", *guard);
                guard
            }
        };

        *count = count.wrapping_add(1);
    }
}

impl Default for MyService {
    fn default() -> Self {
        Self::new()
    }
}
