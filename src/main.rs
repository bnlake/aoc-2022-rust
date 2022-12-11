mod utils;

fn main() {
    let path = String::from("input.txt");
    let contents = utils::read_file(path);
    println!("{contents}");
}
