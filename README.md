# Solana Editions

# Install Dependencies 

### AVM (Anchor Version Manager)

```bash
# repo root
cargo install --git https://github.com/project-serum/anchor avm --locked --force

avm install latest
avm use latest
```

### Verify Anchor Is Working

```bash
# repo root
anchor --version
```


### Install Solita

```bash
# repo root
yarn add -D @metaplex-foundation/solita
```



# Build Flow

```bash
cargo fmt
anchor build
yarn solita
```