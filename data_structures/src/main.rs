
mod pq;


fn main() {
    let mut q = pq::PriorityQueue::<i32>::new();
    q.insert(16);
    q.insert(31);
    q.insert(100);
    q.insert(41);
    q.insert(13);
    q.insert(115);
    for idx in 0..q.queue.len() {
        let node = q.queue[idx];
        println!("{idx}-{:?}", node);
    }
}
