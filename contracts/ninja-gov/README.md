# Ninja Governance

**NOTE**: Reference documentation for this contract is available [here](https://docs.ninja.finance/contracts/gov).

The Gov Contract contains logic for holding polls and Ninja Token (NINJA) staking, and allows the Ninja Protocol to be governed by its users in a decentralized manner. After the initial bootstrapping of Ninja Protocol contracts, the Gov Contract is assigned to be the owner of itself and the Ninja Factory.

New proposals for change are submitted as polls, and are voted on by NINJA stakers through the voting procedure. Polls can contain messages that can be executed directly without changing the Ninja Protocol code.

The Gov Contract keeps a balance of NINJA tokens, which it uses to reward stakers with funds it receives from trading fees sent by the Ninja Collector and user deposits from creating new governance polls. This balance is separate from the Community Pool, which is held by the Community contract (owned by the Gov contract).
