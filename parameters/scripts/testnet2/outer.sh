# This script will run the outer SNARK setup and move the resulting `.params`
# and `.checksum` files to `params` folder under the `src` directory.
# If the parameter size has changed, you will need to manually update these in each corresponding struct.

RUST_BACKTRACE=1 cargo run --release --example setup outer testnet2 || exit

mv outer.metadata ../../src/testnet2/resources
mv outer.proving* ../../src/testnet2/resources
mv outer.verifying ../../src/testnet2/resources
