#[derive(Clone)]
struct Handlers {
    something: &'static str
}

pub trait PingEvent {
    fn print_ping(self);

    fn test(self, &str);
}

pub trait PongEvent {
    fn print_pong(self);
}

impl Handlers {
    pub fn new() -> Self {
        Handlers {
            something: "SUPER-AWESOME"
        }
    }
}

impl PingEvent for Handlers {
    fn print_ping(self) {
        println!("PING");
    }

    fn test(self, some_var: &str) {
        println!("{:?}", some_var);
    }
}

impl PongEvent for Handlers {
    fn print_pong(self) {
        println!("PONG");
    }
}

pub struct Registrar;

impl Registrar {
    pub fn exec_ping<P: PingEvent>(event: P) {
        event.test("TEST");
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
    }

    fn test_ping<P: PingEvent>(strait: P) {
        strait.print_ping();
    }

    fn test_pong<P: PongEvent>(strait: P) {
        strait.print_pong();
    }
}