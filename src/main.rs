mod helloworld;
mod variables;
mod collections;
mod function;
mod control_flow;
mod ownership;
mod structure;

fn main() {
    helloworld::say_hello_world();
    variables::run();
    collections::run();
    function::run();
    control_flow::run();
    ownership::run();
    structure::run();
}
