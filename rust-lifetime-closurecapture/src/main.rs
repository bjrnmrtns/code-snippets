struct ClosureHolder
{
    closures: Vec<Box<dyn FnMut()>>,
}

impl ClosureHolder {
    pub fn call_all(&mut self) {
        let x: () = self.closures.iter_mut().map(|c| { c.as_mut()(); }).collect();
    }
    pub fn add_closure(&mut self, closure: Box<FnMut()>) {
        self.closures.push(closure);
    }
}

fn main() {
    let mut x = 3;
    let mut holder = ClosureHolder { closures: Vec::new(), };
    holder.add_closure(Box::new(|| { println!("hello"); }));
    holder.add_closure(Box::new(|| { x = x + 1; }));
    holder.call_all();
    println!("{}", x);
}

