# REST API example application

This is a prototype of a Rust SDK providing functionality for interacting with the Newcoin network.

The expected api functionality is exampled via the `main.rs` file.


//** This command will not yet work as rs_ncdao has not *yet* been uploaded to crates.io **//
## Install 

    cargo install rs_ncdao

## Run the app

    cargo run

## Run the tests

    cargo test


## Get Table Rows/With Payload:

```rust
   let gtr_response = get_table_rows().await;
    let data_1 = gtr_response.text().await?;

    let gtr_wp_response = get_proposal_by_id(&payload).await;

    let data_2 = gtr_wp_response.unwrap_err();
    let gtr_wp: Response = get_table_rows_with_payload(&payload).await.unwrap();
```

[@newfound8ion/newcoin-sdk](README.md) / Exports

# @newfound8ion/newcoin-rs-api

## Table of contents

### Namespaces

- [&lt;internal\&gt;](modules/internal_.md)

### Classes

- [NCO\_BlockchainAPI](classes/NCO_BlockchainAPI.md)

### Type Aliases

- [NCApproveDaoProposal](modules.md#ncapprovedaoproposal)
- [NCBuyRam](modules.md#ncbuyram)
- [NCChangeFile](modules.md#ncchangefile)
- [NCCreateCollection](modules.md#nccreatecollection)
- [NCCreateDao](modules.md#nccreatedao)
- [NCCreateDaoProposal](modules.md#nccreatedaoproposal)
- [NCCreateDaoStakeProposal](modules.md#nccreatedaostakeproposal)
- [NCCreateDaoUserWhitelistProposal](modules.md#nccreatedaouserwhitelistproposal)
- [NCCreatePermission](modules.md#nccreatepermission)
- [NCCreatePool](modules.md#nccreatepool)
- [NCCreateUser](modules.md#nccreateuser)
- [NCDaoProposalVote](modules.md#ncdaoproposalvote)
- [NCDaoWithdrawVoteDeposit](modules.md#ncdaowithdrawvotedeposit)
- [NCExecuteDaoProposal](modules.md#ncexecutedaoproposal)
- [NCGetAccInfo](modules.md#ncgetaccinfo)
- [NCGetDaoProposals](modules.md#ncgetdaoproposals)
- [NCGetDaoWhiteList](modules.md#ncgetdaowhitelist)
- [NCGetPoolInfo](modules.md#ncgetpoolinfo)
- [NCGetVotes](modules.md#ncgetvotes)
- [NCKeyPair](modules.md#nckeypair)
- [NCKeyValPair](modules.md#nckeyvalpair)
- [NCLinkPerm](modules.md#nclinkperm)
- [NCMintAsset](modules.md#ncmintasset)
- [NCMintFile](modules.md#ncmintfile)
- [NCMintLink](modules.md#ncmintlink)
- [NCMintProfile](modules.md#ncmintprofile)
- [NCModifyAsset](modules.md#ncmodifyasset)
- [NCNameType](modules.md#ncnametype)
- [NCPoolsInfo](modules.md#ncpoolsinfo)
- [NCReturnInfo](modules.md#ncreturninfo)
- [NCReturnTxs](modules.md#ncreturntxs)
- [NCStakeMainDao](modules.md#ncstakemaindao)
- [NCStakePool](modules.md#ncstakepool)
- [NCTxBal](modules.md#nctxbal)
- [NCTxNcoBal](modules.md#nctxncobal)
- [NCUnstakePool](modules.md#ncunstakepool)

### Variables

- [default\_nft\_schema](modules.md#default_nft_schema)
- [devnet\_services](modules.md#devnet_services)
- [devnet\_urls](modules.md#devnet_urls)
- [file\_schema](modules.md#file_schema)
- [link\_schema](modules.md#link_schema)
- [profile\_schema](modules.md#profile_schema)

## Type Aliases

### NCApproveDaoProposal

Ƭ **NCApproveDaoProposal**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `approver` | `String` |
| `approver_prv_key` | `String` |
| `dao_id` | `u32` |
| `dao_owner` | `String` |
| `proposal_author` | `String` |
| `proposal_id` | `u32` |

#### Defined in

[types.ts:185](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L185)

___

### NCBuyRam

Ƭ **NCBuyRam**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `payer` | `String` |
| `payer_prv_key` | `String` |
| `ram_amt` | `u32` |
| `user` | `String` |

#### Defined in

[types.ts:16](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L16)

___

### NCChangeFile

Ƭ **NCChangeFile**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `asset_id` | `String` |
| `editor` | `String` |
| `new_content` | `String` |
| `new_image` | `String` |
| `new_name` | `String` |
| `new_path` | `String` |
| `owner` | `String` |
| `payer` | `String` |
| `payer_prv_key` | `String` |

#### Defined in

[types.ts:380](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L380)

___

### NCCreateCollection

Ƭ **NCCreateCollection**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `allow_notify` | `bool` |
| `burnable` | `bool` |
| `collection_name` | `String` |
| `max_supply` | `u32` |
| `mkt_fee` | `u32` |
| `schema_fields` | [`Vec<NCNameType>`](modules.md#ncnametype)[] |
| `schema_name` | `String` |
| `template_fields` | [`Vec<NCNameType>`](modules.md#ncnametype)[] |
| `template_name` | `String` |
| `user` | `String` |
| `user_prv_active_key` | `String` |
| `xferable` | `bool` |

#### Defined in

[types.ts:35](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L35)

___

### NCCreateDao

Ƭ **NCCreateDao**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `author` | `String` |
| `author_prv_key` | `String` |
| `descr` | `String` |
| `token` | `String` |

#### Defined in

[types.ts:138](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L138)

___

### NCCreateDaoProposal

Ƭ **NCCreateDaoProposal**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `dao_id` | `String` |
| `dao_owner` | `String` |
| `pass_rate` | `u32` |
| `proposer` | `String` |
| `proposer_prv_key` | `String` |
| `summary` | `String` |
| `title` | `String` |
| `url` | `String` |
| `vote_end` | `String` |
| `vote_start` | `String` |

#### Defined in

[types.ts:145](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L145)

___

### NCCreateDaoStakeProposal

Ƭ **NCCreateDaoStakeProposal**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `dao_id` | `String` |
| `dao_owner` | `String` |
| `pass_rate` | `u32` |
| `proposer` | `String` |
| `proposer_prv_key` | `String` |
| `quantity` | { `contract`: `String` ; `quantity`: `String`  } |
| `quantity.contract` | `String` |
| `quantity.quantity` | `String` |
| `to` | `String` |
| `vote_end` | `String` |
| `vote_start` | `String` |

#### Defined in

[types.ts:170](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L170)

___

### NCCreateDaoUserWhitelistProposal

Ƭ **NCCreateDaoUserWhitelistProposal**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `dao_id` | `String` |
| `dao_owner` | `String` |
| `pass_rate` | `u32` |
| `proposer` | `String` |
| `proposer_prv_key` | `String` |
| `type` | `String` |
| `user` | `String` |
| `vote_end` | `String` |
| `vote_start` | `String` |

#### Defined in

[types.ts:158](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L158)

___

### NCCreatePermission

Ƭ **NCCreatePermission**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `author` | `String` |
| `author_prv_active_key` | `String` |
| `perm_name` | `String` |
| `perm_pub_key` | `String` |

#### Defined in

[types.ts:50](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L50)

___

### NCCreatePool

Ƭ **NCCreatePool**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `is_deflatable` | `bool` |
| `is_inflatable` | `bool` |
| `is_treasury` | `bool` |
| `owner` | `String` |
| `owner_prv_active_key` | `String` |
| `ticker` | `String` |

#### Defined in

[types.ts:65](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L65)

___

### NCCreateUser

Ƭ **NCCreateUser**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `cpu_amount` | `String` |
| `net_amount` | `String` |
| `newUser` | `String` |
| `newacc_pub_active_key` | `String` |
| `newacc_pub_owner_key` | `String` |
| `payer` | `String` |
| `payer_prv_key` | `String` |
| `ram_amt` | `u32` |
| `xfer` | `bool` |

#### Defined in

[types.ts:23](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L23)

___

### NCDaoProposalVote

Ƭ **NCDaoProposalVote**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `dao_id` | `String` |
| `dao_owner` | `String` |
| `option` | `String` |
| `proposal_id` | `String` |
| `proposal_type` | `String` |
| `quantity` | `String` |
| `voter` | `String` |
| `voter_prv_key` | `String` |

#### Defined in

[types.ts:217](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L217)

___

### NCDaoWithdrawVoteDeposit

Ƭ **NCDaoWithdrawVoteDeposit**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `vote_id` | `String` |
| `voter` | `String` |
| `voter_prv_key` | `String` |

#### Defined in

[types.ts:228](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L228)

___

### NCExecuteDaoProposal

Ƭ **NCExecuteDaoProposal**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `dao_id` | `u32` |
| `dao_owner` | `String` |
| `exec` | `String` |
| `exec_prv_key` | `String` |
| `proposal_author` | `String` |
| `proposal_id` | `u32` |

#### Defined in

[types.ts:194](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L194)

___

### NCGetAccInfo

Ƭ **NCGetAccInfo**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `contract` | `String` |
| `owner` | `String` |
| `token_name` | `String` |

#### Defined in

[types.ts:392](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L392)

___

### NCGetDaoProposals

Ƭ **NCGetDaoProposals**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `dao_id` | `String` |
| `dao_owner` | `String` |
| `limit` | `u32` |
| `lower_bound` | `String` |
| `proposal_author` | `String` |
| `proposal_id` | `String` |
| `reverse` | `bool` |
| `upper_bound` | `String` |

#### Defined in

[types.ts:204](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L204)

___

### NCGetDaoWhiteList

Ƭ **NCGetDaoWhiteList**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `dao_id` | `String` |
| `dao_owner` | `String` |
| `limit` | `String` |
| `lower_bound` | `String` |
| `reverse` | `bool` |
| `upper_bound` | `String` |

#### Defined in

[types.ts:242](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L242)

___

### NCGetPoolInfo

Ƭ **NCGetPoolInfo**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `code` | `String` |
| `owner` | `String` |

#### Defined in

[types.ts:398](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L398)

___

### NCGetVotes

Ƭ **NCGetVotes**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `limit` | `String` |
| `lower_bound` | `String` |
| `reverse` | `bool` |
| `upper_bound` | `String` |
| `voter` | `String` |

#### Defined in

[types.ts:234](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L234)

___

### NCKeyPair

Ƭ **NCKeyPair**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `prv_key` | `String` |
| `pub_key` | `String` |

#### Defined in

[types.ts:6](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L6)

___

### NCKeyValPair

Ƭ **NCKeyValPair**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `key` | `String` |
| `value` | `Vec<String>` |

#### Defined in

[types.ts:251](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L251)

___

### NCLinkPerm

Ƭ **NCLinkPerm**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `action_owner` | `String` |
| `action_to_link` | `String` |
| `author` | `String` |
| `author_prv_active_key` | `String` |
| `perm_to_link` | `String` |

#### Defined in

[types.ts:57](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L57)

___

### NCMintAsset

Ƭ **NCMintAsset**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `col_name` | `String` |
| `creator` | `String` |
| `immutable_data` | [`Vec<NCKeyValPair>`](modules.md#nckeyvalpair)[] |
| `mutable_data` | [`Vec<NCKeyValPair>`](modules.md#nckeyvalpair)[] |
| `payer` | `String` |
| `payer_prv_key` | `String` |
| `sch_name` | `String` |
| `tmpl_id` | `u32` |
| `user_prv_active_key` | `String` |

#### Defined in

[types.ts:256](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L256)

___

### NCMintFile

Ƭ **NCMintFile**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `content` | `String` |
| `creator` | `String` |
| `image` | `String` |
| `name` | `String` |
| `path` | `String` |
| `payer` | `String` |
| `payer_prv_key` | `String` |
| `user_prv_active_key` | `String` |

#### Defined in

[types.ts:369](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L369)

___

### NCMintLink

Ƭ **NCMintLink**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `col_name` | `String` |
| `creator` | `String` |
| `description` | `String` |
| `image` | `String` |
| `payer` | `String` |
| `payer_prv_key` | `String` |
| `subj_name` | `String` |
| `subj_type` | `String` |

#### Defined in

[types.ts:285](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L285)

___

### NCMintProfile

Ƭ **NCMintProfile**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `apple` | `String` |
| `aspectRatio` | `String` |
| `authority` | `String` |
| `bio` | `String` |
| `blurHash` | `String` |
| `content` | `String` |
| `contentType` | `String` |
| `contentUrl` | `String` |
| `coverContentUrl` | `String` |
| `creator` | `String` |
| `discord` | `String` |
| `displayName` | `String` |
| `email` | `String` |
| `facebook` | `String` |
| `facebookId` | `String` |
| `firstName` | `String` |
| `fullName` | `String` |
| `instagram` | `String` |
| `lastName` | `String` |
| `medium` | `String` |
| `offer` | `String` |
| `payer` | `String` |
| `payer_prv_key` | `String` |
| `phone` | `String` |
| `pinterest` | `String` |
| `reddit` | `String` |
| `signal` | `String` |
| `signature` | `String` |
| `snapchat` | `String` |
| `soundcloud` | `String` |
| `source` | `String` |
| `spotify` | `String` |
| `status` | `String` |
| `telegram` | `String` |
| `tiktok` | `String` |
| `tumblr` | `String` |
| `twitter` | `String` |
| `user_prv_active_key` | `String` |
| `username` | `String` |
| `youtube` | `String` |
| `youtubeId` | `String` |

#### Defined in

[types.ts:318](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L318)

___

### NCModifyAsset

Ƭ **NCModifyAsset**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `asset_id` | `String` |
| `editor` | `String` |
| `new_data` | [`NCKeyValPair`](modules.md#nckeyvalpair)[] |
| `owner` | `String` |
| `payer` | `String` |
| `payer_prv_key` | `String` |

#### Defined in

[types.ts:268](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L268)

___

### NCNameType

Ƭ **NCNameType**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `name` | `String` |
| `type` | `String` |

#### Defined in

[types.ts:11](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L11)

___

### NCPoolsInfo

Ƭ **NCPoolsInfo**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `more` | `bool` |
| `next_key` | `String` |
| `rows` | [`NCPoolInfo`](modules/internal_.md#ncpoolinfo)[] |

#### Defined in

[types.ts:124](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L124)

___

### NCReturnInfo

Ƭ **NCReturnInfo**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `acc_balances` | `Vec<String>` |

#### Defined in

[types.ts:442](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L442)

___

### NCReturnTxs

Ƭ **NCReturnTxs**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `TxID` | `String` |
| `TxID_WithdrawVoteDeposit` | `String` |
| `TxID_addToWhiteList` | `String` |
| `TxID_approveDaoProposal` | `String` |
| `TxID_changeFile` | `String` |
| `TxID_createAcc` | `String` |
| `TxID_createCol` | `String` |
| `TxID_createDao` | `String` |
| `TxID_createDaoProposal` | `String` |
| `TxID_createPerm` | `String` |
| `TxID_createPool` | `String` |
| `TxID_createSch` | `String` |
| `TxID_createTpl` | `String` |
| `TxID_executeDaoProposal` | `String` |
| `TxID_linkPerm` | `String` |
| `TxID_mintAsset` | `String` |
| `TxID_mintFile` | `String` |
| `TxID_mintNft` | `String` |
| `TxID_mintProfile` | `String` |
| `TxID_modifyAsset` | `String` |
| `TxID_removeFromWhiteList` | `String` |
| `TxID_stakeMainDAO` | `String` |
| `TxID_stakePool` | `String` |
| `TxID_txNcoBalance` | `String` |
| `TxID_unstakeMainDAO` | `String` |
| `TxID_unstakePool` | `String` |
| `TxID_voteDaoProposal` | `String` |
| `TxID_withdrawFromPool` | `String` |
| `asset_id` | `String` |
| `dao_id` | `String` |
| `pool_code` | `String` |
| `pool_id` | `String` |
| `proposal_id` | `u32` |
| `template_id` | `String` |
| `tx` | `TransactResult` |

#### Defined in

[types.ts:403](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L403)

___

### NCStakeMainDao

Ƭ **NCStakeMainDao**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `amt` | `String` |
| `payer` | `String` |
| `payer_prv_key` | `String` |

#### Defined in

[types.ts:74](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L74)

___

### NCStakePool

Ƭ **NCStakePool**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `amt` | `String` |
| `owner` | `String` |
| `payer` | `String` |
| `payer_prv_key` | `String` |

#### Defined in

[types.ts:80](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L80)

___

### NCTxBal

Ƭ **NCTxBal**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `amt` | `String` |
| `memo` | `String` |
| `payer` | `String` |
| `payer_prv_key` | `String` |
| `to` | `String` |

#### Defined in

[types.ts:102](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L102)

___

### NCTxNcoBal

Ƭ **NCTxNcoBal**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `amt` | `String` |
| `memo` | `String` |
| `payer` | `String` |
| `payer_prv_key` | `String` |
| `to` | `String` |

#### Defined in

[types.ts:94](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L94)

___

### NCUnstakePool

Ƭ **NCUnstakePool**: `Struct`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `amt` | `String` |
| `payer` | `String` |
| `payer_prv_key` | `String` |

#### Defined in

[types.ts:87](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/types.ts#L87)

## Variables

### default\_nft\_schema

• `Const` **default\_nft\_schema**: { `name`: `String` = 'name'; `type`: `String` = "String" }[]

#### Defined in

[schemas.ts:9](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/schemas.ts#L9)

___

### devnet\_services

• `Const` **devnet\_services**: [`NCInitServices`](modules/internal_.md#ncinitservices)

#### Defined in

[system.ts:43](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/system.ts#L43)

___

### devnet\_urls

• `Const` **devnet\_urls**: [`NCInitUrls`](modules/internal_.md#nciniturls)

#### Defined in

[system.ts:26](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/system.ts#L26)

___

### file\_schema

• `Const` **file\_schema**: { `name`: `String` = 'name'; `type`: `String` = 'String' }[]

#### Defined in

[schemas.ts:19](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/schemas.ts#L19)

___

### link\_schema

• `Const` **link\_schema**: { `name`: `String` = 'name'; `type`: `String` = 'String' }[]

#### Defined in

[schemas.ts:91](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/schemas.ts#L91)

___

### profile\_schema

• `Const` **profile\_schema**: { `name`: `String` = "name"; `type`: `String` = "String" }[]

#### Defined in

[schemas.ts:38](https://github.com/newfound8ion/newcoin-sdk/blob/86b014f/src/schemas.ts#L38)

//** As of yet, the text below has not been reworked for the application.** //
# REST API

The REST API to the example app is described below.

## Get list of Things

### Request

`GET /thing/`

    curl -i -H 'Accept: application/json' http://localhost:7000/thing/

### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 2

    []



## Get Dao Proposal by Id:

```rust
    let url_val = NCInitUrlsDev::default().nodeos_url.clone();
    let gtr_response: Response = get_table_rows().await;
    let data = gtr_response.json().await;

```
### Request

`POST /thing/`

    curl -i -H 'Accept: application/json' -d 'name=Foo&status=new' http://localhost:7000/thing

### Response

    HTTP/1.1 201 Created
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 201 Created
    Connection: close
    Content-Type: application/json
    Location: /thing/1
    Content-Length: 36

    {"id":1,"name":"Foo","status":"new"}

## Get a specific Thing

### Request

`GET /thing/id`

    curl -i -H 'Accept: application/json' http://localhost:7000/thing/1

### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 36

    {"id":1,"name":"Foo","status":"new"}

## Get a non-existent Thing

### Request

`GET /thing/id`

    curl -i -H 'Accept: application/json' http://localhost:7000/thing/9999

### Response

    HTTP/1.1 404 Not Found
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 404 Not Found
    Connection: close
    Content-Type: application/json
    Content-Length: 35

    {"status":404,"reason":"Not found"}

## Create another new Thing

### Request

`POST /thing/`

    curl -i -H 'Accept: application/json' -d 'name=Bar&junk=rubbish' http://localhost:7000/thing

### Response

    HTTP/1.1 201 Created
    Date: Thu, 24 Feb 2011 12:36:31 GMT
    Status: 201 Created
    Connection: close
    Content-Type: application/json
    Location: /thing/2
    Content-Length: 35

    {"id":2,"name":"Bar","status":null}

## Get list of Things again

### Request

`GET /thing/`

    curl -i -H 'Accept: application/json' http://localhost:7000/thing/

### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:31 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 74

    [{"id":1,"name":"Foo","status":"new"},{"id":2,"name":"Bar","status":null}]

## Change a Thing's state

### Request

`PUT /thing/:id/status/changed`

    curl -i -H 'Accept: application/json' -X PUT http://localhost:7000/thing/1/status/changed

### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:31 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 40

    {"id":1,"name":"Foo","status":"changed"}

## Get changed Thing

### Request

`GET /thing/id`

    curl -i -H 'Accept: application/json' http://localhost:7000/thing/1

### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:31 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 40

    {"id":1,"name":"Foo","status":"changed"}

## Change a Thing

### Request

`PUT /thing/:id`

    curl -i -H 'Accept: application/json' -X PUT -d 'name=Foo&status=changed2' http://localhost:7000/thing/1

### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:31 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 41

    {"id":1,"name":"Foo","status":"changed2"}

## Attempt to change a Thing using partial params

### Request

`PUT /thing/:id`

    curl -i -H 'Accept: application/json' -X PUT -d 'status=changed3' http://localhost:7000/thing/1

### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:32 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 41

    {"id":1,"name":"Foo","status":"changed3"}

## Attempt to change a Thing using invalid params

### Request

`PUT /thing/:id`

    curl -i -H 'Accept: application/json' -X PUT -d 'id=99&status=changed4' http://localhost:7000/thing/1

### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:32 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 41

    {"id":1,"name":"Foo","status":"changed4"}

## Change a Thing using the _method hack

### Request

`POST /thing/:id_method=POST`

    curl -i -H 'Accept: application/json' -X POST -d 'name=Baz&_method=PUT' http://localhost:7000/thing/1

### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:32 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 41

    {"id":1,"name":"Baz","status":"changed4"}

## Change a Thing using the _method hack in the url

### Request

`POST /thing/:id_method=POST`

    curl -i -H 'Accept: application/json' -X POST -d 'name=Qux' http://localhost:7000/thing/1_method=PUT

### Response

    HTTP/1.1 404 Not Found
    Date: Thu, 24 Feb 2011 12:36:32 GMT
    Status: 404 Not Found
    Connection: close
    Content-Type: text/html;charset=utf-8
    Content-Length: 35

    {"status":404,"reason":"Not found"}

## Delete a Thing

### Request

`DELETE /thing/id`

    curl -i -H 'Accept: application/json' -X DELETE http://localhost:7000/thing/1/

### Response

    HTTP/1.1 204 No Content
    Date: Thu, 24 Feb 2011 12:36:32 GMT
    Status: 204 No Content
    Connection: close


## Try to delete same Thing again

### Request

`DELETE /thing/id`

    curl -i -H 'Accept: application/json' -X DELETE http://localhost:7000/thing/1/

### Response

    HTTP/1.1 404 Not Found
    Date: Thu, 24 Feb 2011 12:36:32 GMT
    Status: 404 Not Found
    Connection: close
    Content-Type: application/json
    Content-Length: 35

    {"status":404,"reason":"Not found"}

## Get deleted Thing

### Request

`GET /thing/1`

    curl -i -H 'Accept: application/json' http://localhost:7000/thing/1

### Response

    HTTP/1.1 404 Not Found
    Date: Thu, 24 Feb 2011 12:36:33 GMT
    Status: 404 Not Found
    Connection: close
    Content-Type: application/json
    Content-Length: 35

    {"status":404,"reason":"Not found"}

## Delete a Thing using the _method hack

### Request

`DELETE /thing/id`

    curl -i -H 'Accept: application/json' -X POST -d'_method=DELETE' http://localhost:7000/thing/2/

### Response

    HTTP/1.1 204 No Content
    Date: Thu, 24 Feb 2011 12:36:33 GMT
    Status: 204 No Content
    Connection: close

