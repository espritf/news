use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DTYPE};
use pgvector::Vector;
use tokenizers::tokenizer::Tokenizer;
use crate::app::VectorProvider;

pub struct Model {
    model: BertModel,
    tokenizer: Tokenizer,
}

impl Model {
    pub fn build(config: &str, tokenizer: &str, weights: &str) -> Result<Self> {
        let config = std::fs::read_to_string(config)?;
        let config: Config = serde_json::from_str(&config)?;

        let tokenizer = Tokenizer::from_file(tokenizer).map_err(E::msg)?;
        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights], DTYPE, &Device::Cpu)? };

        let model = BertModel::load(vb, &config)?;

        Ok(Self { model, tokenizer })
    }
    fn forward(&self, input: &str) -> Result<Tensor> {
        let tokens = self
            .tokenizer
            .encode(input, true)
            .map_err(E::msg)?
            .get_ids()
            .to_vec();

        let token_ids = Tensor::new(&tokens[..], &self.model.device)
            .unwrap()
            .unsqueeze(0)
            .unwrap();
        let token_type_ids = token_ids.zeros_like().unwrap();

        let ys = self.model.forward(&token_ids, &token_type_ids).unwrap();

        Ok(ys)
    }
}

impl VectorProvider for Model {
    fn vector(&self, input: &str) -> Result<Vector> {
        let embeddings = self.forward(input)?;
        let (_, n_tokens, _) = embeddings.dims3().unwrap();
        let embeddings = (embeddings.sum(1).unwrap() / (n_tokens as f64)).unwrap();

        tracing::info!("pooled embeddings shape: {:?}", embeddings.shape());

        let v = Vector::from(embeddings.get(0).unwrap().to_vec1::<f32>().unwrap());

        Ok(v)
    }
}
