use tokio::task::{yield_now, JoinHandle};

async fn custom_task(id:i32, counter:i32, enable_yield_now: bool){
    for i in 0..counter {
        println!("[TASK:{id}]:Counting = {i}");
        if enable_yield_now {
            yield_now().await;
        }
    }
}


#[tokio::main]
async fn main(){
    let max_count = 10;
    let max_task = 10;
    let mut task_list: Vec<JoinHandle<()>> = Vec::new();

    // Let each task run first
    for i in 0..max_task  {
        let new_task = tokio::spawn(custom_task(i, max_count, true));
        task_list.insert(i as usize, new_task);
    }

    // then we will wait all task later...
    for i in 0..max_task {
        let each_task = task_list.get_mut(i as usize);
        if let Some(t) = each_task{
            t.await;
        } else {
            println!("<X> Task:{i} was not found!");
        }
    }
}