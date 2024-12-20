struct Node<T> {
    node: Option<Box<Node<T>>>,
    value: T,
}
impl<T: std::fmt::Display> Node<T> {
    fn new(val: T) -> Self {
        Node {
            value: val,
            node: None,
        }
    }

    fn get_value(&self) -> &T {
        return &self.value;
    }

    fn set_value(&mut self, new_value: T) {
        self.value = new_value;
    }

    fn set_node(&mut self, new_value: T) {
        self.node = Some(Box::new(Node::new(new_value)));
    }

    fn insert(&mut self, value: T) {
        // El problema estaba en que set_node apunta a self...
        // y self es posible que sea el elemento Raiz
        // let mut current = &self;
        // while let Some(ref node) = current.node {
        //     current = node;

        // }
        // current.set_node(value);
        let mut current = self;
        while let Some(ref mut node) = current.node {
            current = node;
        }
        current.node = Some(Box::new(Node::new(value)));
    }

    fn print_list(&self) {
        let mut string = String::from("[");
        string = format!("[{},", self.get_value());
        let mut current = self;
        while let Some(ref node) = current.node {
            let value = node.get_value();

            string = format!("{} {},", string, value);

            current = node;
        }

        println!("{}]", string);
    }
}

#[warn(dead_code)]
fn main() {
    let mut node = Node::<i32>::new(8);
    let mut value = node.get_value();

    println!("El valor del nodo es {} ", value);

    node.set_value(50);
    value = node.get_value();
    println!("El valor del nodo es {} ", value);

    node.set_node(100);
    node.print_list();

    for n in 1..595 {
        if n % 2 == 0 {
            node.insert(n);
        }
    }
    node.print_list();
    


}
