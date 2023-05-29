import Staking from 0xf8d6e0586b0a20c7

pub fun main(account: Address): Staking.Asset {
    return Staking.assets[account]!
}
