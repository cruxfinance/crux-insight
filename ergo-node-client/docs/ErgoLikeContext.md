# ErgoLikeContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_block_utxo_root** | [**crate::models::AvlTreeData**](.md) |  | 
**headers** | [**Vec<crate::models::SigmaHeader>**](SigmaHeader.md) | fixed number of last block headers in descending order (first header is the newest one) | 
**pre_header** | [**crate::models::PreHeader**](.md) |  | 
**data_boxes** | [**Vec<crate::models::ErgoTransactionOutput>**](ErgoTransactionOutput.md) | boxes, that corresponds to id's of `spendingTransaction.dataInputs` | 
**boxes_to_spend** | [**Vec<crate::models::ErgoTransactionOutput>**](ErgoTransactionOutput.md) | boxes, that corresponds to id's of `spendingTransaction.inputs` | 
**spending_transaction** | [**crate::models::ErgoLikeTransaction**](.md) |  | 
**self_index** | **i64** | index of the box in `boxesToSpend` that contains the script we're evaluating | 
**extension** | [**serde_json::Value**](.md) | prover-defined key-value pairs, that may be used inside a script | 
**validation_settings** | **String** | validation parameters passed to Interpreter.verify to detect soft-fork conditions | 
**cost_limit** | **i64** | hard limit on accumulated execution cost, if exceeded lead to CostLimitException to be thrown | 
**init_cost** | **i64** | initial value of execution cost already accumulated before Interpreter.verify is called | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


