# ckb-type-id

ckb has a built-in type id [contract](https://github.com/nervosnetwork/ckb/blob/develop/script/src/type_id.rs), which is used to constrain the behavior of id generation.

However, it is a standalone contract and will take the place of cell type script, making it impossible to use other contracts.

This library writes the validation logic of type id as a library that can be embedded in any contract, allowing developers to embed the validation logic of type id in their own contracts, and at the same time, it is relatively small in consumption.

The corresponding c version of the library is [link](https://github.com/nervosnetwork/ckb-c-stdlib/blob/master/ckb_type_id.h)

## Usage

You can see sample contracts and examples of contract construction in `test-lib`
