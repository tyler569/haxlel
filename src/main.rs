mod interpreter;

fn main() {
    let source = String::from("\"Hello, world\\!!");

    interpreter::interpret(&source);
}
