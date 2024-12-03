# tracker creator

这里是跟踪爆发的meme币的创始人地址的，目前仅仅是记录在sqlite中，希望后面能用上。当这些地址去买其他的币时，我们跟一跟！


# 使用
1. install sqlx
```bash
cargo install sqlx-cli --no-default-features --features sqlite
```
2. 创建目录
```bash
mkdir data
```
3. 添加环境变量
```bash
cat .env
DATABASE_URL=sqlite:data/tracker.db
```
4. 创建数据库
```bash
sqlx database create
sqlx migrate run
```
5. 配置文件
```bash
cp app.toml.example app.toml
```
6. 运行
```bash
cargo b -r
./target/debug/tracker-creator -c app.toml
```


