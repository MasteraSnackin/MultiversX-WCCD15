#![no_std]

// Import necessary modules and macros from the elrond_wasm library.
elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait WoodMinting {
    /// Initialize the contract. This function is called once when the contract is deployed.
    #[init]
    fn init(&self) {}

    /// Endpoint to allow users to stake "WINTER" tokens.
    /// The tokens will be used to calculate minting rewards.
    #[payable("WINTER")]
    #[endpoint(stakeTokens)]
    fn stake_tokens(&self) -> SCResult<()> {
        // Get the caller's address.
        let caller = self.blockchain().get_caller();
        // Get the amount of "WINTER" tokens sent with the call.
        let payment = self.call_value().egld_value();
        // Retrieve the current staked amount for the caller.
        let current_stake = self.stake(&caller).get();
        // Update the caller's stake with the new amount.
        self.stake(&caller).set(&(&current_stake + &payment));
        Ok(())
    }

    /// Endpoint to mint WOOD tokens based on the staked "WINTER" tokens.
    #[endpoint(mintWood)]
    fn mint_wood(&self) -> SCResult<BigUint> {
        // Get the caller's address.
        let caller = self.blockchain().get_caller();
        // Retrieve the amount of "WINTER" tokens staked by the caller.
        let staked_amount = self.stake(&caller).get();
        // Ensure the caller has at least 1000 "WINTER" tokens staked.
        require!(staked_amount >= 1000u64.into(), "Not enough tokens staked");

        // Get the current blockchain round.
        let current_round = self.blockchain().get_block_round();
        // Retrieve the last round when WOOD was minted for the caller.
        let last_mint_round = self.last_mint_round(&caller).get();

        // Calculate the number of rounds since the last minting.
        let rounds_since_last_mint = current_round - last_mint_round;
        // Define the minting rate: 1 WOOD per 600 rounds.
        let rate = 600;
        // Calculate the number of WOOD tokens to mint.
        let num_tokens = (staked_amount / 1000u64) * (rounds_since_last_mint / rate);

        // If tokens can be minted, update the last mint round.
        if num_tokens > 0 {
            self.last_mint_round(&caller).set(current_round);
        }

        // Return the number of WOOD tokens minted.
        Ok(num_tokens)
    }

    /// Storage mapper to keep track of the staked "WINTER" tokens for each user.
    #[view(getStake)]
    #[storage_mapper("stake")]
    fn stake(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    /// Storage mapper to keep track of the last round WOOD tokens were minted for each user.
    #[view(getLastMintRound)]
    #[storage_mapper("lastMintRound")]
    fn last_mint_round(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;
}