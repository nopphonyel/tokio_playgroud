use tokio::task::yield_now;

async fn task_a(enable_yield:bool){
    for i in 1..5 {
        println!("[A]:{}", i);
        if enable_yield {
            yield_now().await;
        }
    }
}

async fn task_b(enable_yield:bool){
    for i in 1..5 {
        println!("[B]:{}", i);
        if enable_yield {
            yield_now().await;
            // yield_now().await เป็นการ Force ให้มันบังคับไปทำ task อื่นก่อน
            // yeild_now() เฉยๆเนี่ย เป็น async function เพราะฉะนั้น มันจะไม่ทำอะไรถ้าไม่ call await
        }
    }
}


#[tokio::main]
async fn main(){
    // การเรียก tokio::spawn จะทำให้ execute fn นั้นๆเลย แบบ concerently 
    let t1 = tokio::spawn(task_a(true));
    let t2 = tokio::spawn(task_b(true));

    // แต่ว่า เรามาสั่ง await ตรงนี้ไว้ ไม่ให้ task main() มันจบก่อนที่ task t1, t2 จะทำเสร็จ
    t1.await;
    t2.await;

    // ดังนั้น ถ้าอยากแค่เห็นผลลัพธ์ทั้งหมด เราอาจจะ await แค่ t2 ก็ได้หรือเปล่า?
    // ตอบ: ได้แหละ แต่ถ้า t2 เสร็จก่อน t1 นี่คือ จบเลยน้ะ
}