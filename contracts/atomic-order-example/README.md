# Atomic Orders example

This is an example of a smart contract interacting with Injective Chain by using the new atomic
mode of execution for market orders in Injective Exchange. A common request from new applications
built on Injective chain was for the ability to be notified upon the execution of an order.

Previously, because orders placed (from the handler) were matched only at the Endblocker,
smart contracts did not have the ability to use market orders as an atomic primitive for token swaps -
a critical functionality for many business workflows.

This example can also be used to understand how smart contracts can interact with Injective Chain. It assumes a basic
knowledge of Cosmwasm programming (a great starting point will be reading this tutorial: https://book.cosmwasm.com/, and Injective
docs: https://docs.injective.network/develop/guides/cosmwasm-dapps/)

### Functionality

User can trigger the contract by sending swap_spot message with max price and quantity. They need
to include funds for exchange assuming worst agreed price (including expected fees). Contract will
perform swap on their behalf (using atomic order execution) and will send back newly obtained coins
and leftover funds.


## Interacting with Injective Chain

### Messages

Typically Cosmwasm contract execute method returns a response of type Result<Response, ContractError> - if you check the source you
can notice it's defined as  Response<T = Empty>. Type T is used for Custom enum option, which is used for interaction with chain specific
functionality, and by default is set to Empty placeholder.
To send messages to Injective chain (for example create orders directly from your smartcontract), you need:
 - import injective-cosmwasm crate (check https://github.com/InjectiveLabs/cw-injective repo for the current version)
 - change response type of your execute method to Result<Response<InjectiveMsgWrapper>, ContractError>
 - add Injective submessage to response. They should be created as Custom variant of CosmosMsg enum with field of InjectiveMsgWrapper type,
but to make things easier injective-cosmwasm contains helper functions for all supported messages (for example create_spot_market_order_msg()) - check
file msg.rs for the full list of possible messages.

### Handling replies

By default Cosmos contracts either return just some metadata (like events) or "fire and forget" style messages (that will either succeed or fail, in the latter case
roll-backing the whole contract execution). It is possible though to request that after executing a submessage our contract is notified (and will get a response from that
submessage). This swap contract is a good example of a contract where it is necessary - it's impossible to predict what will be the exact result of atomic order execution,
yet the contract must know it to send back correct amounts of funds (swapped tokens and leftover that couldn't be exchanged) back to a requester.
To do so initial execution function must finish with "replyOnSuccess" submessage (atomic market order).
ReplyOnSuccess submessage means that it will call reply method but only if submessage succeeded (otherwise it will rollback everything). Alternatively contract may request
to be notified only on failure (for example if we don't need to rollback the whole transaction in such case) or in both cases.

Handling replies is relatively complex. It's important to remember that due to reentrancy protection, this reply will be handled by a new instance of a contract,
so no local variables will be preserved. If a contract needs to access the prior state it should persist it temporarily between calls.

The reply flow looks as follows:
 - contract persists any state it may need to access in reply handler. As the contracts are executed sequentially, not in parallel, we can assume that the next reply
handler that will be called will be in response for our submessage. This allows us to cache any state easily, but we should make sure to clear any cached state after
processing is finished, and reject processing of main execute method if there's any cached state (to prevent attacks similar to reentrancy attacks)
 - when adding submessages to response that need to be handled in reply handler we must assign them some numerical identifier (usually identifier is used to specify a type
of message)
 - add another entry point called reply. Handling reply messages is more complicated than execute messages, as instead of getting specific, deserialised type of message,
we just get message type ID (set in prior step) and binary blob containing response from other contract or chain
 - deserialize binary response in reply handler (in case of injective chain it's encoded with protobuf) - unfortunately due to size constraints cosmwassm doesn't include
standarized, automated way to deserialize response messages, so it's necessary to do it manually in code.


## About atomic orders execution

Smart contracts now have the ability to execute atomic market orders by sending
a MsgCreate{Spot,Derivative,BinaryOptions}MarketOrder with an order type of {Buy,Sell}_Atomic
which immediately executes the market order against the resting orderbook.

As atomic execution gives a caller a privileged position on the market, it is balanced by higher
trading fees. Each market has a defined atomicFeeMultiplier, which is applied to market's taker fee,
to calculate the fee to be used for this transaction.

Importantly, any fee discounts the trader might have will still apply, and contract may designate
itself as a fees recipient.

## Using atomic orders execution in smart contract

To execute order as atomic, use order type BUY_ATOMIC (9) or SELL_ATOMIC (10). This is supported
only for market orders, on all market types.

Order will be matched and executed immediately, without waiting for end-blocker, against existing
resting orders.

MsgCreateSpotMarketOrderResponse has now an optional field "results", which is a struct consisting
of three fields: Price, Quantity and Fee and describe order execution result.

MsgCreateDerivativeMarketOrderResponse also has an optional field "results", which is a struct
consisting of five fields: Price, Quantity, Fee, Payout and PositionDelta struct.


