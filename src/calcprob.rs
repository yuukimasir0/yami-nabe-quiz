use std::{
    cmp::min,
    collections::{HashMap, BTreeSet},
    fs::File,
    io::{self, BufRead, BufReader},
    time,
};

use rand::prelude::*;
use rand_pcg::Mcg128Xsl64;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum PartOfSpeech {
    Start,
    Verb,             // 動詞
    Adjective,        // 形容詞
    AdjectivalNoun,   // 形容動詞
    Noun,             // 名詞
    Adverb,           // 副詞
    PreNounAdjective, // 連体詞
    Conjunction,      // 接続詞
    Interjection,     // 感動詞
    AuxiliaryVerb,    // 助動詞
    Particle,         // 助詞
    Pronoun,          // 代名詞
    Suffix,           // 接尾辞
    Prefix,           // 接頭辞
    Determiner,       // 形状詞
    Symbol,           // 記号
    AuxiliarySymbol,  // 補助記号
    End,              // 終端記号
    Unknown,
}

pub struct Model {
    counts: HashMap<String, u32>,
    n: HashMap<u32, u32>,
    pos: HashMap<String, PartOfSpeech>,
    transition_rule: HashMap<PartOfSpeech, HashMap<PartOfSpeech, u32>>,
    total: u32,
}

impl Model {
    pub fn new() -> Model {
        Model {
            counts: HashMap::new(),
            n: HashMap::new(),
            pos: HashMap::new(),
            transition_rule: HashMap::new(),
            total: 0,
        }
    }

    #[inline(always)]
    fn set_pos(&mut self, word: &str, prev: &mut PartOfSpeech, nxt: &PartOfSpeech) {
        self.pos.insert(word.to_string(), *nxt);
        self.transition_rule
            .entry(*prev)
            .or_insert(HashMap::new())
            .entry(*nxt)
            .and_modify(|c| *c += 1)
            .or_insert(1);
        *prev = *nxt;
    }

    pub fn make(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let _ = *self.n.entry(0).or_insert(250000);

        for line in reader.lines() {
            let line = line?;
            let words: Vec<&str> = line.split_whitespace().collect();
            let mut prev = PartOfSpeech::Start;
            for chunk in words.chunks(2) {
                if let [word, pos] = chunk {
                    let count = self.counts.entry(word.to_string()).or_insert(0);
                    *self.n.get_mut(count).unwrap() -= 1;
                    *count += 1;
                    *self.n.entry(*count).or_insert(0) += 1;
                    if !self.pos.contains_key(*word) {
                        match *pos {
                            "動詞" => self.set_pos(word, &mut prev, &PartOfSpeech::Verb),
                            "形容詞" => self.set_pos(word, &mut prev, &PartOfSpeech::Adjective),
                            "形容動詞" => self.set_pos(word, &mut prev, &PartOfSpeech::AdjectivalNoun),
                            "名詞" => self.set_pos(word, &mut prev, &PartOfSpeech::Noun),
                            "副詞" => self.set_pos(word, &mut prev, &PartOfSpeech::Adverb),
                            "連体詞" => self.set_pos(word, &mut prev, &PartOfSpeech::PreNounAdjective),
                            "接続詞" => self.set_pos(word, &mut prev, &PartOfSpeech::Conjunction),
                            "感動詞" => self.set_pos(word, &mut prev, &PartOfSpeech::Interjection),
                            "助動詞" => self.set_pos(word, &mut prev, &PartOfSpeech::AuxiliaryVerb),
                            "助詞" => self.set_pos(word, &mut prev, &PartOfSpeech::Particle),
                            "補助記号" => self.set_pos(word, &mut prev, &PartOfSpeech::AuxiliarySymbol),
                            "終端記号" => self.set_pos(word, &mut prev, &PartOfSpeech::End),
                            "代名詞" => self.set_pos(word, &mut prev, &PartOfSpeech::Pronoun),
                            "接尾辞" => self.set_pos(word, &mut prev, &PartOfSpeech::Suffix),
                            "接頭辞" => self.set_pos(word, &mut prev, &PartOfSpeech::Prefix),
                            "形状詞" => self.set_pos(word, &mut prev, &PartOfSpeech::Determiner),
                            "記号" => self.set_pos(word, &mut prev, &PartOfSpeech::Symbol),
                            _ => ()
                        };
                    }
                }
            }
        }
        self.total = self.counts.values().sum();
        Ok(())
    }

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

    pub fn generate(&self, quiz: &[Vec<String>], res: &mut Vec<String>, p: f64) {
        let mut rng = rand_pcg::Pcg64Mcg::new(time::Instant::now().elapsed().as_nanos());
        let mut index = vec![0_usize; quiz.len()];
        let mut idx = Vec::new();
        let mut prev;
        {
            let first = rng.gen_range(0..quiz.len());
            res.push(quiz[first][0].clone());
            prev = self
                .pos
                .get(&quiz[first][0])
                .unwrap_or(&PartOfSpeech::Unknown);
            index[first] += 1;
        }
        loop {
            for (i, quiz) in quiz.iter().enumerate() {
                for j in 0..3 {
                    if index[i] + j >= quiz.len() {
                        break;
                    }
                    match self.transition_rule.get(prev) {
                        Some(q) => {
                            if let Some(q) = q.get(
                                self.pos
                                    .get(&quiz[index[i] + j])
                                    .unwrap_or(&PartOfSpeech::Unknown),
                            ) {
                                idx.push((*q, i, j));
                            }
                        }
                        None => {
                            // idx.push((1, i, j));
                        }
                    }
                }
                idx.push((0, i, 0));
            } //分岐候補の生成
            let id = rng.gen_range(0..idx.len());
            let nxt = idx[id].1;
            if idx[id].0 == 0 && idx[id].2 == 0 {
                index[nxt] += 1;
            } else {
                res.push(quiz[nxt][index[nxt] + idx[id].2].clone());
                prev = self
                    .pos
                    .get(&quiz[nxt][index[nxt] + idx[id].2])
                    .unwrap_or(&PartOfSpeech::Unknown);
                index[nxt] += idx[id].2 + 1;
            }
            if index[nxt] >= quiz[nxt].len() {
                break;
            }
        }
    }

    pub fn test_gen(&self, quiz: &[Vec<String>], res: &mut Vec<String>, p: f64) {
        let mut rng = rand_pcg::Pcg64Mcg::new(time::Instant::now().elapsed().as_nanos());
        let now = time::Instant::now();
        let mut idx = BTreeSet::new();
        let three_secs = time::Duration::from_secs(1);
        while now.elapsed() < three_secs {
            //3秒実行
            idx.insert(self.internal_gen(quiz, &mut rng));
        }
        for (i, idx) in idx.iter().enumerate() {
            println!(r#"{}: len = [{}] str = "{}""#,i, idx.len(), self.make_str(idx, quiz));
        }
    }

    fn internal_gen(&self, quiz: &[Vec<String>], rng: &mut Mcg128Xsl64) -> Vec<(usize, usize)> {
        'gen: loop {
            let mut index = vec![0_usize; quiz.len()];
            let mut res_idx = Vec::new();
            {
                let first = rng.gen_range(0..quiz.len());
                res_idx.push((first, 0));
                index[first] += 1;
            }
            loop {
                let nxt = rng.gen_range(0..index.len());
                if quiz[nxt].len() - index[nxt] < 2 {
                    res_idx.push((nxt, index[nxt]));
                    if res_idx.len() > 50 {
                      return res_idx;
                    } else {
                        continue 'gen;
                    }
                }
                let idx = rng.gen_range(1..min(3, quiz[nxt].len() - index[nxt]));
                index[nxt] += idx;
                res_idx.push((nxt, index[nxt]));
            }
        }
    }

    fn make_str(&self, idx: &[(usize, usize)], quiz: &[Vec<String>]) -> String {
        let mut s = String::new();
        for &(i, j) in idx.iter() {
            s.push_str(&quiz[i][j]);
        }
        s
    }

    pub fn main(&mut self) -> String {
        self.make("static/corpus.txt").unwrap();
        // eprintln!("The corpus has been loaded");
        let mut quiz = Vec::new();
        let mut generated = Vec::new();
        let input = io::stdin().lines();
        for (i, line) in input.into_iter().enumerate() {
            quiz.push(Vec::new());
            let line = line.unwrap();
            for s in line.split_whitespace() {
                quiz[i].push(s.to_string());
            }
            quiz[i].pop();
        }
        self.test_gen(&quiz, &mut generated, 1.6);
        generated.join("")
    }
}