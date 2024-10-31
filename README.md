### Setup
`cargo make`と`mold`入れたら多分動くと思う(`Docker`の環境構築がすんでることは前提条件．`sudo`無しで実行できること前提で書いてるので，`sudo`無しで実行できるようにして．)

```sh
sudo apt install mold
cargo install --locked cargo-make
```

ビルド
```sh
cargo make build
```

起動
```sh
cargo make run
```
起動したら適切に動作していることを確認してください．
`localhost:8080/docs`に行ってドキュメントが起動していればOKだと思います．
あるいはhealth checkしてもらえればよい．
下記のコマンドを実行して`200 OK`が帰っていればよい．  
health check
```sh
curl -v "http://localhost:8080/api/v1/health"
```

db
```sh
curl -v "http://localhost:8080/api/v1/health/db"
```


コンテナのリセット
```sh
cargo make compose-remove
```

実装する際は常時次のコマンドをターミナルで実行した状態でソースを記述してください．
なお，オートセーブは非推奨です(10敗くらいしていい加減切った)
```sh
cargo make watch
```

