# \ScanApi

All URIs are relative to *https://raw.githubusercontent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_box**](ScanApi.md#add_box) | **POST** /scan/addBox | Adds a box to scans, writes box to database if it is not there. You can use scan number 10 to add a box to the wallet.
[**deregister_scan**](ScanApi.md#deregister_scan) | **POST** /scan/deregister | Stop tracking and deregister scan
[**list_all_scans**](ScanApi.md#list_all_scans) | **GET** /scan/listAll | List all the registered scans
[**list_spent_scans**](ScanApi.md#list_spent_scans) | **GET** /scan/spentBoxes/{scanId} | List boxes which are spent.
[**list_unspent_scans**](ScanApi.md#list_unspent_scans) | **GET** /scan/unspentBoxes/{scanId} | List boxes which are not spent.
[**register_scan**](ScanApi.md#register_scan) | **POST** /scan/register | Register a scan
[**scan_stop_tracking**](ScanApi.md#scan_stop_tracking) | **POST** /scan/stopTracking | Stop scan-related box tracking



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


## deregister_scan

> crate::models::ScanId deregister_scan(scan_id)
Stop tracking and deregister scan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | [**ScanId**](ScanId.md) |  | [required] |

### Return type

[**crate::models::ScanId**](ScanId.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_all_scans

> Vec<crate::models::Scan> list_all_scans()
List all the registered scans

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Scan>**](Scan.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_spent_scans

> Vec<crate::models::WalletBox> list_spent_scans(scan_id, min_confirmations, max_confirmations, min_inclusion_height, max_inclusion_height)
List boxes which are spent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **i32** | identifier of a scan | [required] |
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


## list_unspent_scans

> Vec<crate::models::WalletBox> list_unspent_scans(scan_id, min_confirmations, max_confirmations, min_inclusion_height, max_inclusion_height)
List boxes which are not spent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **i32** | identifier of a scan | [required] |
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


## register_scan

> crate::models::ScanId register_scan(scan_request)
Register a scan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_request** | [**ScanRequest**](ScanRequest.md) |  | [required] |

### Return type

[**crate::models::ScanId**](ScanId.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scan_stop_tracking

> crate::models::ScanIdBoxId scan_stop_tracking(scan_id_box_id)
Stop scan-related box tracking

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id_box_id** | [**ScanIdBoxId**](ScanIdBoxId.md) |  | [required] |

### Return type

[**crate::models::ScanIdBoxId**](ScanIdBoxId.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

