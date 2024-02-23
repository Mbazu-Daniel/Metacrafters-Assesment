
# Senddit Solana Program

Senddit is a Solana program that simulates a simple Reddit-like platform on the Solana blockchain. It allows users to post links, upvote posts, store comments, and upvote comments.

## Usage

To use this program, you'll need to deploy it to the Solana blockchain. Here's a step-by-step guide:

1. **Set Up Your Development Environment:** Make sure you have Rust and Solana CLI installed on your machine.

2. **Clone the Repository:** Clone this repository to your local machine.

3. **Navigate to the Senddit Directory:** Open a terminal and navigate to the directory containing the Senddit program.

4. **Build the Program:** Run `cargo build-bpf` to build the program.

5. **Deploy the Program:** Deploy the program to the Solana blockchain using the Solana CLI. You'll need to fund your program account with SOL tokens.

6. **Interact with the Program:** You can now interact with the program using client applications or the Solana CLI. Refer to the program code and associated instructions for details on available actions and their parameters.

## Program Structure

The Senddit program consists of several modules:

- **Initialize:** Initializes the Senddit program.
- **InitPostStore:** Initializes the post store.
- **PostLink:** Posts a link.
- **UpvotePost:** Upvotes a post.
- **InitCommentStore:** Initializes the comment store.
- **PostComment:** Posts a comment.
- **UpvoteComments:** Upvotes a comment.

## Utility Functions

The program includes a utility function `payout_fees` to handle fee payouts.

## Account Structures

The program defines several account structures:

- **Senddit:** Contains information about the Senddit program.
- **PostStore:** Stores information about posts.
- **Post:** Represents a post on the platform.
- **CommentStore:** Stores information about comments.
- **Comment:** Represents a comment on the platform.

## Error Handling

The program includes error codes to handle various error conditions, such as overflow/underflow and invalid inputs.


