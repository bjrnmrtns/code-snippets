struct ClosureHolder<'closure_lifetime>
{
    closures: Vec<Box<dyn FnMut() + 'closure_lifetime>>,
}

impl ClosureHolder<'_> {
    pub fn call_all(&mut self) {
        let x: () = self.closures.iter_mut().map(|c| { c.as_mut()(); }).collect();
    }
    pub fn add_closure<'closure_lifetime>(&mut self, closure: Box<FnMut() + 'closure_lifetime>) {
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

