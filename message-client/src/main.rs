use message_passing_framework::message::{Message, MessageKind};

#[derive(Clone, Copy)]
pub enum CustomMsg {
    Interact(usize),
    MovePlayer(usize),
}

impl std::fmt::Display for CustomMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomMsg::Interact(id) => write!(f, "Interact({})", id),
            CustomMsg::MovePlayer(id) => write!(f, "MovePlayer({})", id),
        }
    }
}

impl MessageKind for CustomMsg {}

#[derive(Clone, Copy, Debug)]
struct F2 {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    a: u32,
    b: bool,
    c: f32,
    d: [F2; 2],
}

fn main() {
    let id = CustomMsg::Interact(23);
    let mut message = Message::new(id);

    let a = 2;
    let b = true;
    let c: f32 = 3.14159;
    println!("a; {}", a);
    println!("b: {}", b);
    println!("c: {}", c);

    let d = [
        F2 { x: 1., y: 2. },
        F2 {
            x: 1.243,
            y: -1234.1,
        },
    ];

    println!("d: {:?}", d);

    println!("{}", message);
    message.push(a);
    println!("{}", message);
    message.push(b);
    println!("{}", message);
    message.push(c);
    println!("{}", message);
    message.push(d);
    println!("{}", message);

    let out_d: [F2; 2] = message.pull();
    let out_c: f32 = message.pull();
    println!("{}", message);
    let out_b: bool = message.pull();
    println!("{}", message);
    let out_a: u32 = message.pull();
    //let (out_d, out_c, out_b, out_a): ([F2; 2], f32, bool, u32) = message.pull(25);
    //let (out_a, out_b, out_c, out_d): (u32, bool, f32, [F2; 2]) = message.pull(25);
    println!("{}", message);
    println!("out_a; {}", out_a);
    println!("out_b: {}", out_b);
    println!("out_c: {}", out_c);
    println!("out_d: {:?}", out_d);

    let complex = Complex { a, b, c, d };
    println!("complex: {:#?}", complex);
    message.push(complex);
    println!("{}", message);
    let out_complex = message.pull::<Complex>();
    println!("{}", message);
    println!("out_complex: {:#?}", out_complex);
}
