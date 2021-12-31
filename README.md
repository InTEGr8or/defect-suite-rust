# My First REST Client in Rust

I'm thinking about re-implementing some current working and in-development projects to Rust, including this Rally CLI client.

Pulling Defect Suite JSON data is not nearly as straightforward as it is in PowerShell.

I wanted to post this example that I just got working in case anyone else needs to create a Rust REST client that requires custom request headers and has nested JSON which has to be implemented in Rust structs.