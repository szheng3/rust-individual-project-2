use rust_bert::pipelines::common::ModelType;
use crate::lib;

#[test]
fn test_bart() {
    let summarization_model = lib::init_summarization_model(ModelType::Bart, 10);
    let input = [String::new(); 1];
    let _output = summarization_model.summarize(&input);
    let result = String::from(_output.join(" "));


    assert_ne!(result.to_string().len(), 0);
}

#[test]
fn test_t5() {
    let summarization_model = lib::init_summarization_model(ModelType::T5, 10);
    let input = [String::new(); 1];
    let _output = summarization_model.summarize(&input);
    let result = String::from(_output.join(" "));

    assert_ne!(result.to_string().len(), 0);
}

// #[test]
// fn test_longt5() {
//     let summarization_model = lib::init_summarization_model(ModelType::LongT5, 10);
//     let input = [String::new(); 1];
//     let _output = summarization_model.summarize(&input);
//     let result = String::from(_output.join(" "));
//
//     assert_ne!(result.to_string().len(), 0);
// }
//
// #[test]
// fn test_prophetNet() {
//     let summarization_model = lib::init_summarization_model(ModelType::ProphetNet, 10);
//     let input = [String::new(); 1];
//     let _output = summarization_model.summarize(&input);
//     let result = String::from(_output.join(" "));
//
//     assert_ne!(result.to_string().len(), 0);
// }


