
# Portfolio Program on Solana Blockchain

This Solana program allows users to manage their portfolios on the blockchain. Users can create and update their portfolios, store links and images, request and approve vouches, send and receive messages, and receive tips.

## Usage

To use this program, follow these steps:

1. **Set Up Your Development Environment:** Ensure you have Rust and Solana CLI installed on your machine.

2. **Clone the Repository:** Clone this repository to your local machine.

3. **Navigate to the Portfolio Program Directory:** Open a terminal and navigate to the directory containing the Portfolio program.

4. **Build the Program:** Run `cargo build-bpf` to build the program.

5. **Deploy the Program:** Deploy the program to the Solana blockchain using the Solana CLI. You'll need to fund your program account with SOL tokens.

6. **Interact with the Program:** You can now interact with the program using client applications or the Solana CLI. Refer to the program code and associated instructions for details on available actions and their parameters.

## Program Structure

The Portfolio program consists of several modules:

- **Initialize:** Initializes the portfolio.
- **CreatePortfolio:** Creates a new portfolio or updates the bio of an existing portfolio.
- **StoreLinks:** Stores links in the portfolio.
- **StoreImage:** Stores an image URL in the portfolio.
- **RequestVouch:** Requests a vouch for the portfolio.
- **ApproveVouch:** Approves a vouch for the portfolio.
- **SendMessage:** Sends a message to the portfolio owner.
- **Tip:** Receives a tip for the portfolio.

## Account Structures

The program defines several account structures:

- **Portfolio:** Contains information about the portfolio, including bio, links, image URL, vouches, vouch requests, messages, and tip amount.
- **Vouch:** Represents a vouch given to a portfolio by another user.
- **VouchRequest:** Represents a request for a vouch made by a portfolio.
- **Message:** Represents a message sent to a portfolio owner.

## Error Handling

The program includes an error code for unauthorized actions.
