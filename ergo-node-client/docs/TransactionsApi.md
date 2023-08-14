# \TransactionsApi

All URIs are relative to *https://raw.githubusercontent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_transaction**](TransactionsApi.md#check_transaction) | **POST** /transactions/check | Checks an Ergo transaction without sending it over the network. Checks that transaction is valid and its inputs are in the UTXO set. Returns transaction identifier if the transaction is passing the checks.
[**check_transaction_as_bytes**](TransactionsApi.md#check_transaction_as_bytes) | **POST** /transactions/checkBytes | Checks an Ergo transaction without sending it over the network given in form of hex-encoded transaction bytes. Checks that transaction is valid and its inputs are in the UTXO set. Returns transaction identifier if the transaction is passing the checks.
[**check_unconfirmed_transaction**](TransactionsApi.md#check_unconfirmed_transaction) | **HEAD** /transactions/unconfirmed/{txId} | Check if given transaction is unconfirmed in pool
[**get_expected_wait_time**](TransactionsApi.md#get_expected_wait_time) | **GET** /transactions/waitTime | Get expected wait time for the transaction with specified fee and size
[**get_fee_histogram**](TransactionsApi.md#get_fee_histogram) | **GET** /transactions/poolHistogram | Get histogram (waittime, (n_trans, sum(fee)) for transactions in mempool. It contains \"bins\"+1 bins, where i-th elements corresponds to transaction with wait time [i*maxtime/bins, (i+1)*maxtime/bins), and last bin corresponds to the transactions with wait time >= maxtime.
[**get_recommended_fee**](TransactionsApi.md#get_recommended_fee) | **GET** /transactions/getFee | Get recommended fee (in nanoErgs) for a transaction with specified size (in bytes) to be proceeded in specified time (in minutes)
[**get_unconfirmed_transaction_by_id**](TransactionsApi.md#get_unconfirmed_transaction_by_id) | **GET** /transactions/unconfirmed/byTransactionId/{txId} | Get unconfirmed transaction from pool
[**get_unconfirmed_transactions**](TransactionsApi.md#get_unconfirmed_transactions) | **GET** /transactions/unconfirmed | Get current pool of the unconfirmed transactions pool
[**get_unconfirmed_transactions_by_ergo_tree**](TransactionsApi.md#get_unconfirmed_transactions_by_ergo_tree) | **POST** /transactions/unconfirmed/byErgoTree | Finds unconfirmed transactions by ErgoTree hex of one of its output or input boxes (if present in UtxoState)
[**send_transaction**](TransactionsApi.md#send_transaction) | **POST** /transactions | Submit an Ergo transaction to unconfirmed pool to send it over the network
[**send_transaction_as_bytes**](TransactionsApi.md#send_transaction_as_bytes) | **POST** /transactions/bytes | Submit an Ergo transaction given as hex-encoded transaction bytes to unconfirmed pool to send it over the network



## check_transaction

> String check_transaction(ergo_transaction)
Checks an Ergo transaction without sending it over the network. Checks that transaction is valid and its inputs are in the UTXO set. Returns transaction identifier if the transaction is passing the checks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ergo_transaction** | [**ErgoTransaction**](ErgoTransaction.md) |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_transaction_as_bytes

> String check_transaction_as_bytes(body)
Checks an Ergo transaction without sending it over the network given in form of hex-encoded transaction bytes. Checks that transaction is valid and its inputs are in the UTXO set. Returns transaction identifier if the transaction is passing the checks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_unconfirmed_transaction

> check_unconfirmed_transaction(tx_id)
Check if given transaction is unconfirmed in pool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** | ID of a transaction in question | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_expected_wait_time

> i32 get_expected_wait_time(fee, tx_size)
Get expected wait time for the transaction with specified fee and size

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fee** | **i32** | Transaction fee (in nanoErgs) | [required] |[default to 1]
**tx_size** | **i32** | Transaction size | [required] |[default to 100]

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fee_histogram

> Vec<crate::models::FeeHistogramBin> get_fee_histogram(bins, maxtime)
Get histogram (waittime, (n_trans, sum(fee)) for transactions in mempool. It contains \"bins\"+1 bins, where i-th elements corresponds to transaction with wait time [i*maxtime/bins, (i+1)*maxtime/bins), and last bin corresponds to the transactions with wait time >= maxtime.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bins** | Option<**i32**> | The number of bins in histogram |  |[default to 10]
**maxtime** | Option<**i64**> | Maximal wait time in milliseconds |  |[default to 60000]

### Return type

[**Vec<crate::models::FeeHistogramBin>**](FeeHistogramBin.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recommended_fee

> i32 get_recommended_fee(wait_time, tx_size)
Get recommended fee (in nanoErgs) for a transaction with specified size (in bytes) to be proceeded in specified time (in minutes)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wait_time** | **i32** | Maximum transaction wait time in minutes | [required] |[default to 1]
**tx_size** | **i32** | Transaction size | [required] |[default to 100]

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unconfirmed_transaction_by_id

> crate::models::ErgoTransaction get_unconfirmed_transaction_by_id(tx_id)
Get unconfirmed transaction from pool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** | ID of a transaction in question | [required] |

### Return type

[**crate::models::ErgoTransaction**](ErgoTransaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unconfirmed_transactions

> Vec<crate::models::ErgoTransaction> get_unconfirmed_transactions(limit, offset)
Get current pool of the unconfirmed transactions pool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items in list to return |  |[default to 50]
**offset** | Option<**i32**> | The number of items in list to skip |  |[default to 0]

### Return type

[**Vec<crate::models::ErgoTransaction>**](ErgoTransaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unconfirmed_transactions_by_ergo_tree

> Vec<crate::models::ErgoTransaction> get_unconfirmed_transactions_by_ergo_tree(body, limit, offset)
Finds unconfirmed transactions by ErgoTree hex of one of its output or input boxes (if present in UtxoState)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |
**limit** | Option<**i32**> | The number of items in list to return |  |[default to 50]
**offset** | Option<**i32**> | The number of items in list to skip |  |[default to 0]

### Return type

[**Vec<crate::models::ErgoTransaction>**](ErgoTransaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_transaction

> String send_transaction(ergo_transaction)
Submit an Ergo transaction to unconfirmed pool to send it over the network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ergo_transaction** | [**ErgoTransaction**](ErgoTransaction.md) |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_transaction_as_bytes

> String send_transaction_as_bytes(body)
Submit an Ergo transaction given as hex-encoded transaction bytes to unconfirmed pool to send it over the network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

