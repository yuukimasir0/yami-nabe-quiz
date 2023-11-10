use std::fs::File;
use std::env;
use vibrato::{Dictionary, Tokenizer};

/*
辞書データのダウンロード
wget https://github.com/daac-tools/vibrato/releases/download/v0.5.0/ipadic-mecab-2_7_0.tar.xz
tar xf ipadic-mecab-2_7_0.tar.xz
*/

pub fn wakachi(text: &str) -> String {
    
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let dict_path = "ipadic-mecab-2_7_0/system.dic.zst";
    let dict_full_path = current_dir.join(dict_path);

    // 辞書ファイルのロード
    let reader = zstd::Decoder::new(File::open(dict_full_path).unwrap()).unwrap();
    let dict = Dictionary::read(reader).unwrap();

    // トークナイザーの生成
    let tokenizer = Tokenizer::new(dict)
        .ignore_space(true).unwrap()
        .max_grouping_len(24);

    // ワーカーの初期化
    let mut worker = tokenizer.new_worker();

    worker.reset_sentence(text);
    worker.tokenize(); // 形態素解析の実行。mutable self

    let res = worker.token_iter()
        .map(|t| { // 出力
            format!("{}", t.surface())
        }).collect::<Vec<_>>().join(" ");
    
    res
}