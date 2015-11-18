/// FSM states for Inventory handling

#[derive(Debug,Clone,PartialEq)]
pub enum States {
    Closed,
    Opened(Actions),
}

impl States {
    pub fn dropable(&self) -> bool {
        if *self == States::Closed { true }
        else { false }
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum Actions {
    Dropping,
    Selling,
    Buying,
    Using,
    Org, // organizing
    
    Closing,
    Opening,
}

impl Actions {
    pub fn next(self) -> Actions {
        match self {
            Actions::Dropping => Actions::Closing,
            Actions::Selling |
            Actions::Buying |
            Actions::Using |
            Actions::Org =>
                Actions::Opening,
            
            _ => self,
        }
    }
}
