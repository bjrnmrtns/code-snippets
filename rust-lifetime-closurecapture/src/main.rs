struct ClosureHolder<'closure_lifetime>
{
    closures: Vec<Box<dyn FnMut() + 'closure_lifetime>>,
}

impl<'closure_lifetime> ClosureHolder<'closure_lifetime> {
    pub fn call_all(&mut self) {
        let _: () = self.closures.iter_mut().map(|c| { c.as_mut()(); }).collect();
    }

    pub fn add_closure<CLOSURE: FnMut() + 'closure_lifetime>(&mut self, closure: CLOSURE) {
        self.closures.push(Box::new(closure));
    }
}

fn main() {
    let mut x = 3;
    let mut holder = ClosureHolder { closures: Vec::new(), };
    holder.add_closure(|| { x = x + 1; });
    holder.call_all();
    //println!("{}", x);
}

