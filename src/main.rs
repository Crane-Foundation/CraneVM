#![allow(unused)]
mod engine;
mod reader;
mod structs;
mod writer;

fn main() {
    //write to test.cb
    writer::write_file(
        "test.cb",
        vec![
            instruction!{MOV, 4, 5},
            instruction!{MOV, 2, 2},
            instruction! {ADD, 5, 4},
            instruction! {SUB, 5, 2},
            instruction!(HALT),
        ],
    );
    //make a file "test.cb" and write the values 0 5 10 0 to it
    let mut engine = engine::engine::Engine::new("test.cb");
    engine.load_file();
    //run the engine
    engine.run();
    //print the memory
}
