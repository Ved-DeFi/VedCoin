# VedCoin - Decentralized Currency for Everyone

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Coverage](https://img.shields.io/badge/coverage-95%25-brightgreen.svg)]()

**VedCoin is the people's cryptocurrency - designed for individuals, businesses, and NGOs to participate in the global decentralized economy.**

## 🎯 Purpose

VedCoin serves as the grassroots adoption layer of the Ved DeFi ecosystem, enabling:
- Daily transactions and payments
- Cross-border remittances
- DeFi applications and services
- Microtransactions and micropayments
- Community governance participation

## ✨ Key Features

### 🚀 Performance
- **Fast Transactions**: Sub-second finality
- **Low Fees**: Minimal transaction costs
- **High Throughput**: 10,000+ TPS capacity
- **Scalable**: Layer 2 solutions ready

### 🔒 Security
- **Formal Verification**: Mathematically proven smart contracts
- **Audit-First**: Regular security audits
- **Multi-Signature**: Secure wallet options
- **Recovery Options**: Social recovery mechanisms

### 🌍 Accessibility
- **Multi-Platform**: Web, mobile, desktop wallets
- **Multi-Language**: 50+ language support
- **Offline Capable**: Air-gapped transactions
- **Beginner Friendly**: Intuitive user interfaces

### 🔗 Interoperability
- **Cross-Chain**: Bridge to major blockchains
- **Standard Compliant**: ERC-20 compatible layers
- **API Rich**: Comprehensive developer APIs
- **Integration Ready**: Easy merchant adoption

## 🏗️ Technical Architecture

### Core Blockchain
- **Consensus**: Delegated Proof-of-Stake (DPoS)
- **Language**: Rust with Wasm smart contracts
- **Virtual Machine**: Custom VM optimized for performance
- **Storage**: Merkle-DAG with state pruning

### Token Economics
```
Total Supply: 100,000,000,000 VED (100 billion)
Initial Circulation: 10-15 billion VED
Distribution:
├── Global Public Airdrop: 30% (30B VED)
├── Government Bridge: 25% (25B VED) 
├── Validator Rewards: 15% (15B VED)
├── Ecosystem Growth: 15% (15B VED)
├── Ved Foundation: 10% (10B VED)
└── Security & Bounties: 5% (5B VED)
```

### Governance Model
- **DAO Controlled**: Community governance through VedCoin holders
- **Proposal System**: On-chain voting for protocol upgrades
- **Delegation**: Stake delegation to active participants
- **Transparency**: All decisions recorded on-chain

## 📦 Repository Structure

```
VedCoin/
├── core/                   # Core blockchain implementation
│   ├── consensus/         # Consensus mechanism
│   ├── networking/        # P2P networking layer
│   ├── storage/           # Data storage and retrieval
│   └── vm/                # Virtual machine
├── contracts/             # Smart contracts
│   ├── token/             # VedCoin token contract
│   ├── governance/        # DAO governance contracts
│   └── defi/              # DeFi protocol contracts
├── wallets/               # Wallet implementations
│   ├── web/               # Web wallet
│   ├── mobile/            # Mobile wallet (React Native)
│   └── cli/               # Command-line wallet
├── tools/                 # Development tools
│   ├── explorer/          # Blockchain explorer
│   ├── faucet/            # Testnet faucet
│   └── validator/         # Validator node setup
├── docs/                  # Documentation
├── tests/                 # Test suites
└── scripts/               # Build and deployment scripts
```

## 🚀 Quick Start

### Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/Ved-DeFi/VedCoin.git
cd VedCoin

# Build the project
cargo build --release

# Run tests
cargo test

# Start local node
./target/release/vedcoin-node --dev
```

### Running a Validator Node

```bash
# Generate validator keys
./target/release/vedcoin-node key generate

# Start validator
./target/release/vedcoin-node \
  --validator \
  --name "YourValidatorName" \
  --chain mainnet
```

### Using the Wallet

```bash
# Create new wallet
./target/release/vedcoin-wallet new

# Check balance
./target/release/vedcoin-wallet balance

# Send transaction
./target/release/vedcoin-wallet send <recipient> <amount>
```

## 📱 Wallet Applications

### Web Wallet
Access your VedCoin through our secure web interface:
- **URL**: https://wallet.vedcoin.org
- **Features**: Full functionality, DeFi integration
- **Security**: Hardware wallet support, multi-sig

### Mobile Wallet
Download our mobile apps:
- **iOS**: Available on App Store
- **Android**: Available on Google Play
- **Features**: QR code scanning, biometric auth, offline mode

### Hardware Wallet Support
Compatible with leading hardware wallets:
- Ledger Nano S/X
- Trezor Model T
- SafePal S1

## 🔧 Developer Resources

### APIs and SDKs
- **JSON-RPC**: Standard blockchain API
- **GraphQL**: Flexible data queries
- **WebSocket**: Real-time updates
- **REST**: HTTP-based API

### SDK Support
- **JavaScript/TypeScript**: Web and Node.js
- **Python**: Data analysis and automation
- **Rust**: Native performance
- **Go**: Server applications

### Documentation
- [API Reference](./docs/api-reference.md)
- [Smart Contract Guide](./docs/smart-contracts.md)
- [Wallet Integration](./docs/wallet-integration.md)
- [DeFi Protocols](./docs/defi-protocols.md)

## 🌟 Use Cases

### For Individuals
- Daily payments and transfers
- Savings and investment
- Cross-border remittances
- Governance participation

### For Businesses
- E-commerce payments
- Supply chain financing
- Employee payments
- Customer rewards

### For NGOs
- Transparent donations
- Aid distribution
- Impact tracking
- Community funding

## 🛡️ Security

### Audits
- [Audit Report #1](./audits/audit-report-1.pdf) - CertiK (2025-Q1)
- [Audit Report #2](./audits/audit-report-2.pdf) - Trail of Bits (2025-Q2)
- [Bug Bounty Program](https://vedcoin.org/bug-bounty)

### Security Features
- Formal verification of core contracts
- Multi-signature wallet support
- Time-locked transactions
- Emergency pause mechanisms

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](./CONTRIBUTING.md).

### Development Process
1. Fork the repository
2. Create a feature branch
3. Write tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

### Code Standards
- Rust code follows rustfmt guidelines
- All public APIs must be documented
- Tests required for new features
- Security review for sensitive changes

## 📞 Support

- **Discord**: https://discord.gg/vedcoin
- **GitHub Issues**: Report bugs and request features
- **Email**: support@vedcoin.org
- **Documentation**: https://docs.vedcoin.org

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

---

**Join the VedCoin community and help build the future of decentralized finance!**
