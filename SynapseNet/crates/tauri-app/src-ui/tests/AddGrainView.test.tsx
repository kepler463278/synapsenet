import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import AddGrainView from '../components/AddGrainView';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('AddGrainView', () => {
  it('renders the component', () => {
    render(<AddGrainView />);
    expect(screen.getByText(/Add Knowledge/i)).toBeInTheDocument();
  });

  it('allows text input', () => {
    render(<AddGrainView />);
    const textarea = screen.getByPlaceholderText(/Enter your knowledge/i);
    
    fireEvent.change(textarea, { target: { value: 'Test knowledge' } });
    expect(textarea).toHaveValue('Test knowledge');
  });

  it('allows tag input', () => {
    render(<AddGrainView />);
    const tagInput = screen.getByPlaceholderText(/Tags/i);
    
    fireEvent.change(tagInput, { target: { value: 'test, demo' } });
    expect(tagInput).toHaveValue('test, demo');
  });

  it('disables submit button when text is empty', () => {
    render(<AddGrainView />);
    const submitButton = screen.getByText(/Add Grain/i);
    
    expect(submitButton).toBeDisabled();
  });

  it('enables submit button when text is provided', () => {
    render(<AddGrainView />);
    const textarea = screen.getByPlaceholderText(/Enter your knowledge/i);
    const submitButton = screen.getByText(/Add Grain/i);
    
    fireEvent.change(textarea, { target: { value: 'Test knowledge' } });
    expect(submitButton).not.toBeDisabled();
  });
});
