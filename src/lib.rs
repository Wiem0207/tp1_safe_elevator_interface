#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Idle,
    MovingUp,
    MovingDown,
    DoorsOpen,
}
#[derive(Debug, PartialEq, Eq)]
pub enum ElevatorError {
    InvalidFloor(i32),
    DoorsAlreadyOpen,
    DoorsAlreadyClosed,
    CannotOpenWhileMoving,
    CannotMoveDoorsOpen,
    EmptyQueue,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Elevator {
    floor: i32,
    state: State,
    queue: Vec<i32>,
}
pub struct ElevatorState {
    pub floor: i32,
    pub state: State,
    pub queue: Vec<i32>,
}
impl Elevator {
    const MIN_FLOOR: i32 = 0;
    const MAX_FLOOR: i32 = 5;
    pub fn new(starting_floor: i32) -> Result<Self, ElevatorError> {
        if (starting_floor < Self::MIN_FLOOR) || (starting_floor > Self::MAX_FLOOR) {
            return Err(ElevatorError::InvalidFloor(starting_floor));
        }
        Ok(Self {
            floor: starting_floor,
            state: State::Idle,
            queue: Vec::new(),
        })
    }
    pub fn call(&mut self, called_floor: i32) -> Result<(), ElevatorError> {
        if (called_floor < Self::MIN_FLOOR) || (called_floor > Self::MAX_FLOOR) {
            return Err(ElevatorError::InvalidFloor(called_floor));
        }
        if self.floor == called_floor {
            return Ok(());
        }
        if self.queue.contains(&called_floor) {
            return Ok(());
        }
        self.queue.push(called_floor);
        if self.state == State::Idle {
            if self.floor > called_floor {
                self.state = State::MovingDown;
            } else {
                self.state = State::MovingUp;
            }
        }
        Ok(())
    }
    pub fn state(&self) -> State {
        return self.state;
    }
    pub fn step(&mut self) -> Result<(), ElevatorError> {
        if self.state == State::DoorsOpen {
            return Err(ElevatorError::CannotMoveDoorsOpen);
        }
        if self.queue.is_empty() {
            self.state = State::Idle;
            return Err(ElevatorError::EmptyQueue);
        }
        let destination = self.queue[0];
        if self.floor < destination {
            self.floor += 1;
            self.state = State::MovingUp;
        }
        if self.floor > destination {
            self.floor -= 1;
            self.state = State::MovingDown;
        }
        if self.floor == destination {
            self.state = State::DoorsOpen;
            self.queue.remove(0);
        }
        Ok(())
    }
    pub fn floor(&self) -> i32 {
        self.floor
    }
    pub fn queue(self) -> Vec<i32> {
        self.queue
    }
    pub fn open_doors(&mut self) -> Result<(), ElevatorError> {
        if self.state == State::DoorsOpen {
            return Err(ElevatorError::DoorsAlreadyOpen);
        }
        if self.state != State::Idle {
            return Err(ElevatorError::CannotOpenWhileMoving);
        }
        self.state = State::DoorsOpen;
        Ok(())
    }
    pub fn close_doors(&mut self) -> Result<(), ElevatorError> {
        if self.state != State::DoorsOpen {
            return Err(ElevatorError::DoorsAlreadyClosed);
        }
        if self.queue.is_empty() {
            self.state = State::Idle;
        }
        let destination = self.queue[0];
        if self.floor < destination {
            self.state = State::MovingUp;
        }
        if self.floor > destination {
            self.state = State::MovingDown;
        }
        Ok(())
    }
    pub fn status(&self)->ElevatorState{
        ElevatorState { floor: self.floor, state: self.state, queue: self.queue.clone()}
    }
}
