# Lunch-n-Learn-Rust

Welcome to **Lunch-n-Learn-Rust**! This repository is designed for hands-on learning and exploration of Rust programming during a fun and interactive *Lunch and Learn* session. Participants will have the chance to dive into Rust by building their own client applications and connecting to a Rust-based server hosted on my laptop.

## Purpose

The goal of this project is to:

- Provide an engaging introduction to Rust programming.
- Demonstrate the client-server model using Rust.
- Encourage hands-on participation, with each participant building and testing their own Rust client app.
- Showcase Rust's strengths in building reliable and performant systems.

## How It Works

### Server

The repository includes a Rust-based server that:
- Listens for client connections.
- Accepts simple requests from client applications.
- Sends responses to connected clients.

The server is pre-built and ready to run on my laptop during the session.

### Client

Participants will:
- Write their own client applications in Rust.
- Connect to the server.
- Interact with the server by sending requests and processing responses.

The client code can be as simple or as complex as you like, depending on your comfort level with Rust.

## Features

- A **basic Rust server** to demonstrate networking fundamentals.
- Easy-to-follow examples and instructions for writing Rust client applications.
- Opportunities to experiment with extending the project during the session.

## Getting Started

### Prerequisites

To participate, you will need:
- **Rust installed** on your system. You can install it using [rustup](https://rustup.rs/).
- A code editor or IDE of your choice (e.g., VS Code, IntelliJ IDEA, or the Rust plugin for Vim/Neovim).

### Steps to Join

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/Lunch-n-Learn-Rust.git
   cd Lunch-n-Learn-Rust
   ```

2. Start the server:
   ```bash
   cargo run --bin server
   ```

3. Create your Rust client app following the code in the `client/` folder.

4. Connect your client to the server and have fun!

## Learning Objectives

By the end of the session, participants will have:
- Learned how to set up and run a Rust server.
- Written a Rust client app to communicate with the server.
- Gained hands-on experience with Rust's networking libraries and programming model.

**Note:** This project is for educational purposes only and is not intended for production use.
