// SynapseNet AI - Embeddings and consequence analysis

pub mod consequence;
pub mod embed;
pub mod model_manager;
pub mod onnx_embed;

pub use consequence::ConsequenceAnalyzer;
pub use embed::EmbeddingModel;
pub use model_manager::{ModelInfo, ModelManager, ALL_MINILM_L6_V2};
pub use onnx_embed::OnnxEmbedding;
