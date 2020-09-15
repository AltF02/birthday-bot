![Build](https://github.com/DankDumpster/birthday-bot/workflows/Build/badge.svg)
# Birthday bot
Birthdaybot is a discord bot that keeps track of everyone's birthday! So you won't have to remember everything, 
and they even get a really cool role. Currently this needs to be self hosted for every server,
but I'm planning on making a running version that'll work for multiple servers

# Run Your own Instance

### Requirements
* [Rust](https://www.rust-lang.org/tools/install)
* [Git](https://git-scm.com/downloads)

## Building
```shell script
git clone https://github.com/DankDumpster/birthday-bot/
cd ./birthday-bot
cargo build --release
mv /target/release/birthday-bot .
```
Now we can run the build 
```shell script
./bithday-bot
```
###### NOTE: this will panic
This will generate a config.yml, fill that out. Once you've done that you can run it again and enjoy!

## Developement environment 

First we need to install the sqlx-cli
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
```shell script
cargo run
```

If you run into any issues please don't hesitate to open an issue
