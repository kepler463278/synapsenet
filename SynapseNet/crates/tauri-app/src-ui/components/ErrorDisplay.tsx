import { ErrorInfo } from '../hooks/useErrorHandler';

interface Props {
  error: ErrorInfo;
  onRetry?: () => void;
  onDismiss?: () => void;
  showDetails?: boolean;
}

export default function ErrorDisplay({ error, onRetry, onDismiss, showDetails = false }: Props) {
  const getIcon = () => {
    switch (error.type) {
      case 'network':
        return '🌐';
      case 'embedding':
        return '🤖';
      case 'storage':
        return '💾';
      case 'validation':
        return '⚠️';
      default:
        return '❌';
    }
  };

  const getColorClass = () => {
    switch (error.type) {
      case 'network':
        return 'error-network';
      case 'embedding':
        return 'error-embedding';
      case 'storage':
        return 'error-storage';
      case 'validation':
        return 'error-validation';
      default:
        return 'error-unknown';
    }
  };

  return (
    <div className={`error-display ${getColorClass()}`}>
      <div className="error-header">
        <span className="error-icon">{getIcon()}</span>
        <span className="error-message">{error.message}</span>
        {onDismiss && (
          <button className="error-dismiss" onClick={onDismiss} aria-label="Dismiss">
            ✕
          </button>
        )}
      </div>

      {showDetails && error.details && (
        <details className="error-details">
          <summary>Show technical details</summary>
          <pre>{error.details}</pre>
        </details>
      )}

      {error.retryable && onRetry && (
        <div className="error-actions">
          <button onClick={onRetry} className="btn-retry">
            🔄 Retry
          </button>
        </div>
      )}
    </div>
  );
}
