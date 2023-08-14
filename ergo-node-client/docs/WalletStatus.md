# WalletStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_initialized** | **bool** | true if wallet is initialized, false otherwise | 
**is_unlocked** | **bool** | true if wallet is unlocked, false otherwise | 
**change_address** | **String** | address to send change to. Empty when wallet is not initialized or locked. By default change address correponds to root key address, could be set via /wallet/updateChangeAddress method. | 
**wallet_height** | **i32** | last scanned height for the wallet (and external scans) | 
**error** | **String** | last wallet error caught | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


