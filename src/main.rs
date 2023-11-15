mod calcprob;
use calcprob::Model;

fn main() {
    let mut model = Model::new();
    let s = model.main();
    print!("{}", s);
}