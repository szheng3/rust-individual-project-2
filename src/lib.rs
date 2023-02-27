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

pub fn init_summarization_model(minlength:i64) -> SummarizationModel {
    let do_steps = move || -> Result<SummarizationModel, ExitFailure> {

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
        summarization_config.min_length= minlength;
        summarization_config.max_length= Option::from(minlength + 30);

        //
        // let config_resource = RemoteResource::from_pretrained(T5ConfigResources::T5_SMALL);
        // let vocab_resource = RemoteResource::from_pretrained(T5VocabResources::T5_SMALL);
        // let weights_resource = RemoteResource::from_pretrained(T5ModelResources::T5_SMALL);
        //
        // let summarization_config = SummarizationConfig::new(
        //     ModelType::T5,
        //     weights_resource,
        //     config_resource,
        //     vocab_resource,
        //     None,
        // );

        // let config_resource = Box::new(RemoteResource::from_pretrained(
        //     BartConfigResources::DISTILBART_CNN_6_6,
        // ));
        // let vocab_resource = Box::new(RemoteResource::from_pretrained(
        //     BartVocabResources::DISTILBART_CNN_6_6,
        // ));
        // let merges_resource = Box::new(RemoteResource::from_pretrained(
        //     BartMergesResources::DISTILBART_CNN_6_6,
        // ));
        // let model_resource = Box::new(RemoteResource::from_pretrained(
        //     BartModelResources::DISTILBART_CNN_6_6,
        // ));
        //
        // let summarization_config = SummarizationConfig {
        //     model_resource,
        //     config_resource,
        //     vocab_resource,
        //     merges_resource: Some(merges_resource),
        //     num_beams: 1,
        //     length_penalty: 1.0,
        //     min_length: minlength,
        //     max_length: Some(minlength+30),
        //     device: Device::Cpu,
        //     ..Default::default()
        // };
        // let config_resource = Box::new(RemoteResource::from_pretrained(
        //     ProphetNetConfigResources::PROPHETNET_LARGE_CNN_DM,
        // ));
        // let vocab_resource = Box::new(RemoteResource::from_pretrained(
        //     ProphetNetVocabResources::PROPHETNET_LARGE_CNN_DM,
        // ));
        // let weights_resource = Box::new(RemoteResource::from_pretrained(
        //     ProphetNetModelResources::PROPHETNET_LARGE_CNN_DM,
        // ));
        //
        // let summarization_config = SummarizationConfig {
        //     model_type: ModelType::ProphetNet,
        //     model_resource: weights_resource,
        //     config_resource,
        //     vocab_resource,
        //     merges_resource: None,
        //     length_penalty: 1.2,
        //     num_beams: 4,
        //     min_length: minlength,
        //     max_length: Some(minlength + 30),
        //     no_repeat_ngram_size: 3,
        //     device: Device::Cpu,
        //     ..Default::default()
        // };

        let summarization_model = SummarizationModel::new(summarization_config)?;
        Ok(summarization_model)
    };

    let summarization_model = thread::spawn(move || {
        do_steps().unwrap()
    }).join().expect("Thread panicked");
    return summarization_model;
}


pub fn sort_arr<T: Ord>(arr: &mut [T]) {
    sorting::bubble_sort(arr);
}

mod sorting {
    pub fn selection_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        for i in 0..len {
            let mut min_idx = i;
            for j in (i + 1)..len {
                if arr[j] < arr[min_idx] {
                    min_idx = j;
                }
            }
            arr.swap(min_idx, i);
        }
    }

    pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        for i in 0..len {
            for j in 0..len - i - 1 {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_bubble_sort() {
            let mut arr = [6, 2, 4, 1, 9, -2, 5];
            bubble_sort(&mut arr);
            assert_eq!(arr, [-2, 1, 2, 4, 5, 6, 9]);
        }

        #[test]
        fn test_selection_sort() {
            let mut arr = [6, 2, 4, 1, 9, -2, 5];
            selection_sort(&mut arr);
            assert_eq!(arr, [-2, 1, 2, 4, 5, 6, 9]);
        }
    }
}
