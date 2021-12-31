use crate::{
    interpreter::Interpreter,
    parser::{Parser, StringParser},
};

mod error;
mod interpreter;
mod parser;
mod symbol;

fn main() {
    let hello = StringParser::new(
        "
        +++++++++++
>+>>>>++++++++++++++++++++++++++++++++++++++++++++
>++++++++++++++++++++++++++++++++<<<<<<[>[>>>>>>+>
+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<[>++++++++++[-
<-[>>+>+<<<-]>>>[<<<+>>>-]+<[>[-]<[-]]>[<<[>>>+<<<
-]>>[-]]<<]>>>[>>+>+<<<-]>>>[<<<+>>>-]+<[>[-]<[-]]
>[<<+>>[-]]<<<<<<<]>>>>>[+++++++++++++++++++++++++
+++++++++++++++++++++++.[-]]++++++++++<[->-<]>++++
++++++++++++++++++++++++++++++++++++++++++++.[-]<<
<<<<<<<<<<[>>>+>+<<<<-]>>>>[<<<<+>>>>-]<-[>>.>.<<<
[-]]<<[>>+>+<<<-]>>>[<<<+>>>-]<<[<+>-]>[<+>-]<<<-]
        "
        .to_string(),
    );

    match hello.parse() {
        Ok(instructions) => {
            let mut interpreter = Interpreter::new();
            if let Err(err) = interpreter.exec(instructions) {
                println!("{}", err)
            }
        }
        Err(err) => println!("{}", err),
    }
}
