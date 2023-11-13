mod calcprob;
// mod wakachi;
use calcprob::Model;
use std::io;

fn main() -> io::Result<()> {
    let mut model = Model::new();
    model.make("static/corpus.txt")?;
    eprintln!("The corpus has been loaded");
    let mut quiz = vec![<Vec<String>>::new(); 3];
    let mut generated = Vec::new();
    let input = io::stdin().lines();
    for (i, line) in input.into_iter().enumerate() {
        let line = line?;
        for s in line.split_whitespace() {
            quiz[i].push(s.to_string());
        }
    }
    model.generate(&quiz, &mut generated, 1.6);
    for s in generated {
        eprint!("{}", s);
    }
    eprintln!();
    Ok(())
}