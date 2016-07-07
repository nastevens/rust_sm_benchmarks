use std::mem;

#[derive(Clone, Copy, Debug, PartialEq)]
enum StateHandlerEnum {
    State1(State1),
    Done(Done),
}

impl StateHandlerEnum {
    fn dispatch(&mut self) {
        let next = match self {
            &mut StateHandlerEnum::State1(ref mut s) => s.handle(),
            &mut StateHandlerEnum::Done(ref mut s) => s.handle(),
        };
        mem::replace(self, next);
    }

    fn done(&self) -> bool {
        match self {
            &StateHandlerEnum::Done(_) => true,
            _ => false,
        }
    }
}

trait StateHandler {
    fn handle(&self) -> StateHandlerEnum;
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct State1 {
    counter: usize
}

impl StateHandler for State1 {
    fn handle(&self) -> StateHandlerEnum {
        if self.counter > 1_000_000 {
            Done::state(self.counter)
        } else {
            State1::state(self.counter + 1)
        }
    }
}

impl State1 {
    pub fn state(counter: usize) -> StateHandlerEnum {
        StateHandlerEnum::State1(State1 {
            counter: counter,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Done {
    counter: usize
}

impl StateHandler for Done {
    fn handle(&self) -> StateHandlerEnum {
        Done::state(self.counter)
    }
}

impl Done {
    pub fn state(counter: usize) -> StateHandlerEnum {
        StateHandlerEnum::Done(Done {
            counter: counter,
        })
    }
}

pub fn run() -> usize {
    let mut state: StateHandlerEnum = StateHandlerEnum::State1(State1 { counter: 0 });
    while !state.done() {
        state.dispatch();
    }
    match state {
        StateHandlerEnum::Done(Done{counter}) => counter,
        _ => 0,
    }
}
