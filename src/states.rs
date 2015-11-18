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
    pub fn next (self, act: Actions) -> States {
        match self {
            States::Closed => {
                States::Opened(act)
            },
            States::Opened(actions) => {
                let next = actions.next();
                
                match next {
                    Actions::Closing => {
                        match act {
                            Actions::Closing => States::Closed,
                            _ => States::Opened(act),
                        }
                    },
                    _ => States::Opened(next.next()),
                }
            }
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum Actions {
    Dropping,
    Bartering,
    Using,
    Org, // organizing
    
    Closing,
    Opening(Box<Actions>),
}

impl Actions {
    pub fn next(self) -> Actions {
        match self {
            Actions::Dropping => Actions::Closing,
            
            Actions::Bartering |
            Actions::Using |
            Actions::Org => Actions::Closing,

            Actions::Opening(next) => {
                *next
            },
            _ => self,
        }
    }
}
