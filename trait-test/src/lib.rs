#[derive(Clone)]
struct Handlers {
    something: &'static str
}

trait PingEvent {
    fn print_ping(self);
}

trait PongEvent {
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
}

impl PongEvent for Handlers {
    fn print_pong(self) {
        println!("PONG");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that() {
        let handlers = Handlers::new();
        //handlers.print();
        qwe(handlers.clone());
        asd(handlers.clone());
    }

    fn qwe<P: PingEvent>(strait: P) {
        strait.print_ping();
    }

    fn asd<P: PongEvent>(strait: P) {
        strait.print_pong();
    }
}