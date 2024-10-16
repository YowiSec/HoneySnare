<p align="center">
  <img src="HoneySnareLogo.png" alt="Logo" width="400">
</p>

<h1 align="center">HoneySnare</h1>

**HoneySnare** is a smart contract honeypot designed to attract and analyze malicious bot interactions on Ethereum and EVM-compatible Layer 2 networks (such as Arbitrum, Optimism, and Base). The project includes a Rust backend that monitors, logs, and archives bot behavior on a daily basis, providing valuable data for further analysis.

## Features
- **Smart Contract Honeypot**: Solidity-based contracts tailored for Ethereum and Layer 2 networks to entice malicious bots.
- **Rust Backend**: A highly performant backend for daily event monitoring, log management, and bot behavior tracking.
- **Automated Log Compression**: Logs are compressed and archived automatically for efficient storage and easy retrieval.
- **Cross-Network Support**: Designed to work on Ethereum, Arbitrum, Optimism, Base, and other EVM-compatible networks.
- **Bot Interaction Analysis**: Collects and archives interaction data for testing vulnerabilities, analyzing attack vectors, and tracking malicious bot activity.
- **GitHub Actions Integration**: Automated daily checks and log updates using GitHub Actions.

## Deployment and Testing
- The Honeypot smart contract has been deployed to Arbitrum.
- The backend has been successfully deployed to Fleek.
- The dApp has been tested by making several calls to the Honeypot contract, confirming its functionality and logging capabilities.

## Site Deployment
You can view the latest logs at the Fleek-hosted URL: [https://brief-sandwich-sparse.on-fleek.app/](https://brief-sandwich-sparse.on-fleek.app/)

Alternatively, you can check the logs folder in the backend crate for local access to log files.

## Project Structure
- `contracts/`: Contains the Solidity smart contracts for the honeypot.
- `backend/`: Houses the Rust backend code for monitoring and logging.
- `.github/workflows/`: Contains the GitHub Actions workflow for automated daily checks.

## Setup and Configuration
1. Clone the repository
2. Set up the required environment variables (see `.env.example`)
3. Deploy the smart contract to your chosen network
4. Update the backend with the deployed contract address
5. Run the backend locally or deploy to your preferred hosting service

## Usage
- Monitor the Fleek-hosted site for daily log updates
- Analyze the archived logs for insights into bot behavior
- Use the GitHub Actions workflow for automated daily checks and log updates
