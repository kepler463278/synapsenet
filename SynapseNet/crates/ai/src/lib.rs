// SynapseNet AI - Embeddings and consequence analysis

pub mod batch;
pub mod consequence;
pub mod embed;
pub mod gpu_providers;
pub mod model_manager;
pub mod multi_model;
pub mod onnx_embed;

pub use batch::{
    BatchConfig, BatchProcessor, BatchProgress, BatchResult, DocumentParser, MarkdownParser,
    PlainTextParser, JsonParser, SupportedFormat,
};
pub use consequence::ConsequenceAnalyzer;
pub use embed::EmbeddingModel;
pub use gpu_providers::GpuProvider;
pub use model_manager::{ModelInfo as ModelManagerInfo, ModelManager, ALL_MINILM_L6_V2};
pub use multi_model::{ModelInfo, ModelSize, MultiModelManager};
pub use onnx_embed::OnnxEmbedding;
