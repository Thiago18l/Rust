// breaking literal strings

fn main() {
    println!("{}", "These
        are
        three lines");
    just_one();
    println!("{}", "These\n\
    are\n\
    Three lines");  
}

fn just_one() {
    println!("{}", "This \
    is \
    just one line");
}