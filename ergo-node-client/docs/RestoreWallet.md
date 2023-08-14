# RestoreWallet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pass** | **String** | Password to encrypt wallet file with | 
**mnemonic** | **String** | Mnemonic seed | 
**mnemonic_pass** | Option<**String**> | Optional pass to password-protect mnemonic seed | [optional]
**use_pre1627_key_derivation** | **bool** | use incorrect(previous) BIP32 key derivation (see https://github.com/ergoplatform/ergo/issues/1627 for details). It's recommended to set to 'true' if the original wallet was created by ergo node before v4.0.105. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


