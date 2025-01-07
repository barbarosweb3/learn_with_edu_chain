# Learn with EduChain

Learn with EduChain is an innovative blockchain-integrated education platform developed as part of the OpenCampus EduChain Hackathon. It focuses on providing a secure and transparent learning environment where courses can be created, managed, and certificates can be issued on the blockchain.

## 🎯 Project Overview

The Learning Module offers the following core functionalities:

- Course creation and management
- Blockchain-based certificate issuance
- Integration with OpenCampus ID for authentication
- Secure and verifiable educational credentials

## 🛠 Technology Stack

- **Backend**: Rust with Actix-web framework
- **Database**: PostgreSQL with SQLx
- **Blockchain Integration**: Ethereum (via ethers-rs)
- **Authentication**: OpenCampus ID OAuth2
- **Smart Contracts**: Solidity (Contract ABIs included)

## 🏗 Architecture

The project follows a modular architecture:

```
src/
├── api/           # REST API endpoints
├── blockchain/    # Blockchain integration layer
├── models/        # Data models
├── services/      # Business logic
├── lib.rs         # Library exports
└── main.rs        # Application entry point
```

## 🚀 Getting Started

### Prerequisites

- Rust (Latest stable version)
- PostgreSQL
- Node.js (for smart contract deployment)
- OpenCampus ID credentials

### Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/learn_with_edu_chain.git
cd learn_with_edu_chain
```

2. Set up environment variables in `.env`:

```
DATABASE_URL="postgresql://username:password@localhost:5432/opencampus_db"
BLOCKCHAIN_URL="http://localhost:8545"
CONTRACT_ADDRESS="0x..."
PRIVATE_KEY="0x..."
```

3. Create and migrate the database:

```bash
createdb opencampus_db
sqlx migrate run
```

4. Build and run the application:

```bash
cargo build
cargo run
```

## 🔐 Authentication

The platform uses OpenCampus ID for authentication. Users can:

- Login with their OpenCampus credentials
- Access their courses and certificates
- Verify their identity for certificate issuance

## 📚 Features

### Course Management

- Create new courses with metadata
- Set course pricing
- Track course progress
- Manage course content

### Certificate Issuance

- Issue blockchain-verified certificates
- Verify certificate authenticity
- Track certificate history
- Export certificates

## 🌐 API Endpoints

### Courses

- `POST /api/v1/courses` - Create a new course
- `GET /api/v1/courses` - List courses
- `GET /api/v1/courses/{id}` - Get course details

### Certificates

- `POST /api/v1/certificates` - Issue a certificate
- `GET /api/v1/certificates/{id}` - Verify a certificate

## 🤝 Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🌟 Acknowledgments

- OpenCampus Team for providing the infrastructure
- EduChain Hackathon organizers
- All contributors and participants

## 📞 Contact

For questions and support, please contact:

- Email: thelandofcodes@gmail.com
- Twitter: @edutech_dev
