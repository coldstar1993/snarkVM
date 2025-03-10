# This script will run the PoSW SNARK setup and move the resulting `.params`
# and `.checksum` files to `params` folder under the `src` directory.
# If the parameter size has changed, you will need to manually update these in each corresponding struct.

RUST_BACKTRACE=1 cargo run --release --example setup posw testnet1 || exit

mv posw.metadata ../../src/testnet1/resources
mv posw.proving* ../../src/testnet1/resources
mv posw.verifying ../../src/testnet1/resources
