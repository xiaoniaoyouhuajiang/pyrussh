use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;
use crate::Client;
use crate::SshConfig;

pub struct SshPool {
    clients: HashMap<String, Arc<Mutex<Client>>>,
    pool: Option<ThreadPool>
}

impl SshPool {
    // todo: concurrent connecting
    pub fn new(configs: &Vec<SshConfig>, thread_num: u64) -> Self {
        let mut clients = HashMap::new();
        for config in configs {
            let client = Client::new_session(
                config.host.as_str(), 
                config.port.as_str(), 
                config.user.as_str(), 
                config.passwd.as_str()
            ).unwrap();
            clients.insert(config.host.clone(), Arc::new(Mutex::new(client)));
        }
        Self { clients, pool: Some(ThreadPool::new(thread_num as usize)) }
    }

    pub fn run_command(&mut self, command: &str) -> HashMap<String, (String, i32)> {
        let (tx, rx) = std::sync::mpsc::channel();
        self.clients.iter().for_each(|(host, client)| {
            // Prepare a task closure responsible for sending the result of the operation.
            let (client, host, tx) = (client.clone(), host.clone(), tx.clone());
            let command = command.to_string();
            let task_closure = move || {
                let mut client = client.lock().expect("Failed to lock client");
                let result = client.run_command(&command);
                let _ = tx.send((host, result));
            };

            // Execute the task closure in the thread pool or spawn it in its own thread.
            if let Some(pool) = &self.pool {
                pool.execute(task_closure)
            } else {
                std::thread::spawn(task_closure);
            }
        });

        drop(tx);

        rx.iter().fold(HashMap::new(), |mut acc, (host, result)| {
            let (stdout, exit_code) = result.unwrap();
            acc.insert(host.clone(), (stdout, exit_code));
            acc
        })
    }
}

