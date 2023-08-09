fn main() {
    let input = Ljx8060State::OpenedEthernet;

    println!("{:?}", input);

    if input == Ljx8060State::OpenedEthernet {
        println!("true");
    }

    if input != Ljx8060State::PreStarted {
        println!("true");
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Ljx8060State {
    NoConnection,
    OpenedEthernet,
    Initialized,
    PreStarted,
    Measuring,
}
