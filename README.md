# Solana Token Backend System

A production-ready custodial wallet system for managing SPL tokens on the Solana Mainnet, built with Rust and Axum.

## Project Overview

This project provides a complete backend infrastructure for applications that require Solana blockchain integration without burdening users with the complexities of crypto wallet management. It creates a "web2-like" experience where users interact with standard email/password credentials, while the system handles wallet creation, private key security, and blockchain transactions in the background.

## Target Audience and Use Cases

This system is designed for developers and businesses who:

*   **Require a Loyalty or Reward System:** Integrate points or tokens into an existing application where the "points" are actual on-chain SPL tokens.
*   **Need Custodial Wallet Management:** Want to provide users with Solana wallets but prefer to manage the security and custody of keys centrally, similar to how an exchange operates.
*   **Want to Subsidize Gas Fees:** The system uses a master treasury wallet to pay for all transaction fees, ensuring a seamless user experience where users never need to hold SOL to transact.
*   **Seek High Performance and Security:** Built with Rust for safety and speed, using industry-standard encryption for sensitive data.

## Key Features

*   **Custodial Wallet Generation:** Automatically generates a unique Solana wallet for every registered user.
*   **Bank-Grade Encryption:** User private keys are encrypted using AES-256-GCM before being stored in the database. The keys are only decrypted in memory for the brief moment required to sign a transaction.
*   **JWT Authentication:** Secure, stateless authentication using JSON Web Tokens.
*   **Spl Token Support:** Full native support for SPL tokens (e.g., USDC, or custom project tokens).
*   **Mainnet Ready:** Configured for the Solana Mainnet Beta, ensuring real-world applicability.
*   **Containerized Deployment:** Includes Docker support for consistent deployment across any environment.

## Technical Architecture

The application is structured as a modular Rust service:

*   `src/main.rs`: Entry point and server initialization.
*   `src/routes.rs`: API definitions and HTTP request handlers.
*   `src/solana_service.rs`: Core logic for interacting with the Solana blockchain.
*   `src/encryption.rs`: Security module for handling key encryption and decryption.
*   `src/db.rs`: Database interaction layer using SQLite.

## Prerequisites

To run this system, you will need:

1.  **Docker Desktop** (Recommended for deployment).
2.  **Solana RPC URL:** An endpoint from a provider like Helius or Alchemy for connecting to the Solana Mainnet.
3.  **SPL Token Mint Address:** The address of the token you wish to distribute or manage.
4.  **Master Wallet:** A funded Solana wallet to act as the treasury and fee payer.

## Quick Start with Docker

The most efficient way to run the application is via Docker.

1.  **Clone the repository**
    
    Clone the codebase to your local machine.

2.  **Configuration**

    Create a `.env` file in the root directory. You can copy the provided example:
    
    `cp .env.example .env`
    
    Open the `.env` file and strictly follow the comments to populate your secrets (RPC URL, specific keys, etc.).

3.  **Launch the Service**

    Run the following command to build and start the container:

    `docker-compose up -d`

    The initial build process may take a few minutes as it compiles the Rust dependencies. Once complete, the API will be accessible at `http://localhost:3000`.

## Manual Installation (Rust Native)

If you prefer to run the application without Docker:

1.  Ensure **Rust** (version 1.70 or later) is installed on your system.
2.  Navigate to the project directory.
3.  Configure the `.env` file as described above.
4.  Execute the run command:

    `cargo run`

## Security Recommendations

This system was built with security in mind, but for a production environment, you should observe the following practices:

*   **Environment Variables:** Never commit your `.env` file to version control.
*   **Secret Management:** For high-value deployments, consider using a dedicated secret management service (like AWS Secrets Manager or Vault) instead of a simple `.env` file.
*   **HTTPS:** Always run this service behind a reverse proxy (like Nginx) with SSL/TLS enabled.
*   **Key Rotation:** Regularly rotate your JWT secrets and database encryption keys.

## API Documentation

The API exposes the following primary endpoints. All protected routes require a valid Bearer token in the Release header.

### Public Routes

*   `POST /signup`: Register a new user and generate their wallet.
*   `POST /login`: Authenticate an existing user and receive a JWT.

### Protected Routes

*   `GET /balance`: Retrieve the SOL and Token balance of the authenticated user.
*   `POST /transfer`: Send tokens from the user's wallet to another address.
*   `POST /buy-token`: Simulates a purchase where the master wallet sends tokens to the user.
*   `POST /reward`: Admin endpoint to airdrop tokens to a specific user.

## API Testing and Verification

To facilitate easy testing and integration, we have included a **Postman Collection** in the repository: `Solana_Token_Backend_UPDATED.postman_collection.json`.

You can import this file into Postman to immediately test the following capabilities:
*   **Debiting Tokens:** Deducting SPL tokens from a user's wallet.
*   **Crediting Tokens:** Distributing SPL tokens to a user as a reward or transfer.
*   **Balance Checks:** Real-time verification of on-chain balances.

This collection provides a practical demonstration of how the REST API bridges standard HTTP requests with complex blockchain operations.

## Official Resources

For developers willing to extend this project or learn more about the underlying technologies, we recommend the following official documentation:

*   **Solana Blockchain:** [Solana Documentation](https://solana.com/docs)
*   **Rust Programming Language:** [The Rust Book](https://doc.rust-lang.org/book/)
*   **Axum Web Framework:** [Axum Documentation](https://docs.rs/axum/latest/axum/)
*   **SPL Token Library:** [Solana Program Library (SPL)](https://spl.solana.com/token)
*   **Docker:** [Docker Documentation](https://docs.docker.com/)
*   **Helius (RPC Provider):** [Helius Documentation](https://docs.helius.dev/)

## License

This project is licensed under the MIT License.

## Support and Contact

For further information, architectural details, or specific implementation questions, please feel free to reach out to the project maintainer directly. We are happy to assist with integration queries.
