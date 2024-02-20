const fs = require('fs');
const { Connection, PublicKey, Transaction, Keypair, SystemProgram, sendAndConfirmTransaction } = require('@solana/web3.js');

const rpcUrl = 'https://api.devnet.solana.com'; // Devnet RPC URL'si
const walletFile = "wallet.json";

async function createWallet() {
    const connection = new Connection(rpcUrl);
    //const payerAccount = Keypair.generate();

    // Creating a new account keypair
    const walletAccount = Keypair.generate();

    // Airdrop Sol to the payer account
    await connection.requestAirdrop(walletAccount.publicKey, 5000000000); // 1 SOL

    // Save the wallet account to a file
    const walletAccountInfo = {
        publicKey: walletAccount.publicKey.toBase58(),
        privateKey: walletAccount.secretKey.toString(),
        balance: 1
    };
    fs.writeFileSync(walletFile, JSON.stringify(walletAccountInfo));
    console.log( `Wallet created and saved to ${walletFile}. Airdropped 1 SOL to wallet ${walletAccount.publicKey.toBase58()}.`);
}

async function getBalance() {
    const walletAccountInfo = JSON.parse(fs.readFileSync(walletFile, 'utf8'));

    const connection = new Connection(rpcUrl);
    const publicKey = new PublicKey(walletAccountInfo.publicKey);
    const balance = await connection.getBalance(publicKey);

    console.log(`Balance of wallet ${walletAccountInfo.publicKey} is ${balance} SOL.`);
}

async function transfer(destinationPublicKey, amount) {
    const walletAccountInfo = JSON.parse(fs.readFileSync(walletFile, 'utf8'));
    console.log("aaa")
    const connection = new Connection(rpcUrl);
    console.log("bbb")
    const privateKeyArray = JSON.parse(walletAccountInfo.privateKey);
    console.log("ccc")

    
    const wallet = Keypair.fromSecretKey(Uint8Array.from(privateKeyArray));
    const destination = new PublicKey(destinationPublicKey);
    
    const tx = new Transaction().add(
        SystemProgram.transfer({
            fromPubkey: wallet.publicKey,
            toPubkey: destination,
            lamports: amount, 
        })
    );

    await sendAndConfirmTransaction(connection, tx, [wallet]);
    console.log(`Sent ${amount} SOL to ${destination.toBase58()}.`);
}

async function main() {

    const args = process.argv.slice(2);
    if (args.length === 0) {
        console.log("Please provide a command: new, airdrop, balance, transfer");
        return;
    }

    const command = args[0];
    
    switch (command) {
        case "new":
            await createWallet();
            break;
        case "airdrop":
            const amount = args.length > 1 ? parseInt(args[1]) : 1;
            await createWallet(amount);
            break;
        case "balance":
            await getBalance();
            break;
        case "transfer":
            if (args.length < 3) {
                console.log('Usage: transfer [destinationPublicKey] [amount]');
                return;
            }
            const destination = args[1];
            const transferAmount = parseInt(args[2]);
            await transfer(destination, transferAmount);
            break;
        default:
            console.log("Invalid command");

    }
}
main().catch(err => console.error(err));
