fn retry() -> impl Sized {}

struct Core<T>(T);

impl Core<XXX> {
    pub fn spawn(self) {}
}

fn main() {
    let core = Core(1);
    core.spawn(retry());
}

