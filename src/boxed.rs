trait StateHandler {
    fn handle(&self) -> Box<StateHandler>;
    fn done(&self) -> bool { false }
    fn value(&self) -> usize { 0 }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct State1(usize);

impl StateHandler for State1 {
    fn handle(&self) -> Box<StateHandler> {
        if self.0 > 1_000_000 {
            Box::new(Done(self.0))
        } else {
            Box::new(State1(self.0 + 1))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Done(usize);

impl StateHandler for Done {
    fn handle(&self) -> Box<StateHandler> {
        Box::new(*self)
    }

    fn done(&self) -> bool { true }

    fn value(&self) -> usize { self.0 }
}

pub fn run() -> usize {
    let mut state: Box<StateHandler> = Box::new(State1(0));
    while !state.done() {
        state = state.handle();
    }
    state.value()
}
