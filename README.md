
---

# Message Program

This is a small Solana program that stores a message on-chain.
It lets you:

* create a message account with a default message
* update the message later
* read the account data

This project uses **Anchor**.

---

## set up environment

```
 PATH="/home/codespace/.local/share/solana/install/active_release/bin:$PATH" or based local path
```
## 1. Install Dependencies

Make sure you have:

* Rust
* Solana CLI
* Anchor

If not installed:

```
solana --version
anchor --version
```

---

## 2. Set the Solana Cluster to Localnet

```
solana config set --url http://127.0.0.1:8899
```

---

## 3. Build the Program

```
anchor build
```

This creates the IDL in:

```
target/idl/message_program.json
```

And the program keypair in:

```
target/deploy/message_program-keypair.json
```

---

## 4. Start a Local Validator

You can use:

```
solana-test-validator
```

Or let Anchor handle it with tests.

---

## 5. Deploy the Program

Run:

```
anchor deploy
```

After deployment, Anchor will print the program ID.

---

## 6. Run Tests

The tests create a message account, update it, and show the stored data.

Run:

```
anchor test
```

Anchor will start a local validator, deploy the program, and run the test scripts.

---

## 7. How to Interact With the Program (Manually)

### Initialize

This creates the account and stores the default message:

```
await program.methods
  .initialize()
  .accounts({
    messageAccount,
    authority,
    systemProgram,
  })
  .rpc();
```

### Update Message

```
await program.methods
  .updateMessage("hello world")
  .accounts({
    messageAccount,
    authority,
  })
  .rpc();
```

### Fetch Message Account

```
const data = await program.account.messageAccount.fetch(messageAccount);
console.log(data);
```

---

## Folder Structure

```
programs/
  message_program/   → Rust program
tests/               → TypeScript tests
Anchor.toml
```

---

## Notes

* The program ID in `declare_id!` must match the one in `Anchor.toml`.
* Make sure the local validator is running before using CLI commands.

---

