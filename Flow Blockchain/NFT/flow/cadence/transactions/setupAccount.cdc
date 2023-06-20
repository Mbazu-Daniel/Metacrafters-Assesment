import CryptoPals from "../contracts/CryptoPals.cdc";
transaction() {
  prepare(signer: AuthAccount) {
    // Store a `CryptoPals.Collection` in our account storage.
    signer.save(<- CryptoPals.createEmptyCollection(), to: /storage/CryptoPals)

    // Link it to the public.
    signer.link<&CryptoPals.Collection>(/public/CryptoPals, target: /storage/CryptoPals)
  }
}