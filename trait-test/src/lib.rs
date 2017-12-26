#[derive(Clone)]
struct Handlers {
    something: String
}

pub trait PingEvent: Clone {
    fn new() -> Self;

    fn print(self);

    fn print_ex(self);

    fn set_something(self, String) -> Self;
}

pub trait PongEvent {
    fn print(self);
}

impl Handlers {
    pub fn new() -> Self {
        Handlers {
            something: String::from("DEFAULT_VALUE")
        }
    }

    pub fn print_ex(self) {
        println!("{}", self.something);
    }

    pub fn set_something(mut self, value: String) -> Self {
        self.something = value;
        self
    }
}

impl PingEvent for Handlers {
    fn new() -> Self {
        Handlers {
            something: String::from("PING_EVENT")
        }
    }

    fn print(self) {
        println!("PING");
    }

    fn print_ex(self) {
        println!("{}", self.something);
    }

    fn set_something(mut self, something: String) -> Self {
        self.something = something;
        self
    }
}

impl PongEvent for Handlers {
    fn print(self) {
        println!("PONG");
    }
}

pub struct Registrar;

impl Registrar {
    pub fn exec_ping<P: PingEvent>(handler: P) {
        let handler = handler.set_something(String::from("Param Handler"));
        
        let one = handler.clone();
        let one = one.set_something(String::from("Handler one"));

        let two = handler.clone();
        let two = two.set_something(String::from("Handler two"));
        
        one.print_ex();
        two.print_ex();
        handler.print_ex();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that() {
        let handlers = Handlers::new();
        test_ping(handlers.clone());
        test_pong(handlers.clone());

        let register = Registrar::exec_ping(handlers.clone());
        handlers.clone().print_ex();
        let handlers = handlers.set_something(String::from("Test"));
        handlers.print_ex();
    }

    fn test_ping<P: PingEvent>(strait: P) {
        strait.print();
    }

    fn test_pong<P: PongEvent>(strait: P) {
        strait.print();
    }
}