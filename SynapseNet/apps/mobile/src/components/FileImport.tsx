import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface FileInfo {
  name: string;
  size: number;
  mime_type: string;
}

interface ImportResult {
  success: boolean;
  files_processed: number;
  grains_created: number;
  errors: string[];
}

interface FileImportProps {
  onImportComplete?: (result: ImportResult) => void;
}

function FileImport({ onImportComplete }: FileImportProps) {
  const [selectedFiles, setSelectedFiles] = useState<string[]>([]);
  const [fileInfos, setFileInfos] = useState<FileInfo[]>([]);
  const [importing, setImporting] = useState(false);
  const [result, setResult] = useState<ImportResult | null>(null);

  const handlePickFiles = async () => {
    try {
      const files = await invoke<string[]>('pick_files');
      setSelectedFiles(files);
      
      // Get file info for each selected file
      const infos = await Promise.all(
        files.map(async (path) => {
          try {
            return await invoke<FileInfo>('get_file_info', { path });
          } catch (error) {
            console.error(`Failed to get info for ${path}:`, error);
            return {
              name: path.split('/').pop() || 'unknown',
              size: 0,
              mime_type: 'unknown',
            };
          }
        })
      );
      
      setFileInfos(infos);
    } catch (error) {
      console.error('Failed to pick files:', error);
      alert('Failed to open file picker');
    }
  };

  const handleImport = async () => {
    if (selectedFiles.length === 0) return;
    
    setImporting(true);
    setResult(null);
    
    try {
      const importResult = await invoke<ImportResult>('import_files', {
        filePaths: selectedFiles,
      });
      
      setResult(importResult);
      onImportComplete?.(importResult);
      
      if (importResult.success) {
        setSelectedFiles([]);
        setFileInfos([]);
      }
    } catch (error) {
      console.error('Import failed:', error);
      setResult({
        success: false,
        files_processed: 0,
        grains_created: 0,
        errors: [String(error)],
      });
    } finally {
      setImporting(false);
    }
  };

  const formatFileSize = (bytes: number) => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const getSupportedTypes = () => {
    return ['Text (.txt)', 'Markdown (.md)', 'JSON (.json)', 'CSV (.csv)'];
  };

  return (
    <div className="file-import">
      <div className="import-header">
        <h3>📁 Import Files</h3>
        <p>Import documents to add to your knowledge base</p>
      </div>

      <div className="supported-types">
        <h4>Supported formats:</h4>
        <div className="type-list">
          {getSupportedTypes().map((type) => (
            <span key={type} className="type-tag">{type}</span>
          ))}
        </div>
      </div>

      <button 
        className="btn-primary"
        onClick={handlePickFiles}
        disabled={importing}
      >
        📂 Select Files
      </button>

      {selectedFiles.length > 0 && (
        <div className="selected-files">
          <h4>Selected Files ({selectedFiles.length}):</h4>
          <div className="file-list">
            {fileInfos.map((info, index) => (
              <div key={index} className="file-item">
                <div className="file-icon">
                  {info.mime_type.startsWith('text/') ? '📄' : '📋'}
                </div>
                <div className="file-details">
                  <div className="file-name">{info.name}</div>
                  <div className="file-meta">
                    {formatFileSize(info.size)} • {info.mime_type}
                  </div>
                </div>
              </div>
            ))}
          </div>

          <button
            className="btn-primary"
            onClick={handleImport}
            disabled={importing}
          >
            {importing ? 'Importing...' : `Import ${selectedFiles.length} file(s)`}
          </button>
        </div>
      )}

      {result && (
        <div className={`import-result ${result.success ? 'success' : 'error'}`}>
          <h4>{result.success ? '✅ Import Complete' : '❌ Import Failed'}</h4>
          <div className="result-stats">
            <p>Files processed: {result.files_processed}</p>
            <p>Grains created: {result.grains_created}</p>
          </div>
          
          {result.errors.length > 0 && (
            <div className="errors">
              <h5>Errors:</h5>
              <ul>
                {result.errors.map((error, index) => (
                  <li key={index}>{error}</li>
                ))}
              </ul>
            </div>
          )}
        </div>
      )}
    </div>
  );
}

export default FileImport;
