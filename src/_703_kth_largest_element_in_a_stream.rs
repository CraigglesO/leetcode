#[allow(dead_code)]
struct KthLargest {
    k: i32,
    nums: Vec<i32>,
}

// NOTE: A heap/priority queue would be faster, but I wanted to try to find a minimal solution
// and it turns out this is "fast enough"
// the fastest is a 0ms solution, this one is 32ms, so it's not great but it works

#[allow(dead_code)]
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut this = Self { k, nums: vec![] };
        for n in nums {
            this.add(n);
        }

        this
    }

    fn add(&mut self, val: i32) -> i32 {
        let ku = self.k as usize;
        if self.nums.len() == 0 {
            self.nums.push(val);
            return self.nums[self.nums.len() - 1];
        }

        for i in 0..self.nums.len() {
            if val > self.nums[i] {
                self.nums.insert(i, val);
                break;
            } else if i > ku {
                break;
            } else if i == self.nums.len() - 1 {
                self.nums.push(val);
            }
        }

        while self.nums.len() - 1 > ku {
            self.nums.pop();
        }

        self.nums[(ku - 1).min(self.nums.len() - 1)]
    }
}
