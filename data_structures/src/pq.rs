
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
    fn swap(&mut self, parent: usize, child: usize) {
            let tmp = self.queue[parent].clone();
            let c = self.queue[child].clone();
            self.queue[parent] = c;
            self.queue[child] = tmp;

    }
    pub fn insert(&mut self, value: T) 
    where T: std::cmp::PartialOrd {
        self.queue.push(value.clone());
        let mut index = self.queue.len() - 1;

        while index > 0 && self.queue[((index-1)).div_euclid(2)] > self.queue[index]{
            let parent = self.queue[index].clone();
            let child = self.queue[index-1].clone();
            //println!("Insert Value:{value}\nPIDX:{p_idx}\nParent: {parent}\nChild:{child}\n-----------------\n");
            if parent > child {
                self.swap(p_idx, index);
                index = p_idx;
                continue;
            }
            break;

        }

    }

}
