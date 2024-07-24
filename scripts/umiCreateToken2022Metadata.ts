import {
    generateSigner,
    percentAmount,
    publicKey,
    PublicKey,
    signerIdentity, 
    signerPayer,
    createSignerFromKeypair
} from '@metaplex-foundation/umi'
import {
    createV1,
    TokenStandard,
} from '@metaplex-foundation/mpl-token-metadata'
import { Keypair } from '@solana/web3.js';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { fromWeb3JsKeypair } from '@metaplex-foundation/umi-web3js-adapters';
import { mplCandyMachine } from '@metaplex-foundation/mpl-candy-machine';
const fs = require('fs');
  
const SPL_TOKEN_2022_PROGRAM_ID: PublicKey = publicKey(
  'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb'
)
  
async function main() {
    const secret: Uint8Array = JSON.parse(fs.readFileSync("my_wallet.json") as string);
    const kp = Keypair.fromSecretKey(Uint8Array.from(secret), {skipValidation: true});

    const umi = createUmi('https://api.devnet.solana.com').use(mplCandyMachine());
    const signer = createSignerFromKeypair(umi, fromWeb3JsKeypair(kp));
    
    console.log("Token owner: ",signer.publicKey);

    umi.use(signerIdentity(signer));
    umi.use(signerPayer(signer));

    const mint = generateSigner(umi)
    const result = await createV1(umi, {
        mint,
        authority: signer,
        name: 'My Token 2022',
        uri: "https://example.com",
        sellerFeeBasisPoints: percentAmount(1),
        splTokenProgram: SPL_TOKEN_2022_PROGRAM_ID,
        tokenStandard: TokenStandard.Fungible,
    }).sendAndConfirm(umi);

    console.log("Transaction sended: ", result);
}

main().catch(console.error);