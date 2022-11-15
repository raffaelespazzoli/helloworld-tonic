# How to run this test

start the server

```shell
cargo run --bin helloworld-server
```

run rust client on a different terminal

```shell
cargo run --release --bin helloworld-client -- --times 100000
```

run java client:

```shell
quarkus build --native
java -jar .quarkus/helloworld/target/quarkus-app/quarkus-run.jar -t 100000
# or
./target/helloworld-1.0.0-SNAPSHOT-runner -t 100000
```
