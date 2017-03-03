mod interpreter;

fn main() {
    let source = String::from("\n\"Hello, world\\!!");

    interpreter::interpret(&source);
}
