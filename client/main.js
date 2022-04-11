const solana = require("@solana/web3.js");
const utils = require("./utils");
const path = require("path");

const PROGRAM_PATH = path.resolve(__dirname, "../../target/deploy/");
const PROGRAM_LIB = path.join(PROGRAM_PATH, "midware_transfer.so");
const PROGRAM_KEY = path.join(PROGRAM_PATH, "midware_transfer-keypair.json");

const PAYER_KEY = path.resolve(__dirname, "./payer.keypair");
const BOB_KEY = path.resolve(__dirname, "./bob.keypair");
const ALICE_KEY = path.resolve(__dirname, "./alice.keypair");
const BANK_KEY = path.resolve(__dirname, "./bank.keypair");

const SYSTEM_ACCOUNT = new solana.PublicKey("11111111111111111111111111111111");

let connection;

async function getBlance(key) {
    let balance_lamports = await connection.getBalance(key);
    let balance_sol = balance_lamports / solana.LAMPORTS_PER_SOL;

    return balance_sol + " sol";
}

async function main() {
    // Establish a connection to validator
    connection = new solana.Connection("http://127.0.0.1:8899", 'confirmed');
    {
        console.log("Connection established: ", await connection.getVersion());
    }

    // Get deployed programId
    const PROGRAM_KEYPAIR = await utils.createKeypairFromFile(PROGRAM_KEY);
    const PROGRAM_ID = PROGRAM_KEYPAIR.publicKey;

    // Have a payer account
    const payer = await utils.createKeypairFromFile(PAYER_KEY);
    {
        let airdrop_req = await connection.requestAirdrop(payer.publicKey, 100 * solana.LAMPORTS_PER_SOL);
        connection.confirmTransaction(airdrop_req);
        console.log(`Payer account is ${payer.publicKey}`);
    }

    // Let's initialize an account to store state to
    const vault = solana.Keypair.generate();
    {
        console.log("Vault is ", vault.publicKey.toBase58())
    }


    // Lets first create a bank account
    const [bank, seed] = await solana.PublicKey.findProgramAddress([Buffer.from("derive-this")], PROGRAM_ID);
    const INIT_BANK = [1];
    {
        console.log("");

        console.log(`Bank address with bump ${seed} is ${bank.toBase58()}`);

        console.log("Calling init bank instruction....");
        const init_bank_instruction = new solana.TransactionInstruction({
            keys: [
                { pubkey: SYSTEM_ACCOUNT, isSigner: false, isWriteable: false },
                { pubkey: bank, isSigner: false, isWritable: true },
                { pubkey: payer.publicKey, isSigner: true, isWritable: true },
            ],
            programId: PROGRAM_ID,
            data: INIT_BANK,
        });

        let tx_hash = await solana.sendAndConfirmTransaction(
            connection,
            new solana.Transaction().add(init_bank_instruction),
            [payer],
        );

        console.log("Success with ", tx_hash);
        console.log("");
    }

    // First call an INIT instruction
    const INIT_UPCODE = [0];
    {
        console.log("");
        console.log("Calling init vault instruction....");
        const init_instruction = new solana.TransactionInstruction({
            keys: [
                { pubkey: SYSTEM_ACCOUNT, isSigner: false, isWriteable: false },
                { pubkey: vault.publicKey, isSigner: true, isWritable: true },
                { pubkey: payer.publicKey, isSigner: true, isWritable: false },
            ],
            programId: PROGRAM_ID,
            data: INIT_UPCODE,
        });

        let tx_hash = await solana.sendAndConfirmTransaction(
            connection,
            new solana.Transaction().add(init_instruction),
            [payer, vault],
        );

        console.log("Success with ", tx_hash);
        console.log("");
    }

    // Let get two users
    let alice = await utils.createKeypairFromFile(ALICE_KEY);
    let bob = await utils.createKeypairFromFile(BOB_KEY);
    {
        console.log("");
        let alice_balance = await getBlance(alice.publicKey);
        let bob_balance = await getBlance(bob.publicKey);
        console.log("Alice with balance: ", alice_balance, alice.publicKey.toBase58());
        console.log("Bob with balance : ", bob_balance, bob.publicKey.toBase58());
    }

    // Alice will deposit some funds for bob
    // This byte code is of instruction::Deposit{ amount: 1sol }
    const ALICE_DEPOSIT_INSTRUCTION = [2, 0, 202, 154, 59, 0, 0, 0, 0];
    {
        console.log("");
        console.log("Calling deposit instruction from alice to bob....");
        const deposit_instruction = new solana.TransactionInstruction({
            keys: [
                { pubkey: SYSTEM_ACCOUNT, isSigner: false, isWriteable: false },
                { pubkey: vault.publicKey, isSigner: true, isWritable: true },
                { pubkey: bank, isSigner: false, isWritable: true },
                { pubkey: alice.publicKey, isSigner: true, isWritable: true },
                { pubkey: bob.publicKey, isSigner: false, isWritable: true },
            ],
            programId: PROGRAM_ID,
            data: ALICE_DEPOSIT_INSTRUCTION,
        });

        let tx_hash = await solana.sendAndConfirmTransaction(
            connection,
            new solana.Transaction().add(deposit_instruction),
            [alice, vault],
        );

        console.log("Success with ", tx_hash);
        let alice_balance = await getBlance(alice.publicKey);
        let bob_balance = await getBlance(bob.publicKey);
        let bank_balance = await getBlance(bank);
        console.log(`Balance of Alice: ${alice_balance}. Bob: ${bob_balance}. Bank: ${bank_balance}`);
        console.log("");
    }

    // Now Bob will withdraw the amount
    // This byte code is of instruction::Withdraw{ amount: 0.5sol }
    const BOB_WITHDRAW = [3, 0, 101, 205, 29, 0, 0, 0, 0];
    {
        console.log("");
        console.log("Calling withdraw instruction from bob....");
        const withdraw_instruction = new solana.TransactionInstruction({
            keys: [
                { pubkey: SYSTEM_ACCOUNT, isSigner: false, isWriteable: false },
                { pubkey: vault.publicKey, isSigner: true, isWritable: true },
                { pubkey: bank, isSigner: false, isWritable: true },
                { pubkey: bob.publicKey, isSigner: true, isWritable: true },
            ],
            programId: PROGRAM_ID,
            data: BOB_WITHDRAW,
        });

        let tx_hash = await solana.sendAndConfirmTransaction(
            connection,
            new solana.Transaction().add(withdraw_instruction),
            [bob, vault],
        );

        console.log("Success with ", tx_hash);
        let alice_balance = await getBlance(alice.publicKey);
        let bob_balance = await getBlance(bob.publicKey);
        let bank_balance = await getBlance(bank);
        console.log(`Balance of Alice: ${alice_balance}. Bob: ${bob_balance}. Bank: ${bank_balance}`);
        console.log("");
    }
}



///////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////
main().then(
    () => {
        console.log("Everything went ok...");
        process.exit(0);
    },

    err => {
        console.log(`Exited with error: ${err}`);
        process.exit(-1)
    },
);
