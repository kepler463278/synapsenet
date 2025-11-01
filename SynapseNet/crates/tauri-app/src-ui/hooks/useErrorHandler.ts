import { useState, useCallback } from 'react';

export interface ErrorInfo {
  message: string;
  type: 'network' | 'embedding' | 'storage' | 'validation' | 'unknown';
  retryable: boolean;
  details?: string;
}

export function useErrorHandler() {
  const [error, setError] = useState<ErrorInfo | null>(null);

  const handleError = useCallback((err: any) => {
    console.error('Error occurred:', err);

    let errorInfo: ErrorInfo = {
      message: 'An unexpected error occurred',
      type: 'unknown',
      retryable: false,
    };

    if (typeof err === 'string') {
      errorInfo.message = err;
      errorInfo.details = err;
    } else if (err instanceof Error) {
      errorInfo.message = err.message;
      errorInfo.details = err.stack;
    } else if (err && typeof err === 'object') {
      // Parse Tauri error format
      const errStr = err.toString().toLowerCase();

      if (errStr.includes('network') || errStr.includes('connection')) {
        errorInfo.type = 'network';
        errorInfo.retryable = true;
        errorInfo.message = 'Network error. Please check your connection.';
      } else if (errStr.includes('embedding') || errStr.includes('model')) {
        errorInfo.type = 'embedding';
        errorInfo.retryable = true;
        errorInfo.message = 'AI model error. Trying fallback...';
      } else if (errStr.includes('storage') || errStr.includes('database')) {
        errorInfo.type = 'storage';
        errorInfo.retryable = false;
        errorInfo.message = 'Storage error. Please check disk space.';
      } else if (errStr.includes('invalid') || errStr.includes('validation')) {
        errorInfo.type = 'validation';
        errorInfo.retryable = false;
        errorInfo.message = 'Invalid input. Please check your data.';
      }

      errorInfo.details = JSON.stringify(err, null, 2);
    }

    setError(errorInfo);
  }, []);

  const clearError = useCallback(() => {
    setError(null);
  }, []);

  const retry = useCallback((operation: () => Promise<void>) => {
    clearError();
    return operation().catch(handleError);
  }, [clearError, handleError]);

  return {
    error,
    handleError,
    clearError,
    retry,
  };
}

export default useErrorHandler;
