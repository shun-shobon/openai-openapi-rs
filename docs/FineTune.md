# FineTune

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**object** | **String** |  | 
**created_at** | **i32** |  | 
**updated_at** | **i32** |  | 
**model** | **String** |  | 
**fine_tuned_model** | Option<**String**> |  | 
**organization_id** | **String** |  | 
**status** | **String** |  | 
**hyperparams** | [**serde_json::Value**](.md) |  | 
**training_files** | [**Vec<crate::models::OpenAiFile>**](OpenAIFile.md) |  | 
**validation_files** | [**Vec<crate::models::OpenAiFile>**](OpenAIFile.md) |  | 
**result_files** | [**Vec<crate::models::OpenAiFile>**](OpenAIFile.md) |  | 
**events** | Option<[**Vec<crate::models::FineTuneEvent>**](FineTuneEvent.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


