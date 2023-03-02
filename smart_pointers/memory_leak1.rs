use std::rc::Rc;

struct Node {
    value: i32,
    parent: Option<Rc<Node>>,
    children: Vec<Rc<Node>>,
}

impl Node {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(Self {
            value,
            parent: None,
            children: Vec::new(),
        })
    }

    fn add_child(&mut self, child: Rc<Self>) {
        self.children.push(child);
        child.parent = Some(Rc::clone(&self));
    }
}

fn main() {
    let root = Node::new(0);
    let child1 = Node::new(1);
    let child2 = Node::new(2);
    child1.add_child(Rc::clone(&child2));
    child2.add_child(Rc::clone(&child1));
    root.add_child(child1);
    root.add_child(child2);
    println!("{}", root.value);
}
