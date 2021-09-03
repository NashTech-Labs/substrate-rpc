# substrate-rpc

This template will help you to create your own RPC that call a Runtime API to interact with a substrate node.
I am using -

- Rust to write the code.
- Substrate as the framework to build a blockchain node.

## Building the project

- Clone this repo and run the following command in the root folder to build the node.
  ```
  cargo build --release
  ```

## Run the project

- Now run the command below to make your substrate node active at port 9944. 
  ```
  ./target/release/node-template --dev --tmp
  ```
  
Now you are good to go, open the polkadot js front-end and try to access the custom RPC we just installed.



