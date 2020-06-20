struct ClosureHolder
{
    closure: Box<dyn FnMut()>,
}

impl ClosureHolder {
    pub fn call(&mut self) {
        self.closure.as_mut()()
    }
}

fn main() {
    let mut x = 3;
    let mut holder = ClosureHolder { closure: Box::new(|| { x = x + 1; }) };
    println!("{}", x);
    holder.call();
}

