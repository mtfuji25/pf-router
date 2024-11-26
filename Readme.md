# Install dependancies
yarn install

# Update key
anchor keys sync

# Build
anchor build

# Deploy
solana program deploy target/deploy/router.so --with-compute-unit-price 100000 --max-sign-attempts 100 -k ./payer.json -u https://cold-hidden-research.solana-mainnet.quiknode.pro/

# Close 
solana program close 72kmmN8NVxrYCYPdAQ9RGuwXrdTpL54ACERtZAubzUf1 --bypass-warning -k ./payer.json -u https://cold-hidden-research.solana-mainnet.quiknode.pro/

solana program close --buffers -k ./payer.json -u https://cold-hidden-research.solana-mainnet.quiknode.pro/