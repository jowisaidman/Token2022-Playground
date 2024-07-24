import {
    publicKey,
    PublicKey,
    signerIdentity, 
    signerPayer,
    createSignerFromKeypair,
} from '@metaplex-foundation/umi'
import {
    mintV1,
    TokenStandard,
} from '@metaplex-foundation/mpl-token-metadata'
import { Keypair } from '@solana/web3.js';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { fromWeb3JsKeypair } from '@metaplex-foundation/umi-web3js-adapters';
import { mplCandyMachine } from '@metaplex-foundation/mpl-candy-machine';
import { findAssociatedTokenPda } from '@metaplex-foundation/mpl-toolbox';
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

    const tokenAddress = process.argv[2];

    if (!tokenAddress) {
        throw new Error('Missing parameter: token2022 address is required');
    }

    const token2022 = publicKey(tokenAddress); // Replace with the token you created 

    const token = findAssociatedTokenPda(umi, {
        mint: token2022,
        owner: umi.identity.publicKey,
        tokenProgramId: SPL_TOKEN_2022_PROGRAM_ID,
    })
    
    const result = await mintV1(umi, {
        mint: token2022,
        token,
        authority: signer,
        amount: 100,
        tokenOwner: signer.publicKey,
        splTokenProgram: SPL_TOKEN_2022_PROGRAM_ID,
        tokenStandard: TokenStandard.NonFungible,
    }).sendAndConfirm(umi)

    console.log("Transaction sended: ", result);
}

main().catch(console.error);