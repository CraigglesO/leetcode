use std::rc::Rc;

// NOTE: There is a simpler way to just use two vector stacks
// one to track the order and one to track the minimum.
// But IMHO, this cheats the system so I'm using a ReferenceCounted Node

#[allow(dead_code)]
struct Node {
    val: i32,
    next: Option<Rc<Node>>,
    min: i32,
}
#[allow(dead_code)]
impl Node {
    fn new(val: i32, next: Option<Rc<Node>>, min: i32) -> Self {
        Node { val, next, min }
    }
}
impl Default for Node {
    fn default() -> Self {
        Node {
            val: i32::MAX,
            next: None,
            min: i32::MAX,
        }
    }
}

#[allow(dead_code)]
struct MinStack {
    min: i32,
    head: Rc<Node>,
}
#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        MinStack {
            min: i32::MAX,
            head: Rc::new(Node::default()),
        }
    }

    fn push(&mut self, val: i32) {
        let min = val.min(self.min);
        self.head = Rc::new(Node::new(val, Some(self.head.to_owned()), min));
        self.min = min;
    }

    fn pop(&mut self) {
        // self.head = self.head.next.unwrap_or_default();
        if let Some(next) = self.head.next.to_owned() {
            self.head = next;
        }
        self.min = self.head.min;
    }

    fn top(&self) -> i32 {
        self.head.val
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}
