# Vesting Contract
This contract implements a vesting mechanism for distributing tokens to stakeholders over a specified period of time. It allows organizations to create stakeholders, set vesting periods, and whitelist addresses for token claims.

## Contract Functions
`createOrganization(string memory _name, address _organizationAddress, uint256 _token) external`
This function is used to create an organization and specify the initial token supply. The organization is associated with the provided `_organizationAddress`, and its name is set to _name. The _token parameter represents the initial token amount.

`newStakeholder(address _stakeholderAddress, string memory _position, uint256 _vestingPeriod, uint256 _token) external`
Organizations can use this function to add a new stakeholder. The `_stakeholderAddress` parameter represents the address of the stakeholder being added. The `_position` parameter is a string indicating the stakeholder's position. The `_vestingPeriod` parameter defines the duration, in seconds, over which the tokens will be vested. Finally, the `_token` parameter represents the amount of tokens allocated to the stakeholder.

`whitelistAddress(address _stakeholder) external`
This function is used by organization owners to whitelist a stakeholder address, enabling them to claim tokens. Only whitelisted addresses can call the `claimToken()` function.

`claimToken() external`
Whitelisted stakeholders can call this function to claim their vested tokens. The function checks if the vesting period has elapsed since the stakeholder's start time. If the conditions are met, the stakeholder can claim the tokens they are eligible for. The claimed tokens are added to the stakeholder's balance.

`getClaimedToken() external view returns (uint256)`
This function allows stakeholders to retrieve the amount of tokens they have already claimed.

`getStakeholderPosition(address _address) external view returns (Stakeholder memory)`
Stakeholders can use this function to retrieve their own stakeholder information, including address, position, vesting period, start time, token amount, and claimed token amount.

## Data Structures
### struct Organization`
* `organizationAddress`: The address associated with the organization.
* `name:` The name of the organization.
* `tokenAmount:` The total amount of tokens held by the organization.

### struct Stakeholder
* `stakeholderAddress`: The address associated with the stakeholder.
* `stakeholderPosition:` The position or role of the stakeholder.
* `vestingPeriod:` The duration, in seconds, over which the tokens will be vested.
* `startTime:` The timestamp when the stakeholder's vesting period starts.
* `tokenAmount:` The total amount of tokens allocated to the stakeholder.
* `claimedToken:` The amount of tokens claimed by the stakeholder so far.

## Mapping Variables
* `stakeholders:` Maps stakeholder addresses to their respective stakeholder struct.
* `whitelistedAddresses:` Maps addresses of whitelisted stakeholders.
* `organizations:` Maps organization addresses to their respective organization struct.
* `balances:` Maps stakeholder addresses to the amount of tokens they have claimed.

### Events
* `NewStakeholder(uint256 startTime, uint256 vestingPeriod):` Emits an event when a new stakeholder is added. Provides the start time and vesting period as event data.

* `Whitelisted(uint256 time, address stakeholder):` Emits an event when a stakeholder address is whitelisted. Provides the timestamp and whitelisted stakeholder address as event data.# Sample Hardhat Project






### How to run this project

```shell
npx hardhat help
npx hardhat test
REPORT_GAS=true npx hardhat test
npx hardhat node
npx hardhat run scripts/deploy.js
```
