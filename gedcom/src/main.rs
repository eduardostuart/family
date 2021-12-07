use gedcom::Parser;

fn main() {
    let mut tk = Parser::new(include_str!("../../samples/eduardo.ged"));
    println!("{:#?}", tk.parse());
}
