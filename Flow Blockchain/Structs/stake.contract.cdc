    pub contract Staking {

// A dictionary of assets, where the keys are addresses and the values are of type Asset.
    pub var assets: {Address: Asset}

// Initialize the assets dictionary as an empty dictionary.
    init() {
        self.assets = {}
        }

    pub struct Asset  {
        pub var account: Address
        pub var createdDate: UInt
        pub var unlockDate: UInt
        pub var percent: UInt 
        pub var amountStaked: UInt
        pub var amountInterest: UInt 
        pub var isOpen: Bool
    
// assign the details to the provided field
    init(_account: Address, _createdDate: UInt,  _unlockDate: UInt, _percent: UInt, 
            _amountStaked: UInt, _amountInterest: UInt, _isOpen: Bool ) {
        self.account = _account
        self.createdDate = _createdDate
        self.unlockDate = _unlockDate
        self.percent = _percent
        self.amountStaked = _amountStaked
        self.amountInterest = _amountInterest
        self.isOpen = _isOpen
    }
}

    pub fun stakeToken (account: Address, createdDate: UInt,  unlockDate: UInt, percent: UInt, 
                        amountStaked: UInt, amountInterest: UInt) {
       // var interest = calculateInterest(amountStaked, amountInterest)
        let newStakedAsset = 
                        Asset( _account: account, _createdDate: createdDate,  _unlockDate:unlockDate, _percent: percent, 
                         _amountStaked: amountStaked, _amountInterest: amountInterest, _isOpen: true)
        self.assets[account] =  newStakedAsset

    }
// pub fun calculateInterest (apy: UInt, amount:  UInt): UInt {
// let interest = apy * amount / 10000
// return interest
// }
    }