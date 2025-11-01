import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import SearchView from '../components/SearchView';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('SearchView', () => {
  it('renders the component', () => {
    render(<SearchView />);
    expect(screen.getByText(/Search Knowledge/i)).toBeInTheDocument();
  });

  it('allows search input', () => {
    render(<SearchView />);
    const searchInput = screen.getByPlaceholderText(/Search/i);
    
    fireEvent.change(searchInput, { target: { value: 'test query' } });
    expect(searchInput).toHaveValue('test query');
  });

  it('displays empty state when no results', () => {
    render(<SearchView />);
    expect(screen.getByText(/No results/i)).toBeInTheDocument();
  });
});
