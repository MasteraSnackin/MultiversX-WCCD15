Initialization: Each contract has an init function, which is called once when the contract is deployed. It typically sets up initial state but is left empty here as no initialization is needed.

Staking Tokens: The stake_tokens endpoint allows users to stake "WINTER" tokens. The amount staked is stored in the contract's state using a storage mapper.

Minting Tokens:

The mint_* endpoints calculate how many tokens (WOOD, FOOD, STONE, GOLD) can be minted based on the amount of "WINTER" tokens staked and the rounds passed since the last mint.
Each contract has a different minting rate, defined by the number of rounds required to mint one token.
State Management:

stake: A storage mapper that tracks the amount of "WINTER" tokens each user has staked.
lastMintRound: A storage mapper that stores the last round when the user minted tokens, ensuring that new tokens can only be minted after the required rounds have passed.
Usage: Users can stake tokens and then periodically call the minting functions to receive their resource tokens.