use anyhow::Result;
use arrow::array::{
    Array, ArrayRef, BinaryArray, Float32Array, Int64Array, ListArray, StringArray,
};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_writer::ArrowWriter;
use parquet::file::properties::WriterProperties;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use synapsenet_core::Grain;
use tracing::{info, warn};

/// Batch size for Parquet export (10,000 grains per file)
const BATCH_SIZE: usize = 10_000;

/// Parquet exporter for grains
pub struct ParquetExporter {
    output_dir: String,
    batch_size: usize,
}

impl ParquetExporter {
    /// Create new Parquet exporter
    pub fn new(output_dir: String) -> Self {
        Self {
            output_dir,
            batch_size: BATCH_SIZE,
        }
    }

    /// Export grains to Parquet files
    pub fn export(&self, grains: Vec<Grain>) -> Result<ExportStats> {
        info!("Starting Parquet export of {} grains", grains.len());

        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)?;

        let mut stats = ExportStats {
            total_grains: grains.len(),
            files_created: 0,
            bytes_written: 0,
        };

        // Process grains in batches
        for (batch_idx, batch) in grains.chunks(self.batch_size).enumerate() {
            let file_path = format!("{}/grains_{:04}.parquet", self.output_dir, batch_idx);
            let bytes = self.write_batch(batch, &file_path)?;

            stats.files_created += 1;
            stats.bytes_written += bytes;

            info!(
                "Wrote batch {} ({} grains) to {}",
                batch_idx,
                batch.len(),
                file_path
            );
        }

        info!(
            "Export complete: {} files, {} bytes",
            stats.files_created, stats.bytes_written
        );

        Ok(stats)
    }

    /// Write a batch of grains to a Parquet file
    fn write_batch(&self, grains: &[Grain], file_path: &str) -> Result<u64> {
        // Create Arrow schema
        let schema = create_grain_schema();

        // Convert grains to Arrow arrays
        let batch = grains_to_record_batch(grains, schema.clone())?;

        // Create Parquet writer with Snappy compression
        let file = File::create(file_path)?;
        let props = WriterProperties::builder()
            .set_compression(parquet::basic::Compression::SNAPPY)
            .build();

        let mut writer = ArrowWriter::try_new(file, schema, Some(props))?;

        // Write batch
        writer.write(&batch)?;
        writer.close()?;

        // Get file size
        let metadata = std::fs::metadata(file_path)?;
        Ok(metadata.len())
    }
}

/// Parquet importer for grains
pub struct ParquetImporter {
    input_dir: String,
}

impl ParquetImporter {
    /// Create new Parquet importer
    pub fn new(input_dir: String) -> Self {
        Self { input_dir }
    }

    /// Import grains from Parquet files
    pub fn import(&self) -> Result<ImportStats> {
        info!("Starting Parquet import from {}", self.input_dir);

        let mut stats = ImportStats {
            total_grains: 0,
            imported: 0,
            skipped: 0,
            invalid: 0,
        };

        // Find all Parquet files
        let files = self.find_parquet_files()?;

        if files.is_empty() {
            warn!("No Parquet files found in {}", self.input_dir);
            return Ok(stats);
        }

        info!("Found {} Parquet files", files.len());

        // Process each file
        for file_path in files {
            let batch_stats = self.import_file(&file_path)?;
            stats.total_grains += batch_stats.total_grains;
            stats.imported += batch_stats.imported;
            stats.skipped += batch_stats.skipped;
            stats.invalid += batch_stats.invalid;

            info!(
                "Imported {} from {} ({} imported, {} skipped, {} invalid)",
                file_path.display(),
                batch_stats.total_grains,
                batch_stats.imported,
                batch_stats.skipped,
                batch_stats.invalid
            );
        }

        info!(
            "Import complete: {} total, {} imported, {} skipped, {} invalid",
            stats.total_grains, stats.imported, stats.skipped, stats.invalid
        );

        Ok(stats)
    }

    /// Import grains from a single Parquet file
    fn import_file(&self, file_path: &Path) -> Result<ImportStats> {
        let file = File::open(file_path)?;
        let reader = parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder::try_new(file)?
            .build()?;

        let mut stats = ImportStats {
            total_grains: 0,
            imported: 0,
            skipped: 0,
            invalid: 0,
        };

        for batch_result in reader {
            let batch = batch_result?;
            let grains = record_batch_to_grains(&batch)?;

            stats.total_grains += grains.len();

            for grain in grains {
                // Verify signature
                match grain.verify() {
                    Ok(true) => {
                        // TODO: Store grain in database
                        // For now, just count as imported
                        stats.imported += 1;
                    }
                    Ok(false) => {
                        warn!(
                            "Invalid signature for grain {:?}",
                            hex_encode(&grain.id[..8])
                        );
                        stats.invalid += 1;
                    }
                    Err(e) => {
                        warn!("Error verifying grain: {}", e);
                        stats.invalid += 1;
                    }
                }
            }
        }

        Ok(stats)
    }

    /// Find all Parquet files in the input directory
    fn find_parquet_files(&self) -> Result<Vec<std::path::PathBuf>> {
        let mut files = Vec::new();

        for entry in std::fs::read_dir(&self.input_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "parquet" {
                        files.push(path);
                    }
                }
            }
        }

        files.sort();
        Ok(files)
    }
}

/// Export statistics
#[derive(Debug, Clone)]
pub struct ExportStats {
    pub total_grains: usize,
    pub files_created: usize,
    pub bytes_written: u64,
}

/// Import statistics
#[derive(Debug, Clone)]
pub struct ImportStats {
    pub total_grains: usize,
    pub imported: usize,
    pub skipped: usize,
    pub invalid: usize,
}

/// Create Arrow schema for grains
fn create_grain_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("id", DataType::Binary, false),
        Field::new(
            "vec",
            DataType::List(Arc::new(Field::new("item", DataType::Float32, true))),
            false,
        ),
        Field::new("author_pk", DataType::Binary, false),
        Field::new("crypto_backend", DataType::Utf8, false), // "Classical" or "PostQuantum"
        Field::new("ts_unix_ms", DataType::Int64, false),
        Field::new(
            "tags",
            DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))),
            false,
        ),
        Field::new("mime", DataType::Utf8, false),
        Field::new("lang", DataType::Utf8, false),
        Field::new("title", DataType::Utf8, true),
        Field::new("summary", DataType::Utf8, true),
        Field::new("sig", DataType::Binary, false),
    ]))
}

/// Convert grains to Arrow RecordBatch
fn grains_to_record_batch(grains: &[Grain], schema: Arc<Schema>) -> Result<RecordBatch> {
    let mut ids: Vec<Vec<u8>> = Vec::new();
    let mut vecs = Vec::new();
    let mut author_pks: Vec<Vec<u8>> = Vec::new();
    let mut crypto_backends = Vec::new();
    let mut timestamps = Vec::new();
    let mut tags_list = Vec::new();
    let mut mimes = Vec::new();
    let mut langs = Vec::new();
    let mut titles = Vec::new();
    let mut summaries = Vec::new();
    let mut sigs: Vec<Vec<u8>> = Vec::new();

    for grain in grains {
        ids.push(grain.id.to_vec());
        vecs.push(grain.vec.clone());
        author_pks.push(grain.meta.author_pk.clone());
        crypto_backends.push(format!("{:?}", grain.meta.crypto_backend));
        timestamps.push(grain.meta.ts_unix_ms);
        tags_list.push(grain.meta.tags.clone());
        mimes.push(grain.meta.mime.clone());
        langs.push(grain.meta.lang.clone());
        titles.push(grain.meta.title.clone());
        summaries.push(grain.meta.summary.clone());
        sigs.push(grain.sig.clone());
    }

    // Create Arrow arrays
    let id_refs: Vec<&[u8]> = ids.iter().map(|v| v.as_slice()).collect();
    let author_pk_refs: Vec<&[u8]> = author_pks.iter().map(|v| v.as_slice()).collect();
    let sig_refs: Vec<&[u8]> = sigs.iter().map(|v| v.as_slice()).collect();

    let id_array = BinaryArray::from_vec(id_refs);
    let vec_array = create_float_list_array(vecs);
    let author_pk_array = BinaryArray::from_vec(author_pk_refs);
    let crypto_backend_array = StringArray::from(crypto_backends);
    let ts_array = Int64Array::from(timestamps);
    let tags_array = create_string_list_array(tags_list);
    let mime_array = StringArray::from(mimes);
    let lang_array = StringArray::from(langs);
    let title_array = StringArray::from(titles);
    let summary_array = StringArray::from(summaries);
    let sig_array = BinaryArray::from_vec(sig_refs);

    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(id_array) as ArrayRef,
            Arc::new(vec_array) as ArrayRef,
            Arc::new(author_pk_array) as ArrayRef,
            Arc::new(crypto_backend_array) as ArrayRef,
            Arc::new(ts_array) as ArrayRef,
            Arc::new(tags_array) as ArrayRef,
            Arc::new(mime_array) as ArrayRef,
            Arc::new(lang_array) as ArrayRef,
            Arc::new(title_array) as ArrayRef,
            Arc::new(summary_array) as ArrayRef,
            Arc::new(sig_array) as ArrayRef,
        ],
    )?;

    Ok(batch)
}

/// Convert Arrow RecordBatch to grains
fn record_batch_to_grains(batch: &RecordBatch) -> Result<Vec<Grain>> {
    let mut grains = Vec::new();

    let num_rows = batch.num_rows();

    // Extract columns
    let id_array = batch
        .column(0)
        .as_any()
        .downcast_ref::<BinaryArray>()
        .unwrap();
    let vec_array = batch
        .column(1)
        .as_any()
        .downcast_ref::<ListArray>()
        .unwrap();
    let author_pk_array = batch
        .column(2)
        .as_any()
        .downcast_ref::<BinaryArray>()
        .unwrap();
    let crypto_backend_array = batch
        .column(3)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();
    let ts_array = batch
        .column(4)
        .as_any()
        .downcast_ref::<Int64Array>()
        .unwrap();
    let tags_array = batch
        .column(5)
        .as_any()
        .downcast_ref::<ListArray>()
        .unwrap();
    let mime_array = batch
        .column(6)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();
    let lang_array = batch
        .column(7)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();
    let title_array = batch
        .column(8)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();
    let summary_array = batch
        .column(9)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();
    let sig_array = batch
        .column(10)
        .as_any()
        .downcast_ref::<BinaryArray>()
        .unwrap();

    for i in 0..num_rows {
        let id: [u8; 32] = id_array.value(i).try_into()?;
        let vec = extract_float_list(vec_array, i);
        let author_pk = author_pk_array.value(i).to_vec(); // Now Vec<u8> to support variable length
        let crypto_backend_str = crypto_backend_array.value(i);
        let crypto_backend = match crypto_backend_str {
            "Classical" => synapsenet_core::CryptoBackend::Classical,
            "PostQuantum" => synapsenet_core::CryptoBackend::PostQuantum,
            _ => {
                // Fallback: detect from public key length
                if author_pk.len() == 32 {
                    synapsenet_core::CryptoBackend::Classical
                } else {
                    synapsenet_core::CryptoBackend::PostQuantum
                }
            }
        };
        let ts_unix_ms = ts_array.value(i);
        let tags = extract_string_list(tags_array, i);
        let mime = mime_array.value(i).to_string();
        let lang = lang_array.value(i).to_string();
        let title = if title_array.is_null(i) {
            None
        } else {
            Some(title_array.value(i).to_string())
        };
        let summary = if summary_array.is_null(i) {
            None
        } else {
            Some(summary_array.value(i).to_string())
        };
        let sig = sig_array.value(i).to_vec();

        let grain = Grain {
            id,
            vec,
            meta: synapsenet_core::GrainMeta {
                author_pk,
                crypto_backend,
                ts_unix_ms,
                tags,
                mime,
                lang,
                title,
                summary,
                embedding_model: None, // Legacy data doesn't have this
                embedding_dimensions: None,
            },
            sig,
        };

        grains.push(grain);
    }

    Ok(grains)
}

/// Create Arrow ListArray from Vec<Vec<f32>>
fn create_float_list_array(data: Vec<Vec<f32>>) -> ListArray {
    use arrow::array::Float32Builder;
    use arrow::array::ListBuilder;

    let values_builder = Float32Builder::new();
    let mut builder = ListBuilder::new(values_builder);

    for vec in data {
        for &value in &vec {
            builder.values().append_value(value);
        }
        builder.append(true);
    }

    builder.finish()
}

/// Create Arrow ListArray from Vec<Vec<String>>
fn create_string_list_array(data: Vec<Vec<String>>) -> ListArray {
    use arrow::array::ListBuilder;
    use arrow::array::StringBuilder;

    let values_builder = StringBuilder::new();
    let mut builder = ListBuilder::new(values_builder);

    for vec in data {
        for value in &vec {
            builder.values().append_value(value);
        }
        builder.append(true);
    }

    builder.finish()
}

/// Extract Vec<f32> from ListArray at index
fn extract_float_list(array: &ListArray, index: usize) -> Vec<f32> {
    let values = array.values();
    let float_array = values.as_any().downcast_ref::<Float32Array>().unwrap();

    let start = array.value_offsets()[index] as usize;
    let end = array.value_offsets()[index + 1] as usize;

    (start..end).map(|i| float_array.value(i)).collect()
}

/// Extract Vec<String> from ListArray at index
fn extract_string_list(array: &ListArray, index: usize) -> Vec<String> {
    let values = array.values();
    let string_array = values.as_any().downcast_ref::<StringArray>().unwrap();

    let start = array.value_offsets()[index] as usize;
    let end = array.value_offsets()[index + 1] as usize;

    (start..end)
        .map(|i| string_array.value(i).to_string())
        .collect()
}

/// Helper for hex encoding
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
