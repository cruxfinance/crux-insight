# \WalletApi

All URIs are relative to *https://raw.githubusercontent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_box**](WalletApi.md#add_box) | **POST** /scan/addBox | Adds a box to scans, writes box to database if it is not there. You can use scan number 10 to add a box to the wallet.
[**check_seed**](WalletApi.md#check_seed) | **POST** /wallet/check | Check whether mnemonic phrase is corresponding to the wallet seed
[**extract_hints**](WalletApi.md#extract_hints) | **POST** /wallet/extractHints | Extract hints from a transaction
[**generate_commitments**](WalletApi.md#generate_commitments) | **POST** /wallet/generateCommitments | Generate signature commitments for inputs of an unsigned transaction
[**get_wallet_status**](WalletApi.md#get_wallet_status) | **GET** /wallet/status | Get wallet status
[**wallet_addresses**](WalletApi.md#wallet_addresses) | **GET** /wallet/addresses | Get wallet addresses
[**wallet_balances**](WalletApi.md#wallet_balances) | **GET** /wallet/balances | Get total amount of confirmed Ergo tokens and assets
[**wallet_balances_unconfirmed**](WalletApi.md#wallet_balances_unconfirmed) | **GET** /wallet/balances/withUnconfirmed | Get summary amount of confirmed plus unconfirmed Ergo tokens and assets
[**wallet_boxes**](WalletApi.md#wallet_boxes) | **GET** /wallet/boxes | Get a list of all wallet-related boxes, both spent and unspent. Set minConfirmations to -1 to get mempool boxes included.
[**wallet_boxes_collect**](WalletApi.md#wallet_boxes_collect) | **POST** /wallet/boxes/collect | Get a list of collected boxes.
[**wallet_derive_key**](WalletApi.md#wallet_derive_key) | **POST** /wallet/deriveKey | Derive new key according to a provided path
[**wallet_derive_next_key**](WalletApi.md#wallet_derive_next_key) | **GET** /wallet/deriveNextKey | Derive next key
[**wallet_get_private_key**](WalletApi.md#wallet_get_private_key) | **POST** /wallet/getPrivateKey | Get the private key corresponding to a known address
[**wallet_get_transaction**](WalletApi.md#wallet_get_transaction) | **GET** /wallet/transactionById | Get wallet-related transaction by id
[**wallet_init**](WalletApi.md#wallet_init) | **POST** /wallet/init | Initialize new wallet with randomly generated seed
[**wallet_lock**](WalletApi.md#wallet_lock) | **GET** /wallet/lock | Lock wallet
[**wallet_payment_transaction_generate_and_send**](WalletApi.md#wallet_payment_transaction_generate_and_send) | **POST** /wallet/payment/send | Generate and send payment transaction (default fee of 0.001 Erg is used)
[**wallet_rescan**](WalletApi.md#wallet_rescan) | **POST** /wallet/rescan | Rescan wallet (all the available full blocks). When fromHeight is set wallet would not see any boxes below it.
[**wallet_restore**](WalletApi.md#wallet_restore) | **POST** /wallet/restore | Create new wallet from existing mnemonic seed
[**wallet_transaction_generate**](WalletApi.md#wallet_transaction_generate) | **POST** /wallet/transaction/generate | Generate arbitrary transaction from array of requests.
[**wallet_transaction_generate_and_send**](WalletApi.md#wallet_transaction_generate_and_send) | **POST** /wallet/transaction/send | Generate and send arbitrary transaction
[**wallet_transaction_sign**](WalletApi.md#wallet_transaction_sign) | **POST** /wallet/transaction/sign | Sign arbitrary unsigned transaction with wallet secrets and also secrets provided.
[**wallet_transactions**](WalletApi.md#wallet_transactions) | **GET** /wallet/transactions | Get a list of all wallet-related transactions
[**wallet_transactions_by_scan_id**](WalletApi.md#wallet_transactions_by_scan_id) | **GET** /wallet/transactionsByScanId/{scanId} | Get scan-related transactions by scan id
[**wallet_unlock**](WalletApi.md#wallet_unlock) | **POST** /wallet/unlock | Unlock wallet
[**wallet_unsigned_transaction_generate**](WalletApi.md#wallet_unsigned_transaction_generate) | **POST** /wallet/transaction/generateUnsigned | Generate unsigned transaction from array of requests.
[**wallet_unspent_boxes**](WalletApi.md#wallet_unspent_boxes) | **GET** /wallet/boxes/unspent | Get a list of unspent boxes. Set minConfirmations to -1 to have mempool boxes considered.
[**wallet_update_change_address**](WalletApi.md#wallet_update_change_address) | **POST** /wallet/updateChangeAddress | Update address to be used to send change to



## add_box

> String add_box(scan_ids_box)
Adds a box to scans, writes box to database if it is not there. You can use scan number 10 to add a box to the wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_ids_box** | [**ScanIdsBox**](ScanIdsBox.md) |  | [required] |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_seed

> crate::models::PassphraseMatch check_seed(check_wallet)
Check whether mnemonic phrase is corresponding to the wallet seed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_wallet** | [**CheckWallet**](CheckWallet.md) |  | [required] |

### Return type

[**crate::models::PassphraseMatch**](PassphraseMatch.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extract_hints

> crate::models::TransactionHintsBag extract_hints(hint_extraction_request)
Extract hints from a transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hint_extraction_request** | [**HintExtractionRequest**](HintExtractionRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionHintsBag**](TransactionHintsBag.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_commitments

> crate::models::TransactionHintsBag generate_commitments(generate_commitments_request)
Generate signature commitments for inputs of an unsigned transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**generate_commitments_request** | [**GenerateCommitmentsRequest**](GenerateCommitmentsRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionHintsBag**](TransactionHintsBag.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wallet_status

> crate::models::WalletStatus get_wallet_status()
Get wallet status

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WalletStatus**](WalletStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_addresses

> Vec<String> wallet_addresses()
Get wallet addresses

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_balances

> crate::models::BalancesSnapshot wallet_balances()
Get total amount of confirmed Ergo tokens and assets

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BalancesSnapshot**](BalancesSnapshot.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_balances_unconfirmed

> crate::models::BalancesSnapshot wallet_balances_unconfirmed()
Get summary amount of confirmed plus unconfirmed Ergo tokens and assets

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BalancesSnapshot**](BalancesSnapshot.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_boxes

> Vec<crate::models::WalletBox> wallet_boxes(min_confirmations, max_confirmations, min_inclusion_height, max_inclusion_height)
Get a list of all wallet-related boxes, both spent and unspent. Set minConfirmations to -1 to get mempool boxes included.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**min_confirmations** | Option<**i32**> | Minimal number of confirmations, -1 means we consider unconfirmed |  |[default to 0]
**max_confirmations** | Option<**i32**> | Maximum number of confirmations, -1 means unlimited |  |[default to -1]
**min_inclusion_height** | Option<**i32**> | Minimal box inclusion height |  |[default to 0]
**max_inclusion_height** | Option<**i32**> | Maximum box inclusion height, -1 means unlimited |  |[default to -1]

### Return type

[**Vec<crate::models::WalletBox>**](WalletBox.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_boxes_collect

> Vec<crate::models::WalletBox> wallet_boxes_collect(boxes_request_holder)
Get a list of collected boxes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**boxes_request_holder** | [**BoxesRequestHolder**](BoxesRequestHolder.md) | This API method recieves balance and assets, according to which, it's collecting result | [required] |

### Return type

[**Vec<crate::models::WalletBox>**](WalletBox.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_derive_key

> crate::models::DeriveKeyResult wallet_derive_key(derive_key)
Derive new key according to a provided path

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**derive_key** | [**DeriveKey**](DeriveKey.md) |  | [required] |

### Return type

[**crate::models::DeriveKeyResult**](DeriveKeyResult.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_derive_next_key

> crate::models::DeriveNextKeyResult wallet_derive_next_key()
Derive next key

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DeriveNextKeyResult**](DeriveNextKeyResult.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_get_private_key

> String wallet_get_private_key(body)
Get the private key corresponding to a known address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_get_transaction

> Vec<crate::models::WalletTransaction> wallet_get_transaction(id)
Get wallet-related transaction by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Transaction id | [required] |

### Return type

[**Vec<crate::models::WalletTransaction>**](WalletTransaction.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_init

> crate::models::InitWalletResult wallet_init(init_wallet)
Initialize new wallet with randomly generated seed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**init_wallet** | [**InitWallet**](InitWallet.md) |  | [required] |

### Return type

[**crate::models::InitWalletResult**](InitWalletResult.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_lock

> wallet_lock()
Lock wallet

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_payment_transaction_generate_and_send

> String wallet_payment_transaction_generate_and_send(payment_request)
Generate and send payment transaction (default fee of 0.001 Erg is used)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_request** | [**Vec<crate::models::PaymentRequest>**](PaymentRequest.md) |  | [required] |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_rescan

> wallet_rescan(wallet_rescan_request)
Rescan wallet (all the available full blocks). When fromHeight is set wallet would not see any boxes below it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_rescan_request** | Option<[**WalletRescanRequest**](WalletRescanRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_restore

> wallet_restore(restore_wallet)
Create new wallet from existing mnemonic seed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**restore_wallet** | [**RestoreWallet**](RestoreWallet.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_transaction_generate

> crate::models::ErgoTransaction wallet_transaction_generate(requests_holder)
Generate arbitrary transaction from array of requests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**requests_holder** | [**RequestsHolder**](RequestsHolder.md) | This API method receives a sequence of requests as an input. Each request will produce an output of the resulting transaction (with fee output created automatically). Currently supported types of requests are payment and asset issuance requests. An example for a transaction with requests of both kinds is provided below. Please note that for the payment request \"assets\" and \"registers\" fields are not needed. For asset issuance request, \"registers\" field is not needed. You may specify boxes to spend by providing them in \"inputsRaw\". Please note you need to have strict equality between input and output total amounts of Ergs in this case. If you want wallet to pick up the boxes, leave \"inputsRaw\" empty. | [required] |

### Return type

[**crate::models::ErgoTransaction**](ErgoTransaction.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_transaction_generate_and_send

> String wallet_transaction_generate_and_send(requests_holder)
Generate and send arbitrary transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**requests_holder** | [**RequestsHolder**](RequestsHolder.md) | See description of /wallet/transaction/generate | [required] |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_transaction_sign

> crate::models::ErgoTransaction wallet_transaction_sign(transaction_signing_request)
Sign arbitrary unsigned transaction with wallet secrets and also secrets provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_signing_request** | [**TransactionSigningRequest**](TransactionSigningRequest.md) | With this API method an arbitrary unsigned transaction can be signed with secrets provided or stored in the wallet. Both DLOG and Diffie-Hellman tuple secrets are supported. Please note that the unsigned transaction contains only identifiers of inputs and data inputs. If the node holds UTXO set, it is able to extract boxes needed. Otherwise, input (and data-input) boxes can be provided in \"inputsRaw\" and \"dataInputsRaw\" fields. | [required] |

### Return type

[**crate::models::ErgoTransaction**](ErgoTransaction.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_transactions

> Vec<crate::models::WalletTransaction> wallet_transactions(min_inclusion_height, max_inclusion_height, min_confirmations, max_confirmations)
Get a list of all wallet-related transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**min_inclusion_height** | Option<**i32**> | Minimal tx inclusion height |  |
**max_inclusion_height** | Option<**i32**> | Maximal tx inclusion height |  |
**min_confirmations** | Option<**i32**> | Minimal confirmations number |  |
**max_confirmations** | Option<**i32**> | Maximal confirmations number |  |

### Return type

[**Vec<crate::models::WalletTransaction>**](WalletTransaction.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_transactions_by_scan_id

> Vec<crate::models::WalletTransaction> wallet_transactions_by_scan_id(scan_id, min_inclusion_height, max_inclusion_height, min_confirmations, max_confirmations, include_unconfirmed)
Get scan-related transactions by scan id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **i32** | identifier of a scan | [required] |
**min_inclusion_height** | Option<**i32**> | Minimal tx inclusion height |  |
**max_inclusion_height** | Option<**i32**> | Maximal tx inclusion height |  |
**min_confirmations** | Option<**i32**> | Minimal confirmations number |  |
**max_confirmations** | Option<**i32**> | Maximal confirmations number |  |
**include_unconfirmed** | Option<**bool**> | Include transactions from mempool |  |[default to false]

### Return type

[**Vec<crate::models::WalletTransaction>**](WalletTransaction.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_unlock

> wallet_unlock(unlock_wallet)
Unlock wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unlock_wallet** | [**UnlockWallet**](UnlockWallet.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_unsigned_transaction_generate

> crate::models::UnsignedErgoTransaction wallet_unsigned_transaction_generate(requests_holder)
Generate unsigned transaction from array of requests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**requests_holder** | [**RequestsHolder**](RequestsHolder.md) | The same as /wallet/transaction/generate but generates unsigned transaction. | [required] |

### Return type

[**crate::models::UnsignedErgoTransaction**](UnsignedErgoTransaction.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_unspent_boxes

> Vec<crate::models::WalletBox> wallet_unspent_boxes(min_confirmations, max_confirmations, min_inclusion_height, max_inclusion_height)
Get a list of unspent boxes. Set minConfirmations to -1 to have mempool boxes considered.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**min_confirmations** | Option<**i32**> | Minimal number of confirmations, -1 means we consider unconfirmed |  |[default to 0]
**max_confirmations** | Option<**i32**> | Maximum number of confirmations, -1 means unlimited |  |[default to -1]
**min_inclusion_height** | Option<**i32**> | Minimal box inclusion height |  |[default to 0]
**max_inclusion_height** | Option<**i32**> | Maximum box inclusion height, -1 means unlimited |  |[default to -1]

### Return type

[**Vec<crate::models::WalletBox>**](WalletBox.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_update_change_address

> wallet_update_change_address(body)
Update address to be used to send change to

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

