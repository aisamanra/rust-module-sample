pub mod foo;

pub const NAME: &'static str = "bar/mod.rs";

pub fn print_helper() {
    println!("in bar/mod.rs {{");
    println!("        foo::NAME\t= {:?}",       foo::NAME);
    println!("  self::foo::NAME\t= {:?}", self::foo::NAME);
    println!("      ::foo::NAME\t= {:?}",     ::foo::NAME);
    println!("}}");
}
