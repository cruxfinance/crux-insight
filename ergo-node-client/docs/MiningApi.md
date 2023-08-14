# \MiningApi

All URIs are relative to *https://raw.githubusercontent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mining_read_miner_reward_address**](MiningApi.md#mining_read_miner_reward_address) | **GET** /mining/rewardAddress | Read miner reward address
[**mining_read_miner_reward_pubkey**](MiningApi.md#mining_read_miner_reward_pubkey) | **GET** /mining/rewardPublicKey | Read public key associated with miner rewards
[**mining_request_block_candidate**](MiningApi.md#mining_request_block_candidate) | **GET** /mining/candidate | Request block candidate
[**mining_request_block_candidate_with_mandatory_transactions**](MiningApi.md#mining_request_block_candidate_with_mandatory_transactions) | **POST** /mining/candidateWithTxs | Request block candidate
[**mining_submit_solution**](MiningApi.md#mining_submit_solution) | **POST** /mining/solution | Submit solution for current candidate



## mining_read_miner_reward_address

> crate::models::RewardAddress mining_read_miner_reward_address()
Read miner reward address

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RewardAddress**](RewardAddress.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mining_read_miner_reward_pubkey

> crate::models::RewardPubKey mining_read_miner_reward_pubkey()
Read public key associated with miner rewards

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RewardPubKey**](RewardPubKey.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mining_request_block_candidate

> crate::models::WorkMessage mining_request_block_candidate()
Request block candidate

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WorkMessage**](WorkMessage.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mining_request_block_candidate_with_mandatory_transactions

> crate::models::WorkMessage mining_request_block_candidate_with_mandatory_transactions(ergo_transaction)
Request block candidate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ergo_transaction** | [**Vec<crate::models::ErgoTransaction>**](ErgoTransaction.md) |  | [required] |

### Return type

[**crate::models::WorkMessage**](WorkMessage.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mining_submit_solution

> mining_submit_solution(pow_solutions)
Submit solution for current candidate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pow_solutions** | [**PowSolutions**](PowSolutions.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

