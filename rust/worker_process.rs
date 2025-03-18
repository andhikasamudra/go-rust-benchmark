use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const WORKER_COUNT: usize = 10;
const QUEUE_SIZE: usize = 100;

#[derive(Clone)]
struct TaskQueue {
    queue: Arc<Mutex<Vec<i32>>>,
}

impl TaskQueue {
    fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(Vec::with_capacity(QUEUE_SIZE))),
        }
    }

    fn push(&self, task: i32) -> bool {
        let mut queue = self.queue.lock().unwrap();
        if queue.len() < QUEUE_SIZE {
            queue.push(task);
            true
        } else {
            false
        }
    }

    fn pop(&self) -> Option<i32> {
        let mut queue = self.queue.lock().unwrap();
        if !queue.is_empty() {
            Some(queue.remove(0))
        } else {
            None
        }
    }
}

async fn process_task(queue: web::Data<TaskQueue>) -> impl Responder {
    let task = chrono::Utc::now().timestamp_nanos() as i32;
    if queue.push(task) {
        HttpResponse::Ok().json(serde_json::json!({ "status": "queued" }))
    } else {
        HttpResponse::ServiceUnavailable().json(serde_json::json!({ "error": "Queue full" }))
    }
}

fn worker(queue: TaskQueue, id: usize) {
    loop {
        if let Some(task) = queue.pop() {
            thread::sleep(Duration::from_millis(50));
            println!("Worker {} processed task {}", id, task);
        } else {
            thread::sleep(Duration::from_millis(10));
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let queue = TaskQueue::new();
    let queue_clone = queue.clone();

    // Start worker pool
    for i in 0..WORKER_COUNT {
        let worker_queue = queue_clone.clone();
        thread::spawn(move || worker(worker_queue, i));
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(queue.clone()))
            .route("/process", web::get().to(process_task))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
