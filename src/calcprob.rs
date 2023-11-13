use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    time,
};

use rand::prelude::*;
use rand_pcg::Mcg128Xsl64;

pub struct Model {
    pub counts: HashMap<String, u32>,
    pub n: HashMap<u32, u32>,
    pub joshi: HashMap<String, bool>,
    pub total: u32,
}

impl Model {
    pub fn new() -> Model {
        let mut joshi = HashMap::new();
        let input_str = "の から ぞ ほど ばかり だけ が さ よ ね な を や ろ い ら し か かい かな が な ね とも かも もが の ぞ ぜ や よ さ す のに やら ものか もんか もん わ かしら かし って ってば ば と ても でも けれど けれども が のに ので から し て で なり ながら たり つつ ところで まま ものの や とも ども に を は も こそ でも しか ほか だって ばかり まで だけ さえ ほど くらい ぐらい など なんか なんて なり やら か ぞ し ばし がてら なぞ なんぞ ずつ のみ きり や だに すら の に と や し やら か なり だの とか も が の を に へ と から より で";
        for s in input_str.split_whitespace() {
            joshi.insert(s.to_string(), true);
        }
        Model {
            counts: HashMap::new(),
            n: HashMap::new(),
            joshi,
            total: 0,
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
                *self.n.get_mut(count).unwrap() -= 1;
                *count += 1;
                *self.n.entry(*count).or_insert(0) += 1;
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

    pub fn generate(&self, quiz: &[Vec<String>], res: &mut Vec<String>, p: f64){
        let mut rng = rand_pcg::Pcg64Mcg::new(time::Instant::now().elapsed().as_nanos());
        let mut gen_str = vec![Vec::new(); 20];
        for (eb, s) in gen_str.iter_mut().enumerate() {
            self.internal_gen(quiz, p, &mut rng, s, eb)
        }
        let mut minp = 1e100;
        let mut mins = vec![String::new()];
        for s in gen_str {
            let p = self.calc_perplexity(&s,Self::prob_ft);
            if minp > p {
                minp = p;
                mins = s;
            }
        }
        for s in mins {
            res.push(s);
        }
    }

    fn internal_gen(&self, quiz: &[Vec<String>], p: f64, rng: &mut Mcg128Xsl64, gen_str: &mut Vec<String>, eb: usize) {
        let mut temp_str = vec![Vec::new(); 20];
        for s in temp_str.iter_mut() {
            let mut index = vec![0_usize; quiz.len()];
            let mut prob = vec![0.; quiz.len()];
            let mut last;
            let mut maxidx = 0;
            let mut maxval ;
            let mut particle = false;
            {
                let first = rng.gen_range(0..quiz.len());
                s.push(quiz[first][0].clone());
                index[first] += 1;
                last = first;
            }
            loop {
                maxval = 1e99;
                for i in 0..quiz.len() { 
                    s.push(quiz[i][index[i]].clone());
                    prob[i] = self.calc_perplexity(s, Self::prob_ft) + rng.gen_range(0. ..20.);
                    if particle && *self.joshi.get(&quiz[i][index[i]]).unwrap_or(&false) {
                        prob[i] = 1e100;
                    }
                    if i == last {
                        prob[i] *= p;
                    }
                    if prob[i] < maxval {
                        maxval = prob[i];
                        maxidx = i;
                    }
                    s.pop();
                }
                let branch = rng.gen_range(0..20);
                if branch == eb {
                    index[last] += 1;
                } else {
                    last = maxidx;
                    s.push(quiz[last][index[last]].clone());
                    particle = *self.joshi.get(&quiz[last][index[last]]).unwrap_or(&false);
                    index[last] += 1;
                }
                if index[last] == quiz[last].len() {
                    break;
                }
            }
        }
        let mut minp = 1e100;
        let mut mins = vec![String::new()];
        for s in temp_str {
            let p = self.calc_perplexity(&s,Self::prob_ft);
            if minp > p {
                minp = p;
                mins = s;
            }
        }
        for s in mins {
            gen_str.push(s);
        }
    }
}