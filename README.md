# Token2022-Playground

Inside the scripts folder you will find two scripts, one for creating a non-fungible 2022 token and another to mint them.

### Running

- Install the packages by running: `npm install`
- Create a wallet inside script folder: `solana-keygen new --outfile my_wallet.json`
- Run the script umiCreateToken2022Metadata to create the Token2022: `ts-node umiCreateToken2022Metadata.ts`
- Fetch the token address created from https://explorer.solana.com/?cluster=devnet
- Run the script umiMintToken2022 to mint tokens: `ts-node umiCreateToken2022.ts <token_addres>` (example: `ts-node umiCreateToken2022.ts GHf8bqWAee3KYaVDhdYygqkpujK6wCZjWpDYPN8NX65i`)
