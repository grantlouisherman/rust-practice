
mod pq;


fn main() {
    let mut q = pq::PriorityQueue::<i32>::new();
    q.insert(10);
    q.insert(11);
    q.insert(5);
    for node in q.queue {
        println!("{:?}", node);
    }
}
