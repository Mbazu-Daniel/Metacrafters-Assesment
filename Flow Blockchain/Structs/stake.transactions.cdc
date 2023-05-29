import Staking from 0xf8d6e0586b0a20c7

transaction(account: Address, createdDate: UInt, unlockDate: UInt, percent: UInt, 
            amountStaked: UInt, amountInterest: UInt) {

    prepare(signer: AuthAccount) {}

    execute {
        Staking.stakeToken(account: account, createdDate: createdDate,  unlockDate:unlockDate, percent: percent, 
                         amountStaked: amountStaked, amountInterest: amountInterest)
        log("Staked Succefully")
    }
}
