// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

contract CollateralProtection {
    address public owner;
    uint256 public collateralThreshold;

    struct LoanPolicy {
        uint256 amount;
        uint256 threshold;
        uint256 time;
        uint256 repayAmount;
        uint256 wallet;
        bool paid;
    }

    mapping(address => uint256) public loanCollateral;
    mapping(address => LoanPolicy) public loans;

    event LoanCreated(address indexed borrower, uint256 amount, uint256 collateralAmount);
    event CollateralReturned(address indexed borrower, uint256 collateralAmount);

    constructor() {
        owner = tx.origin;
    }

    uint256 private constant BASIC_LOAN = 0.5 ether;
    uint256 private constant PREMIUM_LOAN = 1 ether;
    uint256 private constant BASIC_LOAN_TIME = 30 days;
    uint256 private constant PREMIUM_LOAN_TIME = 60 days;

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can perform this action");
        _;
    }

    function createLoan(uint256 amount, uint256 collateralAmount) external onlyOwner {
        require(amount > 0, "Loan amount must be greater than zero");
        require(collateralAmount > 0, "Collateral amount must be greater than zero");
        require(loans[msg.sender].amount == 0, "Loan already exists");

        if (amount > BASIC_LOAN) {
            loans[owner] = LoanPolicy(
                amount,
                collateralAmount,
                block.timestamp + PREMIUM_LOAN_TIME,
                amount + (amount * 20) / 100,
                0,
                false
            );
        } else if (amount <= BASIC_LOAN) {
            loans[owner] = LoanPolicy(
                amount,
                collateralAmount,
                block.timestamp + BASIC_LOAN_TIME,
                amount + (amount * 10) / 100,
                0,
                false
            );
        }

        loanCollateral[msg.sender] = collateralAmount;
        emit LoanCreated(msg.sender, amount, collateralAmount);
    }

    function collectLoan() external payable {
        require(loanCollateral[owner] != 0, "You do not have collateral");
        require(loans[owner].wallet == 0, "Wallet must be zero");
        require(loans[owner].threshold >= loans[owner].amount, "Your collateral is too low");
        require(!loans[owner].paid, "Loan not disbursed");
        (bool sent, ) = (owner).call{value: loans[owner].amount}("");
        require(sent, "Failed to send Ether");
        loans[owner].wallet += loans[owner].amount;
    }

    function payLoan() external payable {
        require(loans[owner].repayAmount > 0, "No loan available to pay");
        require(msg.value >= loans[owner].repayAmount, "Insufficient amount");
        require(!loans[owner].paid, "Loan not disbursed");
        payable(address(this)).transfer(msg.value);
        loans[owner].repayAmount -= msg.value;
        loanCollateral[msg.sender] = 0;
        emit CollateralReturned(owner, msg.value);
    }

    receive() payable external {}
}
