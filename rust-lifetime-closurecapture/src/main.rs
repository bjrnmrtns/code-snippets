struct ClosureHolder<'closure_lifetime, UIContext>
{
    closures: Vec<Box<dyn Fn(&mut UIContext) + 'closure_lifetime>>,
}

impl<'closure_lifetime, UIContext> ClosureHolder<'closure_lifetime, UIContext> {
    pub fn call_all(&mut self, context: &mut UIContext) {
        let _: () = self.closures.iter_mut().map(|c| { c(context); }).collect();
    }

    pub fn add_closure<CLOSURE: Fn(&mut UIContext) + 'closure_lifetime>(&mut self, closure: CLOSURE) {
        self.closures.push(Box::new(closure));
    }
}

struct MyUIContext {
    pub x: i32,
}

fn main() {
    let mut x = MyUIContext { x: 0 };
    let mut holder: ClosureHolder<MyUIContext> = ClosureHolder { closures: Vec::new(), };
    holder.add_closure(|x| { x.x = x.x + 1; });
    holder.call_all(&mut x);
    println!("{}", x.x);
}

