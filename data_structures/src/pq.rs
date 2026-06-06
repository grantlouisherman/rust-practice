
pub struct PriorityQueue<T> {
    pub queue: Vec<T>
}

impl<T:std::clone::Clone + std::fmt::Display> PriorityQueue<T> {
    pub fn new() -> Self {
        let q: Vec<T> = vec![]; 
        Self {
            queue: q
        }
    }
    fn swap(&self, q: &mut Vec<T>, parent: usize, child: usize) {
            let tmp = q[parent].clone();
            let c = q[child].clone();
            println!("SWAP: Parent:{tmp}, CHild:{c}");
            q[parent] = c;
            q[child] = tmp;
            println!("{}", q[parent]);

    }
    pub fn insert(&mut self, value: T) 
    where T: std::cmp::PartialOrd {
        let mut q = &mut self.queue;
        q.push(value.clone());
        let mut index = self.queue.len() - 1;

        while index > 0 {
            let p_idx = ((index-1)).div_euclid(2);
            let parent = self.queue[p_idx].clone();
            let child = self.queue[index].clone();
            println!("Insert Value:{value}\nPIDX:{p_idx}\nParent: {parent}\nChild:{child}\n-----------------\n");
            if parent > child {
                self.swap(q, p_idx, index);
                index = p_idx;
                continue;
            }
            break;

        }

    }

}
