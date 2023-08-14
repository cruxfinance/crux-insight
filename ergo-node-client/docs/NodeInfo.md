# NodeInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**app_version** | **String** |  | 
**full_height** | Option<**i32**> | Can be 'null' if state is empty (no full block is applied since node launch) | 
**headers_height** | Option<**i32**> | Can be 'null' if state is empty (no header applied since node launch) | 
**max_peer_height** | Option<**i32**> | Maximum block height of connected peers. Can be 'null' if state is empty (no peer connected since node launch) | 
**best_full_header_id** | Option<**String**> | Base16-encoded 32 byte modifier id | 
**previous_full_header_id** | Option<**String**> | Base16-encoded 32 byte modifier id | 
**best_header_id** | Option<**String**> | Base16-encoded 32 byte modifier id | 
**state_root** | Option<**String**> | Can be 'null' if state is empty (no full block is applied since node launch) | 
**state_type** | **String** |  | 
**state_version** | Option<**String**> | Can be 'null' if no full block is applied since node launch | 
**is_mining** | **bool** |  | 
**peers_count** | **i32** | Number of connected peers | 
**unconfirmed_count** | **i32** | Current unconfirmed transactions count | 
**difficulty** | Option<**i32**> | Difficulty on current bestFullHeaderId. Can be 'null' if no full block is applied since node launch. Difficulty is a BigInt integer.  | 
**current_time** | **i64** | Basic timestamp definition | 
**launch_time** | **i64** | Basic timestamp definition | 
**headers_score** | Option<**i32**> | Can be 'null' if no headers is applied since node launch. headersScore is a BigInt integer. | 
**full_blocks_score** | Option<**i32**> | Can be 'null' if no full block is applied since node launch. fullBlocksScore is a BigInt integer. | 
**genesis_block_id** | Option<**String**> | Base16-encoded 32 byte modifier id | 
**parameters** | [**crate::models::Parameters**](.md) |  | 
**eip27_supported** | Option<**bool**> | Whether EIP-27 locked in | [optional]
**rest_api_url** | Option<**String**> | Publicly accessible url of node which exposes restApi in firewall | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


