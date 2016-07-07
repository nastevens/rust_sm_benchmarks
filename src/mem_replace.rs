use std::mem;

trait StateHandler {
    fn handle(&mut self) -> StateHandlerEnum;
    fn done(&self) -> bool { false }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum StateHandlerEnum {
    State1(State1),
    Done(Done),
    // 
    NoState,
}

impl StateHandler for StateHandlerEnum {
    fn handle(&mut self) -> StateHandlerEnum {
        let next = match self {
            &mut StateHandlerEnum::State1(ref mut s) => s.handle(),
            &mut StateHandlerEnum::Done(ref mut s) => s.handle(),
            &mut StateHandlerEnum::NoState => panic!(),
        };
        mem::replace(self, next);
        StateHandlerEnum::NoState
    }

    fn done(&self) -> bool {
        match self {
            &StateHandlerEnum::State1(ref s) => s.done(),
            &StateHandlerEnum::Done(ref s) => s.done(),
            &StateHandlerEnum::NoState => panic!(),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct State1 {
    counter: usize
}

impl StateHandler for State1 {
    fn handle(&mut self) -> StateHandlerEnum {
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
    fn handle(&mut self) -> StateHandlerEnum {
        Done::state(self.counter)
    }

    fn done(&self) -> bool { true }
}

impl Done {
    pub fn state(counter: usize) -> StateHandlerEnum {
        StateHandlerEnum::Done(Done {
            counter: counter,
        })
    }
}

pub fn run() -> usize {
    let mut state: StateHandlerEnum = State1::state(0);
    while !state.done() {
        state.handle();
    }
    match state {
        StateHandlerEnum::Done(Done{counter}) => counter,
        _ => 0,
    }
}
