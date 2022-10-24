[![Rust](https://github.com/Angry-Penguins-Colony/sc-customize-nft/actions/workflows/rust.yml/badge.svg)](https://github.com/Angry-Penguins-Colony/sc-customize-nft/actions/workflows/rust.yml)

[![codecov](https://codecov.io/gh/Angry-Penguins-Colony/sc-customize-nft/branch/master/graph/badge.svg?token=H5FTEWR9AI)](https://codecov.io/gh/Angry-Penguins-Colony/sc-customize-nft)

# 📜 Smart Contract

## Description

The customization smart contract of Angry Penguins Colony's customization system. 

# How to set up the contract?

## Build contract

```
erdpy contract build
```

## Deploy contract

There are three arguments to deploy the smart contract : 

| Argument                                | Explication                                                  |
| --------------------------------------- | ------------------------------------------------------------ |
| Collection identifier of the Equippable | For example, the Angry Penguins Colony is `APC-928458`.      |
| IPFS Gateway                            | If you don't know what to put, https://ipfs.io/ipfs/ is recommended.<br />It is worth specifying another gateway if your visuals are hosted on services like Pinata.cloud. |
| Equippable name format                  | This is the name of the Equippable.<br /><br />{number} must be included. It will be replaced the number (not the nonce) of the penguin<br />E.g. `Penguin #{number}` will become `Penguin #10` if the Equippable is the 10st |


Fill erdpy.json, then run:
```
erdpy contract deploy
```


## Register an item

To equip an Item, you must register it.

### Register the collection to a slot

First, register the collection of your item to a slot.

```rust
TransferTransaction {
    Sender: <account address of the sender>
    Receiver: <smart contract address>
    Value: 0
    GasLimit: 6_000_000
    Data: "registerItem" +
            "@" + <slot in hexadecimal encoding>
            "@" + <collection identifier in hexadecimal neconding>
}
```

> **EXAMPLE**
>
> Register the collection HAT-5e78d4 as a hat.
> ```rust
> TransferTransaction {
>     Sender: <account address of the sender>
>     Receiver: <smart contract address>
>     Value: 0
>     GasLimit: 6_000_000
>     Data: "registerItem" +
>        "@686174" + // hat
>        "@4841542D356537386434" // HAT-5e78d4
> }
> ```

### Fill each item

The smart contract needs one SFT of each item to read its attributes.  
So, we must send each.

```rust
TransferTransaction {
    Sender: <account address of the sender>
    Receiver: <same as sender>
    Value: 0
    GasLimit: 6_000_000
    Data: "ESDTNFTTransfer" +
            "@" + <token identifier in hexadecimal encoding> +
            "@" + <the nonce in hexadecimal encoding> +
            "@01" + // quantity to send; always one
            "@" + <smart contract address in hexadecimal encoding> +
            "@66696C6C" + // name of method to call; this is "fill"
}
```

## Transfer required role

On Elrond, we cannot update the URI associated with an NFT (we can just add a new URI).   
To update the visual, we burn and recreate a new NFT, with the wanted visual.

> The total supply of your collection will stay the same.

Therefore, we need to **transfer the creation role** of the Equippable to the smart contract:
```rust
TransferCreationRoleTransaction {
    Sender: <address of the current creation role owner>
    Receiver: erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u // metachain
    Value: 0
    GasLimit: 60_000_000 + length of Data field in bytes * 1500
    Data: "transferNFTCreateRole" +
            "@" + <token identifier in hexadecimal encoding> +
            "@" + <address of the current creation role owner in hexadecimal encoding> +
            "@" + <smart contract address in hexadecimal encoding>
}
```

We also need to authorize the smart contract to **burn** the Equippable:
```rust
AssigningBurnRoleTransaction {
    Sender: <address of the ESDT manager>
    Receiver: erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u
    Value: 0
    GasLimit: 60_000_000
    Data: "setSpecialRole" +
            "@" + <equippable token identifier in hexadecimal encoding> +
            "@" + <smart contract address in a hexadecimal encoding> +
            "@45534454526F6C654E46544275726E20" + // ESDTRoleNFTBurn 
}
```

## Enqueue image to render 

This endpoint makes the `server-push-renderer`, render the Equippable.

```rust
TransferTransaction {
    Sender: <account address of the sender>
    Receiver: <smart contract>
    Value: 1_000_000_000_000_000, // 0.001 EGLD
    GasLimit: 50_000_000
    Data: "renderImage" +
            "@" + <equippable attributes in hexadecimal encoding>
}
```

> **EXAMPLE**
>
> Render the image of an equippable that has "Pirate Hat" as hat slot.
> 
> ```rust
> TransferTransaction {
>     Sender: <account address of the sender>
>     Receiver: <smart contract>
>     Value: 1_000_000_000_000_000, // 0.001 EGLD
>     GasLimit: 50_000_000
>     Data: "renderImage" +
>        "@4861743A50697261746520486174" + // Hat:Pirate Hat
> }
> ```

> **💡 WHY THIS TRANSACTION MUST BE PAYED?**
> 
> For the **autonomy** of the system.  
> 
> The server push renderer listen to `renderImage` transactions, and responds by sending a `setCidOf` transaction.   
> 
> But, if the server has not funds, he will not be able to send the transaction `setCidOf`. All the system would be frozen.  
> Hopefully, before that happen, the server will claim the wallet of the smart contract. 

# How to customize?
## Equip an Equippable

Transfer the Equippable NFT and the Items SFT to the smart contract while calling the endpoint to `customize`.

> **⚠️ WARNING**
>
> The final Equippable must have a CID associated. (The association is made with the `renderImage` transaction and the `server-push-renderer`).   
> Otherwise, the smart contract can't know which URI set on the NFT.

```rust
TransferTransaction {
    Sender: <account address of the sender>
    Receiver: <same as sender>
    Value: 0
    GasLimit: 20_000_000
    Data: "MultiESDTNFTTransfer" +
            "@" + <receiver bytes in hexadecimal encoding>
            "@02" + <number of tokens to transfer in hexadecimal encoding> +
            "@" + <equippable token identifier in hex encoding> +
            "@" + <equippable to customize nonce in hex encoding> +
            "@01" + // quantity of equippable to send; we just send one
            "@" + <item to equip 1 identifier in hexadecimal encoding> +
            "@" + <item to equip 1 nonce in hexadecimal encoding> +
            "@01" + // quantity of item to equip 1; we just send one
            <...> + // item to equip can be repeated
            "@customize"
}
```

> **💡 WHAT IF I SEND AN ITEM ON OCCUPIED SLOT ?**
>
> Don't worry. This case is handled by the smart contract:  
> 
> The item on the occupied slot will be unequipped and sent back to the user. While the new item will be equipped to the slot.


## Unequip an Equippable

Call the `customize` endpoint while transferring the Equippable NFT. Then, add the slots you want to unequip after `customize` in the data field.

> **EXAMPLE**
>
> Unequip slot "hat" and "beak" of address erd1swns5yj0vlj05pxx79qafp582mww74lzcwd499lay3z4t3x3sdxsup9fjm
> ```rust
> TransferTransaction {
>     Sender: <account address of the sender>
>     Receiver: <same as sender>
>     Value: 0
>     GasLimit: 20_000_000
>     Data: "MultiESDTNFTTransfer" +
>        "@83a70a124f67e4fa04c6f141d4868756dcef57e2c39b5297fd244555c4d1834d" + // the address
>        "@01" + // only one token sent (the Equippable)
>        "@50454E4755494E2D613161316131" + // PENGUIN-a1a1a1
>        "@05" + // nonce 05
>        "@01" + // quantity of equippable to send
>        "@customize" +
>        "@686174" + // hat
>        "@6265616B" // beak
> }
> ```

## Equip and unequip an Equippable in the same transaction

```rust
TransferTransaction {
    Sender: <account address of the sender>
    Receiver: <same as sender>
    Value: 0
    GasLimit: 20_000_000
    Data: "MultiESDTNFTTransfer" +
        "@" + <receiver bytes in hexadecimal encoding>
        "@" + <number of tokens to transfer in hexadecimal encoding> +
        "@" + <equippable token identifier in hex encoding> +
        "@" + <equippable to customize nonce in hex encoding> +
        "@01" + // quantity of penguin to send
        "@" + <item to equip 1 identifier in hexadecimal encoding> +
        "@" + <item to equip 1 nonce in hexadecimal encoding> +
        "@01" + // just send one item to equip 1
        <...> // item to equip can be repeated
        "@customize" +
        "@" + <slot to desequip in hex encoding>
}
```

# Miscellaneous

To run tests, run :

```
cargo test -p customize_nft --test lib
```

> The units and integrations tests are written with the Rust testing framework. So `erdpy contract test` will not work. 

## What can be improved ?

- [ ] (optimization) RETAKE the sorting in EquippableNftAttributes to be more efficient. For the moment, we sort the entire array each time.
    - could we sort only in top_decode and top_encode ?
    - in `set_item`, could we insert the new item in the right index