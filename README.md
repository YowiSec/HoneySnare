<h1 align="center">HoneySnare</h1>

<p align="center">
  <img src="HoneySnareLogo.png" alt="Logo" width="400">
</p>

**HoneySnare** is an advanced multi-chain honeypot system designed to attract and analyze malicious bot interactions across multiple blockchain networks. Initially focused on EVM-compatible chains (Arbitrum, Optimism, Base, and Blast) with planned expansion to Solana, this project serves as a research tool for Web3 security analysis and bot behavior tracking.

## Features
- **Multi-Chain Honeypot System**: 
  - EVM Networks: Arbitrum, Optimism, Base, and Blast support
  - Solana Support (Coming Soon)
  - Cross-chain interaction monitoring and analysis
- **Advanced Smart Contracts**:
  - Sophisticating baiting mechanisms
  - Multiple vulnerability vectors for research
  - Chain-specific optimizations
- **Modern Rust Backend Infrastructure**:
  - Built with `alloy-rs`: Latest high-performance EVM framework
  - Type-safe contract interactions
  - Efficient ABI encoding/decoding
  - High-performance event monitoring
  - Automated log management and archiving
  - Multi-chain RPC handling
  - Real-time interaction tracking
- **Security Research Tools**:
  - Bot behavior analysis
  - Attack vector documentation
  - Cross-chain comparison analytics
- **Automated Operations**:
  - GitHub Actions integration for continuous monitoring
  - Automated log compression and archiving
  - Daily status checks and updates

## Technical Implementation
### Backend Architecture
The backend is built using modern Rust tooling:
```rust
use alloy_sol_types::sol;  // Modern EVM type system
use alloy_primitives::Address;  // Type-safe EVM primitives
use alloy_sol_types::SolEvent;  // Efficient event handling

// Example of type-safe event handling with alloy-rs
sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
}
```

Key technical features:
- **alloy-rs Integration**: Using the latest EVM crate that supersedes ethers-rs
- **Type-safe Contract Interactions**: Leveraging Rust's type system for robust EVM interactions
- **Efficient Event Parsing**: Using alloy-rs's optimized event decoding
- **Cross-Chain Compatibility**: Unified interface for multiple EVM chains

## Current Status
- **Active Development**: Project is being enhanced with multi-chain support
- **Deployment Progress**:
  - Arbitrum: Initial deployment and testing phase
  - Other EVM Chains: Pending deployment
  - Solana: In development
- **Monitoring System**: Active on Arbitrum, ready for multi-chain expansion

## Site Deployment
Monitor live interactions at: [https://brief-sandwich-sparse.on-fleek.app/](https://brief-sandwich-sparse.on-fleek.app/)

## Project Structure
```
honeysnare/
├── contracts/          # Smart contract implementations
│   ├── src/           # Contract source files
│   └── test/          # Contract test suite
├── backend/           # Rust monitoring system
│   ├── src/           # Backend source code
│   └── logs/          # Interaction logs
└── .github/workflows/ # Automated monitoring setup
```

## Setup Guide
1. **Environment Setup**
   ```bash
   # Clone repository
   git clone https://github.com/yourusername/honeysnare.git
   cd honeysnare

   # Set up environment variables
   cp .env.example .env
   # Add your RPC URLs for each chain
   ```

2. **Contract Deployment**
   ```bash
   cd contracts
   forge build
   forge script script/Deploy.s.sol --rpc-url $YOUR_RPC_URL --broadcast --verify
   ```

3. **Backend Configuration**
   ```bash
   cd backend
   # Update chain configs with deployed addresses
   cargo build --release
   ```

## Development Roadmap
- [x] Initial Arbitrum honeypot implementation
- [x] Modern Rust backend with alloy-rs
- [ ] Multi-chain deployment
- [ ] Solana integration
- [ ] Enhanced analytics dashboard
- [ ] Cross-chain correlation analysis

## Contributing
This project is part of ongoing Web3 security research. While the core contracts and monitoring system are under active development, suggestions and discussions are welcome.

## Security Note
This project is designed for security research and educational purposes. The contracts intentionally contain vulnerabilities for research purposes.

## License
GNU General Public License v3.0

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
