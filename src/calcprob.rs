use std::{
    cmp::min,
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    time,
};

use rand::prelude::*;
use rand_pcg::Mcg128Xsl64;

pub struct Model {
    counts: HashMap<(String, String), u32>,
    n: HashMap<u32, u32>,
    total: u32,
}

impl Model {
    pub fn new() -> Model {
        Model {
            counts: HashMap::new(),
            n: HashMap::new(),
            total: 0,
        }
    }

    pub fn make(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let _ = *self.n.entry(0).or_insert(250000);

        for line in reader.lines() {
            let line = line?;
            let words: Vec<&str> = line.split_whitespace().collect();
            for words in words.windows(2) {
                if let [word0,word1] = words {
                    let count = self.counts.entry((word0.to_string(), word1.to_string())).or_insert(0);
                    *self.n.get_mut(count).unwrap() -= 1;
                    *count += 1;
                    *self.n.entry(*count).or_insert(0) += 1;
                }
            }
        }
        self.total = self.counts.values().sum();
        Ok(())
    }

    pub fn prob_ft(&self, word: &[String]) -> f64 {
        let count = *self.counts.get(&(word[0].to_string(), word[1].to_string())).unwrap_or(&0);
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
        F: Fn(&Self, &[String]) -> f64,
    {
        let mut entropy: f64 = 0.0;
        let mut num: u32 = 0;
        for word in sentence.windows(2) {
            entropy -= prob(self, word).log2();
            num += 1;
        }
        entropy / num as f64
    }

    pub fn calc_perplexity<F>(&self, sentence: &[String], prob: F) -> f64
    where
        F: Fn(&Self, &[String]) -> f64,
    {
        let entropy: f64 = self.calc_entropy(sentence, prob);
        entropy.exp2()
    }

    pub fn generate(&self, quiz: &[Vec<String>], res: &mut Vec<String>) {
        let mut rng = rand_pcg::Pcg64Mcg::new(time::Instant::now().elapsed().as_nanos());
        let now = time::Instant::now();
        let mut fvst = Vec::new();
        let limit_times = time::Duration::from_secs(3);
        let under_qoi = quiz.iter().map(|x| x.len()).sum::<usize>() / quiz.len() / 3;
        while now.elapsed() < limit_times {
            fvst.push(self.internal_gen(quiz, &mut rng, under_qoi));
        }
        fvst.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        for s in &fvst[0].1 {    
            res.push(s.clone());
        }
    }

    #[inline]
    fn internal_gen(&self, quiz: &[Vec<String>], rng: &mut Mcg128Xsl64, under_qoi: usize) -> (f64, Vec<String>) {
        'gen: loop {
            let mut index = vec![0_usize; quiz.len()];
            let mut idx = Vec::with_capacity(under_qoi * (quiz.len() + 2));
            let mut num = vec![0_usize; quiz.len()];
            {
                let first = rng.gen_range(0..quiz.len());
                idx.push((first, 0));
                index[first] += 1;
                num[first] += 1;
            }
            loop {
                let nxt = rng.gen_range(0..index.len());
                if quiz[nxt].len() - index[nxt] < 3 {
                    idx.push((nxt, index[nxt]));
                    num[nxt] += 1;
                    for x in num {
                        if x < under_qoi {
                            continue 'gen;
                        }
                    }
                    if idx.len() > under_qoi * (quiz.len() + 2) {
                        let s = self.make_str(&idx, quiz);
                        let p = self.calc_perplexity(&s, Self::prob_ft);
                        return (p, s);
                    } else {
                        continue 'gen;
                    }
                }
                let idx_add = rng.gen_range(1..min(3, quiz[nxt].len() - index[nxt]));
                index[nxt] += idx_add;
                num[nxt] += 1;
                idx.push((nxt, index[nxt]));
            }
        }
    }

    fn make_str(&self, idx: &[(usize, usize)], quiz: &[Vec<String>]) -> Vec<String> {
        let mut s = Vec::new();
        for &(i, j) in idx.iter() {
            let q = quiz[i][j].clone();
            if q == "?" || q == "でしょ" || q == "う" || q == "でしょう" {
                continue;
            }
            s.push(q);
        }
        s.push("でしょう?".to_string());
        s
    }

    pub fn main(&self, quiz: &[Vec<String>]) -> String {
        if quiz.len() == 1 {
            return quiz[0].join("")
        }
        let mut generated = Vec::new();
        self.generate(quiz, &mut generated);
        generated.join("")
    }
}