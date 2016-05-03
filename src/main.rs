mod foo;
mod bar;

fn main() {
    println!("in main.rs {{");
    println!("    foo::NAME\t\t= {:?}", foo::NAME);
    println!("    bar::NAME\t\t= {:?}", bar::NAME);
    println!("    bar::foo::NAME\t= {:?}", bar::foo::NAME);
    println!("}}");
    bar::print_helper();
}
