# **VoteChain Relay Server 🚀**

The **VoteChain Relay Server** is a microservice designed to facilitate **meta-transactions** for the **VoteChain system**. It enables users to cast votes on the blockchain without directly paying gas fees. The relay server handles transaction signing, fee payment, and forwarding transactions to the blockchain seamlessly.

It also incorporates **JWT-based authentication** and **session management** using **PostgreSQL** via the **Diesel ORM**.

---

## **🧩 Features**

- **Meta-transactions**: Users sign transactions offline; the server pays gas fees and relays them.
- **Blockchain Integration**: Connects to an Ethereum-compatible blockchain using RPC.
- **JWT Authentication**: Secure API access with JSON Web Tokens.
- **Session Management**: Stores session data using PostgreSQL.
- **RESTful API**: Provides routes for managing polls and votes.

---

## **🚀 Getting Started**

Follow these steps to set up and run the project locally.

### **1. Prerequisites**

Ensure you have the following installed:

- **Rust** (latest stable version)
- **Docker & Docker Compose** (for PostgreSQL)
- **Node.js** (optional, for interacting with MetaMask)

---

### **2. Setup**

1. **Clone the Repository**

   ```bash
   git clone https://github.com/your-repo/votechain-relay.git
   cd votechain-relay
   ```

2. Environment Configuration

   Copy the example environment file and configure it.

   ```bash
   cp .env.example .env
   ```

   > Remember to update `.env` with your settings (e.g., RPC URL, database credentials, JWT secrets, wallet private key).

3. Start PostgreSQL Database

   Use Docker Compose to run PostgreSQL locally:

   ```bash
   docker-compose up -d
   ```

4. Install Dependencies

   ```bash
   cargo build
   ```

5. Run Database Migrations

   Apply database schema using Diesel.

   ```bash
   diesel migration run
   ```

6. Run the Server

   Start the Actix-Web server:

   ```bash
   cargo run
   ```

> The server will be available at the local address defined in `.env` (default: `http://localhost:8080`).

## 📡 API Details

Below is a quick overview of the API routes:

### **Authentication**

| **Method** | **Endpoint**    | **Description**            |
| ---------- | --------------- | -------------------------- |
| POST       | `/auth/signin`  | Sign in a user and get JWT |
| POST       | `/auth/refresh` | Refresh access token       |

---

### **Poll Management**

| **Method** | **Endpoint**       | **Description**               |
| ---------- | ------------------ | ----------------------------- |
| GET        | `/polls`           | Retrieve all available polls  |
| GET        | `/polls/{id}`      | Retrieve a specific poll      |
| POST       | `/polls/create`    | Create a new poll on-chain    |
| POST       | `/polls/cast_vote` | Cast a vote in an active poll |

---

### **Health Check**

| **Method** | **Endpoint** | **Description**     |
| ---------- | ------------ | ------------------- |
| GET        | `/health`    | Check server health |

## 🛠️ Tech Stack

This project leverages the following libraries and technologies:

- Rust: Core programming language.

  - Actix-Web: High-performance web framework for API development.
  - Diesel: ORM for managing PostgreSQL database connections and migrations.
  - jsonwebtoken: Library for handling JWT-based authentication.
  - alloy-rs: For interacting with Ethereum-compatible blockchains (e.g., RPC, signing).

- PostgreSQL: Relational database for storing session data.

## 📝 VoteChain Ecosystem

The **VoteChain Relay Server** is a critical component of the **VoteChain ecosystem**, which provides a decentralized, blockchain-based voting platform. It ensures seamless interaction between users, the blockchain, and off-chain services. The ecosystem includes:

1. **VoteChain Smart Contracts** (`votechain`):

   - Written in **Solidity**, these contracts manage all on-chain voting logic, including poll creation, vote casting, and result aggregation.
   - Designed to be immutable and transparent, ensuring trust in the voting process.

2. **VoteChain Relay Server** (_this project_):

   - Facilitates **meta-transactions** to offload gas fees from users by relaying signed transactions to the blockchain.
   - Manages secure authentication using **JWT tokens** and session data stored in a **PostgreSQL** database.
   - Acts as a bridge between the blockchain and the client application, providing a user-friendly API.

3. **VoteChain Client Application** (`votechain-client`):
   - A front-end application for end-users to interact with the VoteChain system.
   - Allows users to create polls, cast votes, and view results, utilizing wallets like **MetaMask** for signing transactions.
   - Connects to the relay server to submit signed transactions and fetch poll data.

Together, these components form a **secure, gas-optimized, and user-friendly voting solution**, leveraging blockchain technology for transparency and decentralization.
