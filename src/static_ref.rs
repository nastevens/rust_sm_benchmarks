trait StateHandler {
    fn handle(&self, counter: &mut usize) -> &'static StateHandler;
    fn done(&self) -> bool { false }
}

const STATE1: &'static StateHandler = &State1;
const DONE: &'static StateHandler = &Done;

#[derive(Clone, Copy, Debug, PartialEq)]
struct State1;

impl StateHandler for State1 {
    fn handle(&self, counter: &mut usize) -> &'static StateHandler {
        if *counter > 1_000_000 {
            DONE
        } else {
            *counter += 1;
            STATE1
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Done;

impl StateHandler for Done {
    fn handle(&self, _: &mut usize) -> &'static StateHandler {
        DONE
    }

    fn done(&self) -> bool { true }
}

pub fn run() -> usize {
    let mut state: &'static StateHandler = STATE1;
    let mut counter: usize = 0;
    while !state.done() {
        state = state.handle(&mut counter);
    }

    counter
}
