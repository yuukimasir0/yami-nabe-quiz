mod Calcprob;
use Calcprob::Model;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let mut model = Model::new();
    model.make("static/corpus.txt")?;
    eprintln!("The corpus has been loaded");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    println!("Number of types : {}", model.number_of_type());
    println!("Number of Words that appear only once : {}", model.n_one());
    let (key, count) = Model::most_frequent_word(&mut model.counts).expect("msg");
    println!("Most frequent word : [{}, {}]", key, count);
    println!(
        "Average occurrence count = {}\n",
        model.ave_occurrence_count()
    );
    println!(
        "-------------------------------------------------------------------------------------\n"
    );

    for sentence in reader.lines() {
        let sentence = sentence.unwrap();
        println!("{}\n", sentence);
        for word in sentence.split_whitespace() {
            println!(
                "{:<20} ML = {:<25} GT = {:<25}",
                word,
                model.prob_ml(word),
                model.prob_ft(word)
            );
        }

        println!(
            "\n ml test set perplexity = {}\n ft test set perplexity = {}\n",
            model.calc_perplexity(sentence.as_str(), Model::prob_ml),
            model.calc_perplexity(sentence.as_str(), Model::prob_ft)
        );

        println!("-------------------------------------------------------------------------------------\n");
    }
    Ok(())
}