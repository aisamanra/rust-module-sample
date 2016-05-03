mod foo;
mod bar;

fn main() {
    println!("in main.rs {{");
    println!("        foo::NAME\t= {:?}",       foo::NAME);
    println!("  self::foo::NAME\t= {:?}", self::foo::NAME);
    println!("      ::foo::NAME\t= {:?}",     ::foo::NAME);
    println!("}}");
    bar::print_helper();
}
