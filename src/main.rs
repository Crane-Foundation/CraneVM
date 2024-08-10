#![allow(unused)]
pub mod engine;
mod reader;
mod structs;
mod writer;

fn main() {
    //write to test.cb
    writer::write_file(
        "test.cb",
        vec![instruction! {MOV, 0, 3}, instruction! {HALT, 0}],
    );
    //make a file "test.cb" and write the values 0 5 10 0 to it
    let mut engine = engine::engine::Engine::new("test.cb");
    engine.load_file();
    //run the engine
    engine.run();
    //print the memory
}
