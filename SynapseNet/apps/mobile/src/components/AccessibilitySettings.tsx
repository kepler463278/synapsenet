import { useEffect, useState } from 'react';
import { useAccessibility } from '../hooks/useAccessibility';

function AccessibilitySettings() {
  const { info, isScreenReaderRunning, announce } = useAccessibility();
  const [textSize, setTextSize] = useState<'small' | 'medium' | 'large'>('medium');
  const [highContrast, setHighContrast] = useState(false);
  const [reduceMotion, setReduceMotion] = useState(false);

  useEffect(() => {
    if (info) {
      // Apply system preferences
      if (info.is_reduce_motion_enabled) {
        setReduceMotion(true);
        document.body.classList.add('reduce-motion');
      }
      
      // Detect preferred text size
      if (info.preferred_content_size.includes('Large')) {
        setTextSize('large');
        document.body.classList.add('large-text');
      }
    }
  }, [info]);

  const handleTextSizeChange = (size: 'small' | 'medium' | 'large') => {
    setTextSize(size);
    document.body.classList.remove('small-text', 'medium-text', 'large-text');
    document.body.classList.add(`${size}-text`);
    announce(`Text size changed to ${size}`);
  };

  const handleHighContrastToggle = (enabled: boolean) => {
    setHighContrast(enabled);
    if (enabled) {
      document.body.classList.add('high-contrast');
    } else {
      document.body.classList.remove('high-contrast');
    }
    announce(`High contrast mode ${enabled ? 'enabled' : 'disabled'}`);
  };

  const handleReduceMotionToggle = (enabled: boolean) => {
    setReduceMotion(enabled);
    if (enabled) {
      document.body.classList.add('reduce-motion');
    } else {
      document.body.classList.remove('reduce-motion');
    }
    announce(`Reduce motion ${enabled ? 'enabled' : 'disabled'}`);
  };

  return (
    <div className="accessibility-settings">
      <div className="settings-header">
        <h3>♿ Accessibility</h3>
        <p>Customize your experience</p>
      </div>

      {isScreenReaderRunning && (
        <div className="accessibility-banner">
          <p>✅ Screen reader detected</p>
          <p className="hint">Enhanced accessibility features are active</p>
        </div>
      )}

      <div className="setting-group">
        <div className="setting-item">
          <div>
            <div className="setting-label">Text Size</div>
            <div className="setting-desc">Adjust text size for better readability</div>
          </div>
          <div className="text-size-buttons">
            <button
              className={`size-btn ${textSize === 'small' ? 'active' : ''}`}
              onClick={() => handleTextSizeChange('small')}
              aria-label="Small text size"
              aria-pressed={textSize === 'small'}
            >
              A
            </button>
            <button
              className={`size-btn ${textSize === 'medium' ? 'active' : ''}`}
              onClick={() => handleTextSizeChange('medium')}
              aria-label="Medium text size"
              aria-pressed={textSize === 'medium'}
            >
              A
            </button>
            <button
              className={`size-btn large ${textSize === 'large' ? 'active' : ''}`}
              onClick={() => handleTextSizeChange('large')}
              aria-label="Large text size"
              aria-pressed={textSize === 'large'}
            >
              A
            </button>
          </div>
        </div>

        <div className="setting-item">
          <div>
            <div className="setting-label">High Contrast</div>
            <div className="setting-desc">Increase contrast for better visibility</div>
          </div>
          <label className="toggle">
            <input
              type="checkbox"
              checked={highContrast}
              onChange={(e) => handleHighContrastToggle(e.target.checked)}
              aria-label="Toggle high contrast mode"
            />
            <span className="slider"></span>
          </label>
        </div>

        <div className="setting-item">
          <div>
            <div className="setting-label">Reduce Motion</div>
            <div className="setting-desc">Minimize animations and transitions</div>
          </div>
          <label className="toggle">
            <input
              type="checkbox"
              checked={reduceMotion}
              onChange={(e) => handleReduceMotionToggle(e.target.checked)}
              aria-label="Toggle reduce motion"
            />
            <span className="slider"></span>
          </label>
        </div>
      </div>

      {info && (
        <div className="accessibility-info">
          <h4>System Settings</h4>
          <ul>
            <li>
              <span>Screen Reader:</span>
              <span>{isScreenReaderRunning ? 'Active' : 'Inactive'}</span>
            </li>
            <li>
              <span>Bold Text:</span>
              <span>{info.is_bold_text_enabled ? 'Enabled' : 'Disabled'}</span>
            </li>
            <li>
              <span>System Motion:</span>
              <span>{info.is_reduce_motion_enabled ? 'Reduced' : 'Normal'}</span>
            </li>
          </ul>
        </div>
      )}
    </div>
  );
}

export default AccessibilitySettings;
