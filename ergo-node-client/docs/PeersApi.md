# \PeersApi

All URIs are relative to *https://raw.githubusercontent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**connect_to_peer**](PeersApi.md#connect_to_peer) | **POST** /peers/connect | Add address to peers list
[**get_all_peers**](PeersApi.md#get_all_peers) | **GET** /peers/all | Get all known peers
[**get_blacklisted_peers**](PeersApi.md#get_blacklisted_peers) | **GET** /peers/blacklisted | Get blacklisted peers
[**get_connected_peers**](PeersApi.md#get_connected_peers) | **GET** /peers/connected | Get current connected peers
[**get_peers_status**](PeersApi.md#get_peers_status) | **GET** /peers/status | Get last incoming message timestamp and current network time
[**get_peers_sync_info**](PeersApi.md#get_peers_sync_info) | **GET** /peers/syncInfo | Get sync info reported by peers, including versions, current status and height (if available)
[**get_peers_track_info**](PeersApi.md#get_peers_track_info) | **GET** /peers/trackInfo | Get track info reported by peers, including count of invalid modifiers and details of requested and received modifiers



## connect_to_peer

> connect_to_peer(body)
Add address to peers list

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


## get_all_peers

> Vec<crate::models::Peer> get_all_peers()
Get all known peers

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Peer>**](Peer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blacklisted_peers

> crate::models::BlacklistedPeers get_blacklisted_peers()
Get blacklisted peers

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BlacklistedPeers**](BlacklistedPeers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connected_peers

> Vec<crate::models::Peer> get_connected_peers()
Get current connected peers

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Peer>**](Peer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_peers_status

> Vec<crate::models::PeersStatus> get_peers_status()
Get last incoming message timestamp and current network time

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PeersStatus>**](PeersStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_peers_sync_info

> Vec<crate::models::SyncInfo> get_peers_sync_info()
Get sync info reported by peers, including versions, current status and height (if available)

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::SyncInfo>**](SyncInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_peers_track_info

> Vec<crate::models::TrackInfo> get_peers_track_info()
Get track info reported by peers, including count of invalid modifiers and details of requested and received modifiers

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::TrackInfo>**](TrackInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

