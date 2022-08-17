# Build & Run Instructions
To verify the service is running after completing one of the following methods, run `curl http://localhost:8002/` or go to [http://localhost:8002/](http://localhost:8002/) in the browser. If the service is live, you'll be greeted with a success message.

## Method 1: Dev Build & Run
Pretty simple, just `cargo run`.

## Method 2: Release Build & Run
This one's a little tricker, it's `cargo run --release`.

## Method 3: Docker Build & Run
Build the image: `docker build -t service-tipinenrichment .`.

Run the image: `docker run -p 8002:8002 --name service-tie service-tipinenrichment`.

# Progress

## Actix-Web Server
[Actix-Web](https://actix.rs/) server is set up to receive the JSON object array described in the [documentation](https://github.com/Automated-Subaru-Tuning-Utilities/Documentation#request-structure-1) for tip-in-enrichment and convert these to a single vector of structs, usable by Rust.

## Models
Request (Log) and response (Correction) structs are both setup.

## Tip-In-Enrichment Calculations
Actix-Web endpoint sends vector of Log structs to tip-in-enrichment module. This module will be responsible for the calculations described in the [documentation](https://github.com/Automated-Subaru-Tuning-Utilities/Documentation#steps-for-tuning) using the data provided to it, generating a vector of Correction objects to be sent back to the actix-web endpoint to be served as the endpoint response.
