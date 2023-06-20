import CryptoPals from "../contracts/CryptoPals.cdc";
import NonFungibleToken from "../contracts/NonFungibleToken.cdc"

pub fun main(address: Address): Int {
  let publicCollection = getAccount(address).getCapability(/public/CryptoPals)
              .borrow<&CryptoPals.Collection>()
              ?? panic("The address does not have a Collection.")


 let value = publicCollection.getIDs().length
log(value)
 return value


}