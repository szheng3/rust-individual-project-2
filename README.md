[![Tests](https://github.com/szheng3/rust-individual-project-2/actions/workflows/tests.yml/badge.svg)](https://github.com/szheng3/rust-individual-project-2/actions/workflows/tests.yml)
[![Build binary release](https://github.com/szheng3/rust-individual-project-2/actions/workflows/release.yml/badge.svg)](https://github.com/szheng3/rust-individual-project-2/actions/workflows/release.yml)
[![Clippy](https://github.com/szheng3/rust-individual-project-2/actions/workflows/lint.yml/badge.svg)](https://github.com/szheng3/rust-individual-project-2/actions/workflows/lint.yml)
[![Rustfmt](https://github.com/szheng3/rust-individual-project-2/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/szheng3/rust-individual-project-2/actions/workflows/rustfmt.yml)
[![publish to Dockerhub](https://github.com/szheng3/rust-individual-project-2/actions/workflows/publish.yml/badge.svg)](https://github.com/szheng3/rust-individual-project-2/actions/workflows/publish.yml)
[![Benchmark](https://github.com/szheng3/rust-individual-project-2/actions/workflows/bench.yml/badge.svg)](https://github.com/szheng3/rust-individual-project-2/actions/workflows/bench.yml)

# Individual Project 2 - Rust micro-server for Text Summarization

This project aims to build a Rust micro-server that summarizes text, based on the common task of reading and summarizing books among students. The project uses the `rust actix` and `libtorch` to run a pre-trained `hugging-face` model for summarization.

## Architectural Diagram


## Project Goals/Outcomes

* Develop my Rust micro-service 
* Use Github Codespaces and Copilot
* Integrate libtorch and 'hugging-face pretrained models' into a Rust Cli project

## Setup

1. Install rust via [rustup](https://rustup.rs/)
2. Install the libtorch (for Mac M1), Intel chips users can skip this step
```
brew install pytorch@1.13.1
```


## Not Mac ARM chips users

* Run, you can pass any text as the parameter at the end of the command. See below.
```
make run 
```

* Release
```
make releasex86
```

* Bench
```
make benchx86
```

## Mac ARM chips users
* change the path in the Makefile to your libtorch path
```
export LIBTORCH=/opt/homebrew/Cellar/pytorch/1.13.1 &&export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
```
* Run, you can pass any text as the parameter at the end of the command. See below.
```
make runarm 
```
* Release
```
make release
```
* Bench
```
make bench
```



## CI/CD

Github Actions configured in .github/workflows

## Docker

* This repo main branch is automatically published to Dockerhub with [CI/CD](https://github.com/szheng3/rust-individual-project-2/actions/workflows/publish.yml), you can pull the image from [here](https://hub.docker.com/repository/docker/szheng3/sz-rust-ml-cli/general)
```
docker pull szheng3/sz-rust-ml-cli:latest
```
* Run the docker image, you can pass any text as the parameter at the end of the command. See below.
```
docker run szheng3/sz-rust-ml-cli:latest 'The Chinese monarchy collapsed in 1912 with the Xinhai Revolution, when the Republic of China (ROC) replaced the Qing dynasty. In its early years as a republic, the country underwent a period of instability known as the \"Warlord Era\" before mostly reunifying in 1928 under a Nationalist government. A civil war between the nationalist Kuomintang (KMT) and the Chinese Communist Party (CCP) began in 1927. Japan invaded China in 1937, starting the Second Sino-Japanese War and temporarily halting the civil war. The surrender and expulsion of Japanese forces from China in 1945 left a power vacuum in the country, which led to renewed fighting between the CCP and the Kuomintang.'
```

## GitHub releases
The binary could be downloaded from the release pages. [release](https://github.com/szheng3/rust-individual-project-2/releases)

## Benchmark Results
![Benchmark](./assets/report.png)

## Progress Log

- [x] Configure Github Codespaces.
- [x] Initialise Rust project with pretrained model from [hugging-face](https://huggingface.co/transformers/model_doc/bart.html)
- [x] CI/CD with Github Actions
- [x] Tag and Releases
- [x] Benchmark

## References


* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [rust-bert](https://github.com/guillaume-be/rust-bert)