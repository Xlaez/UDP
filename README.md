## The App

When you start the server, it waits for connection from a client. When you start the client, it binds to a random port on your local  address `127.0.0.1.0`. Start the both servers on different terminals.
To quit and stop the client, you should send `quit`, if you want to get a list(array/slice) of the data you have been sending so far, send the text `my_data`.

## CLIENT

To start the client, call the command below:

```bash
$ cargo run --bin client
```

## Server

To start the server, call the command below:

```bash
$ cargo run --bin server
```

## BUILD

Do not forget to the app so that cargo installs the necessary crates:

```bash
$ cargo build
```