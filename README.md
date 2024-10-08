# HoneySnare

**HoneySnare** is a smart contract honeypot designed to attract and analyze malicious bot interactions on Ethereum and EVM-compatible Layer 2 networks (such as Arbitrum, Optimism, and Base). The project includes a Rust backend that monitors, logs, and archives bot behavior in real-time, providing valuable data for further analysis.

## Features
- **Smart Contract Honeypot**: Solidity-based contracts tailored for Ethereum and Layer 2 networks to entice malicious bots.
- **Rust Backend**: A highly performant backend for real-time event monitoring, log management, and bot behavior tracking.
- **Automated Log Compression**: Logs are compressed and archived automatically for efficient storage and easy retrieval.
- **Cross-Network Support**: Designed to work on Ethereum, Arbitrum, Optimism, Base, and other EVM-compatible networks.
- **Bot Interaction Analysis**: Collects and archives interaction data for testing vulnerabilities, analyzing attack vectors, and tracking malicious bot activity.


# TODO
- Deploy Honeypot.sol to Polygon.
- Update the backend crate's main.rs to use hardcoded deployed honeypot address.
