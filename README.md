# task-tree

## Getting Started

### Installing Launching the model

    curl -fsSL https://ollama.com/install.sh | sh
 
    ollama run qwen2.5-coder:7b-instruct

### Building and running the Rust component

    cargo build

then 

    cargo run -- 3

in order to invoke the CLI with an argument of 3.
Outputs will be saved in `out1.txt`, `out2.txt` and `out3.txt`.