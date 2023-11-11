use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

use rand::prelude::*;

pub struct Model {
    pub counts: HashMap<String, u32>,
    pub n: HashMap<u32, u32>,
    pub total: u32,
    pub types: u32,
}

impl Model {
    pub fn new() -> Model {
        Model {
            counts: HashMap::new(),
            n: HashMap::new(),
            total: 0,
            types: 0,
        }
    }

    pub fn make(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let _ = *self.n.entry(0).or_insert(250000);

        for line in reader.lines() {
            let line = line?;
            for word in line.split_whitespace() {
                let count = self.counts.entry(word.to_string()).or_insert(0);
                if count == &mut 0 {
                    self.types += 1;
                }
                *self.n.get_mut(count).unwrap() -= 1;
                *count += 1;
                *self.n.entry(*count).or_insert(0) += 1;
            }
        }
        self.total = self.counts.values().sum();
        Ok(())
    }

    // pub fn number_of_type(&self) -> u32 {
    //     self.types
    // }

    // pub fn n_one(&self) -> f64 {
    //     self.n[&1] as f64 / self.types as f64
    // }

    // pub fn most_frequent_word(map: &'a HashMap<String, u32>) -> Option<(&'a String, &'a u32)> {
    //     let mut sorted_vec: Vec<(&'a String, &'a u32)> = map.iter().collect();
    //     sorted_vec.sort_by(|a, b| b.1.cmp(a.1));
    //     sorted_vec.get(0).cloned()
    // }

    // pub fn ave_occurrence_count(&self) -> f64 {
    //     self.total as f64 / self.types as f64
    // }

    // pub fn prob_ml(&self, word: &str) -> f64 {
    //     let count = *self.counts.get(word).unwrap_or(&0);
    //     count as f64 / self.total as f64
    // }

    pub fn prob_ft(&self, word: &str) -> f64 {
        let count = *self.counts.get(word).unwrap_or(&0);
        let r_star =
            if let (Some(&n_r), Some(&n_r_plus)) = (self.n.get(&count), self.n.get(&(count + 1))) {
                (count as f64 + 1.0_f64) * (n_r_plus as f64 / n_r as f64)
            } else {
                count as f64
            };
        if r_star <= 0.0 {
            count as f64 / self.total as f64
        } else {
            r_star / self.total as f64
        }
    }

    pub fn calc_entropy<F>(&self, sentence: &[String], prob: F) -> f64
    where
        F: Fn(&Self, &str) -> f64,
    {
        let mut entropy: f64 = 0.0;
        let mut num: u32 = 0;
        for word in sentence {
            entropy += (1.0_f64 / prob(self, word)).log2();
            num += 1;
        }
        entropy / num as f64
    }

    pub fn calc_perplexity<F>(&self, sentence: &[String], prob: F) -> f64
    where
        F: Fn(&Self, &str) -> f64,
    {
        let entropy: f64 = self.calc_entropy(sentence, prob);
        entropy.exp2()
    }

    pub fn generate(&self, quiz: &[Vec<String>], res: &mut Vec<String>, p: f64){
        // let mut res = Vec::new();
        let mut rng = rand_pcg::Pcg64Mcg::new(42);
        let mut index = vec![0_usize; quiz.len()];
        let mut prob = vec![0.; quiz.len()];
        let mut last;
        let mut maxidx = 0;
        let mut maxval ;
        {
            let first = rng.gen_range(0..quiz.len());
            res.push(quiz[first][0].clone());
            index[first] += 1;
            last = first;
        }
        'main: 
        loop {
            maxval = 1e99;
            for i in 0..quiz.len() { 
                res.push(quiz[i][index[i]].clone());
                prob[i] = self.calc_perplexity(res, Self::prob_ft);
                if i == last {
                    prob[i] *= p;
                }
                if prob[i] < maxval {
                    maxval = prob[i];
                    maxidx = i;
                }
                res.pop();
            }
            // eprintln!("{:?}", prob);
            let branch = rng.gen_range(0..10);
           if branch == 9 {
                for (i, idx) in index.iter_mut().enumerate() {
                    *idx += 1;
                    if *idx == quiz[i].len() {
                        break 'main;
                    }
                }
            } else {
                last = maxidx;
                res.push(quiz[last][index[last]].clone());
                index[last] += 1;
            }
            if index[last] == quiz[last].len() {
                break;
            }
        }
    }
}