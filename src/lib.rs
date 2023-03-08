use std::sync::Once;
use std::thread;

use actix_web::rt::Runtime;
use exitfailure::ExitFailure;
use rust_bert::bart::{
    BartConfigResources, BartMergesResources, BartModelResources, BartVocabResources,
};
use rust_bert::longt5::{LongT5ConfigResources, LongT5ModelResources, LongT5VocabResources};
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};
use rust_bert::prophetnet::{ProphetNetConfigResources, ProphetNetModelResources, ProphetNetVocabResources};
use rust_bert::resources::RemoteResource;
use rust_bert::t5::{T5ConfigResources, T5ModelResources, T5VocabResources};
use serde::Deserialize;
use serde::Serialize;
use tch::Device;

static mut SUMMARIZATION_MODEL: Option<SummarizationModel> = None;

static INIT_MODEL: Once = Once::new();

pub fn init_summarization_model(model_type: ModelType, minlength: i64) -> SummarizationModel {
    let do_steps = move || -> Result<SummarizationModel, ExitFailure> {
        let summarization_config = match model_type {
            ModelType::LongT5 => {
                let config_resource = RemoteResource::from_pretrained(LongT5ConfigResources::TGLOBAL_BASE_BOOK_SUMMARY);
                let vocab_resource = RemoteResource::from_pretrained(LongT5VocabResources::TGLOBAL_BASE_BOOK_SUMMARY);
                let weights_resource = RemoteResource::from_pretrained(LongT5ModelResources::TGLOBAL_BASE_BOOK_SUMMARY);
                let mut summarization_config = SummarizationConfig::new(
                    ModelType::LongT5,
                    weights_resource,
                    config_resource,
                    vocab_resource,
                    None,
                );
                summarization_config.min_length = minlength;
                summarization_config.max_length = Option::from(minlength + 30);
                summarization_config
            }
            ModelType::T5 => {
                let config_resource = RemoteResource::from_pretrained(T5ConfigResources::T5_SMALL);
                let vocab_resource = RemoteResource::from_pretrained(T5VocabResources::T5_SMALL);
                let weights_resource = RemoteResource::from_pretrained(T5ModelResources::T5_SMALL);
                let mut summarization_config=SummarizationConfig::new(
                    ModelType::T5,
                    weights_resource,
                    config_resource,
                    vocab_resource,
                    None,
                );
                summarization_config.min_length = minlength;
                summarization_config.max_length = Option::from(minlength + 30);
                summarization_config
            }
            ModelType::Bart => {
                let config_resource = Box::new(RemoteResource::from_pretrained(
                    BartConfigResources::DISTILBART_CNN_6_6,
                ));
                let vocab_resource = Box::new(RemoteResource::from_pretrained(
                    BartVocabResources::DISTILBART_CNN_6_6,
                ));
                let merges_resource = Box::new(RemoteResource::from_pretrained(
                    BartMergesResources::DISTILBART_CNN_6_6,
                ));
                let model_resource = Box::new(RemoteResource::from_pretrained(
                    BartModelResources::DISTILBART_CNN_6_6,
                ));
                let summarization_config = SummarizationConfig {
                    model_resource,
                    config_resource,
                    vocab_resource,
                    merges_resource: Some(merges_resource),
                    num_beams: 1,
                    length_penalty: 1.0,
                    min_length: minlength,
                    max_length: Some(minlength + 30),
                    device: Device::cuda_if_available(),
                    ..Default::default()
                };
                summarization_config
            }
            _ => {
                let config_resource = Box::new(RemoteResource::from_pretrained(
                    ProphetNetConfigResources::PROPHETNET_LARGE_CNN_DM,
                ));
                let vocab_resource = Box::new(RemoteResource::from_pretrained(
                    ProphetNetVocabResources::PROPHETNET_LARGE_CNN_DM,
                ));
                let weights_resource = Box::new(RemoteResource::from_pretrained(
                    ProphetNetModelResources::PROPHETNET_LARGE_CNN_DM,
                ));

                let summarization_config = SummarizationConfig {
                    model_type: ModelType::ProphetNet,
                    model_resource: weights_resource,
                    config_resource,
                    vocab_resource,
                    merges_resource: None,
                    length_penalty: 1.2,
                    num_beams: 4,
                    min_length: minlength,
                    max_length: Some(minlength + 30),
                    no_repeat_ngram_size: 3,
                    device: Device::cuda_if_available(),
                    ..Default::default()
                };
                summarization_config
            }
        };


        let summarization_model = SummarizationModel::new(summarization_config)?;
        Ok(summarization_model)
    };

    let summarization_model = thread::spawn(move || {
        do_steps().unwrap()
    }).join().expect("Thread panicked");
    return summarization_model;
}

