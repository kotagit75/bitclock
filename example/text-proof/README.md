# Example: Creating a Proof
## 0 Preparation
Before creating a proof, please start BitClock using the following command.
```bash
./target/release/bitclock
```
## 1 Proof
Let’s try creating a proof.
First, run the following command.
```bash
./example/text-proof/proof.sh Hello > key
```
This command created a proof that the string `Hello` exists. Generating the proof takes a little time.
## 2 Retrieving the Proof
We saved the standard output of `./example/text-proof/proof.sh` to the `key` file, but the data stored in the `key` file is not the proof itself, but rather an ID used to identify the proof. Let’s retrieve the proof.
```bash
./target/release/cli find $(cat key) > proof.json
```
This command saved the proof’s contents to `proof.json`. You can verify the contents with `cat proof.json`.
## 3 Verify
Finally, let’s verify that the created proof is correct.
```bash
./target/release/cli verify $(cat proof.json)
```
If `true` is displayed, it was successful! You have successfully created a proof that the string `Hello` existed!
