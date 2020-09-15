![Rust](https://github.com/DankDumpster/birthday-bot/workflows/build/badge.svg)
# Birthday bot
Birthdaybot is a discord bot that keeps track of everyone's birthday! So you won't have to remember everything, 
and they even get a really cool role. Currently this needs to be self hosted for every server,
but I'm planning on making a running version that'll work for multiple servers

## How to set up

first we need to install the sqlx-cli
```shell script
cargo install --version=0.1.0-beta.1 sqlx-cli --no-default-features --features postgres
```

Now we can run it, this will take some time as it needs to compile everything
```shell script
cargo run
```
###### NOTE: this will panic

Now it will generate a config.yml for us, fill that out. If something else goes wrong for any reason you can create the config.yml manually with these values
```yaml
---
token: ""
prefix: .
db_uri: "postgres://postgres:password@localhost/postgres"
role_id: 0
guild_id: 0
location: "./config.yml"
```

Now once that is done we can run it again and it should work now
```
cargo run
```

Now if you want to run it on a server of some sort use the release build
```shell script
cargo build --release
```
This will create a executable in /target/release, you can use this executable everywhere without needing the other files
###### NOTE: You will still need config.yml or it will generate one for you

If you run into any issues don't hesitate to open an issue
