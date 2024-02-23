// This code defines a Solana program called "senddit" which simulates a simple Reddit-like platform on the Solana blockchain.
// It allows users to post links, upvote posts, store comments, and upvote comments.

// Import necessary libraries
use anchor_lang::prelude::*;
use solana_program::{
    program::invoke,
    system_instruction::transfer,
    pubkey::Pubkey,
};

// Define program ID
declare_id!("BadaeRW55gEgAaZ8sgbi2jxPC24GklToOjuCJB3u");

// Define the "senddit" module
#[program]
pub mod senddit {
    use super::*;

    // Initialize the senddit program
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Extract accounts from the context
        let senddit: &mut Account<Senddit> = &mut ctx.accounts.senddit;
        let authority: &mut Signer = &mut ctx.accounts.authority;

        // Set initial values for the senddit account
        senddit.authority = authority.key();
        senddit.treasury = authority.key();
        senddit.fee = (0.001 * 1e9) as u64; // 0.001 SOL
        Ok(())
    }

    // Initialize the post store
    pub fn init_post_store(ctx: Context<InitPostStore>) -> Result<()> {
        // Extract accounts from the context
        let senddit: &mut Account<Senddit> = &mut ctx.accounts.senddit;
        let treasury: &mut UncheckedAccount = &mut ctx.accounts.treasury;
        let post_store: &mut Account<PostStore> = &mut ctx.accounts.post_store;
        let authority: &mut Signer = &mut ctx.accounts.authority;

        // Pay out fees and reset post store fields
        payout_fees(treasury, authority, senddit, None);
        post_store.posts = 0;
        post_store.bump = *ctx.bumps.get("post_store").unwrap();

        Ok(())
    }

    // Post a link
    pub fn post_link(ctx: Context<PostLink>, link: String) -> Result<()> {
        // Extract accounts from the context
        let senddit: &mut Account<Senddit> = &mut ctx.accounts.senddit;
        let treasury: &mut UncheckedAccount = &mut ctx.accounts.treasury;
        let post_store: &mut Account<PostStore> = &mut ctx.accounts.post_store;
        let post: &mut Account<Post> = &mut ctx.accounts.post;
        let authority: &mut Signer = &mut ctx.accounts.authority;
        let poster_wallet: &mut UncheckedAccount = &mut ctx.accounts.poster_wallet;

        // Pay out fees to the poster and the platform
        payout_fees(poster_wallet, authority, senddit, Some(treasury));

        // Check if the link is valid
        if link.len() == 0 {
            return Err(ErrorCode::NoTextSubmitted.into());
        }
        if link.len() > 196 {
            return Err(ErrorCode::LinkTooLarge.into());
        }

        // Increment post count and set post fields
        post_store.posts = post_store
            .posts.checked_add(1)
            .ok_or(ErrorCode::OverflowUnderflow)?;
        post.authority = authority.key();
        post.link = link;
        post.upvotes = 1;
        post.comments = 0;
        post.bump = *ctx.bumps.get("post").unwrap();

        Ok(())
    }

    // Upvote a post
    pub fn upvote_post(ctx: Context<UpvotePost>, _number: String) -> Result<()> {
        // Extract accounts from the context
        let senddit: &mut Account<Senddit> = &mut ctx.accounts.senddit;
        let treasury: &mut UncheckedAccount = &mut ctx.accounts.treasury;
        let post: &mut Account<Post> = &mut ctx.accounts.post;
        let authority: &mut Signer = &mut ctx.accounts.authority;
        let poster_wallet: &mut UncheckedAccount = &mut ctx.accounts.poster_wallet;

        // Pay out fees to the voter and the platform
        payout_fees(poster_wallet, authority, senddit, Some(treasury));

        // Increment upvotes count of the post
        post.upvotes = post
            .upvotes.checked_add(1)
            .ok_or(ErrorCode::OverflowUnderflow)?;

        Ok(())
    }

    // Initialize the comment store
    pub fn init_comment_store(ctx: Context<InitCommentStore>) -> Result<()> {
        // Extract accounts from the context
        let senddit: &mut Account<Senddit> = &mut ctx.accounts.senddit;
        let treasury: &mut UncheckedAccount = &mut ctx.accounts.treasury;
        let comment_store: &mut Account<CommentStore> = &mut ctx.accounts.comment_store;
        let authority: &mut Signer = &mut ctx.accounts.authority;

        // Pay out fees and reset comment store fields
        payout_fees(treasury, authority, senddit,  None);
        comment_store.comments = 0;
        comment_store.bump = *ctx.bumps.get("comment_store").unwrap();

        Ok(())
    }

    // Post a comment
    pub fn post_comment(ctx: Context<PostComment>, text: String) -> Result<()> {
        // Extract accounts from the context
        let senddit: &mut Account<Senddit> = &mut ctx.accounts.senddit;
        let treasury: &mut UncheckedAccount = &mut ctx.accounts.treasury;
        let comment_store: &mut Account<CommentStore> = &mut ctx.accounts.comment_store;
        let comment: &mut Account<Comment> = &mut ctx.accounts.comment;
        let authority: &mut Signer = &mut ctx.accounts.authority;
        let commenter_wallet: &mut UncheckedAccount = &mut ctx.accounts.commenter_wallet;

        // Pay out fees to the commenter and the platform
        payout_fees(commenter_wallet, authority, senddit, Some(treasury));

        // Check if the comment text is valid
        if text.len() == 0 {
            return Err(ErrorCode::NoTextSubmitted.into());
        }
        if text.len() > 192 {
            return Err(ErrorCode::CommentTooLarge.into());
        }

        // Increment comment count and set comment fields
        comment_store.comments = comment_store.comments.checked_add(1).ok_or(ErrorCode::OverflowUnderflow)?;
        comment.authority = authority.key();
        comment.text = text;
        comment.upvotes = 1;
        comment.bump = *ctx.bumps.get("comment").unwrap();

        Ok(())
    }

    // Upvote a comment
    pub fn upvote_comments(ctx: Context<UpvoteComments>, _number: String) -> Result<()> {
        // Extract accounts from the context
        let senddit: &mut Account<Senddit> = &mut ctx.accounts.senddit;
        let treasury: &mut UncheckedAccount = &mut ctx.accounts.treasury;
        let comment: &mut Account<Comment> = &mut ctx.accounts.comment;
        let authority: &mut Signer = &mut ctx.accounts.authority;
        let commenter_wallet: &mut UncheckedAccount = &mut ctx.accounts.commenter_wallet;

        // Pay out fees to the voter and the platform
        payout_fees(commenter_wallet, authority, senddit, Some(treasury));

        // Increment upvotes count of the comment
        comment.upvotes = comment.upvotes.checked_add(1).ok_or(ErrorCode::OverflowUnderflow)?;

        Ok(())
    }
}

// Utility function to pay out fees
pub fn payout_fees<'info>(
    to: &mut UncheckedAccount<'info>,
    from: &mut Signer<'info>,
    senddit: &mut Account<'info, Senddit>,
    treasury: Option<&mut UncheckedAccount<'info>>
) {
    // First transfer money to the platform
    if let Some(treasury) = treasury {
        invoke(
            &transfer(from.key, treasury.key, senddit.fee),
            &[from.to_account_info(), treasury.to_account_info()],
        )
        .unwrap();
    }
    // Then transfer money to the user
    invoke(
        &transfer(from.key, to.key, senddit.fee),
        &[from.to_account_info(), to.to_account_info()],
    )
    .unwrap();
}

// Define account structures and their lengths
#[account]
pub struct Senddit {
    pub authority: Pubkey,
    pub treasury: Pubkey,
    pub fee: u64,
    pub bump: u8
}
impl Senddit {
    pub const LEN: usize = DISCRIMINATOR + PUBKEY + PUBKEY + UNSIGNED_64 + BUMP;
}

#[account]
pub struct PostStore {
    pub authority: Pubkey,
    pub posts: u128,
    pub bump: u8
}
impl PostStore {
    pub const LEN: usize = DISCRIMINATOR + PUBKEY + UNSIGNED_128 + BUMP;
}

#[account]
pub struct Post {
    pub authority: Pubkey,
    pub link: String,
    pub upvotes: u64,
    pub comments: u64,
    pub bump: u8
}
impl Post {
    pub const LEN: usize = DISCRIMINATOR + PUBKEY + STRING_PREFIX + MAX_LINK_SIZE + UNSIGNED_64 + UNSIGNED_64 + BUMP;
}

#[account]
pub struct CommentStore {
    pub authority: Pubkey,
    pub comments: u128,
    pub bump: u8
}
impl CommentStore {
    pub const LEN: usize = DISCRIMINATOR + PUBKEY + UNSIGNED_128 + BUMP;
}

#[account]
pub struct Comment {
    pub authority: Pubkey,
    pub text: String,
    pub upvotes: u64,
    pub bump: u8
}
impl Comment {
    pub const LEN: usize = DISCRIMINATOR + PUBKEY + STRING_PREFIX + MAX_COMMENT_SIZE + UNSIGNED_64 + BUMP;
}

// Define error codes
#[error_code]
pub enum ErrorCode {
    LinkAlreadySubmitted,
    OverflowUnderflow,
    NoTextSubmitted,
    LinkTooLarge,
    CommentTooLarge
}
