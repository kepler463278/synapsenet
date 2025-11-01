# Requirements Document - SynapseNet v0.4

## Introduction

SynapseNet v0.4 represents the transformation from a developer-focused CLI tool into a mass-market application accessible to ordinary users. This release introduces a native Web UI built with Tauri, multi-model AI support, global P2P mesh networking, batch knowledge processing, and an enhanced Proof-of-Effort (PoE) economic model. The goal is to enable viral adoption by lowering the barrier to entry while maintaining the core principles of decentralization, privacy, and quantum-resistant security.

## Glossary

- **SynapseNet**: The decentralized semantic memory network system
- **Web UI**: The Tauri-based graphical user interface application
- **Knowledge Grain**: A unit of semantic information stored in the network (also called "grain" or "zerno")
- **Embedding Model**: An AI model that converts text into semantic vector representations
- **PoE (Proof-of-Effort)**: The economic mechanism that rewards nodes for contributing valuable knowledge
- **NGT Token**: The native token used to reward knowledge contributions
- **DHT (Distributed Hash Table)**: A decentralized system for peer discovery across the internet
- **Batch Processing**: The ability to import and process large volumes of knowledge at once
- **P2P Mesh**: The peer-to-peer network topology connecting SynapseNet nodes
- **NAT Hole-Punching**: A technique to establish direct connections between nodes behind routers
- **mDNS**: Multicast DNS for local network peer discovery
- **Bootstrap Peer**: A manually configured initial peer for network entry
- **Semantic Search**: Search based on meaning rather than exact keyword matching
- **Novelty Score**: A metric measuring how unique a knowledge grain is
- **Coherence Score**: A metric measuring logical connections between knowledge grains
- **Reuse Score**: A metric measuring how often a knowledge grain helps other users

## Requirements

### Requirement 1: Native Web UI Application

**User Story:** As a non-technical user, I want a graphical interface to interact with SynapseNet, so that I can use semantic memory without learning command-line tools.

#### Acceptance Criteria

1. WHEN the user launches the application, THE Web UI SHALL display a main window with navigation to add, search, and view knowledge grains
2. THE Web UI SHALL run as a native desktop application using Tauri with Rust backend and TypeScript frontend
3. THE Web UI SHALL provide a text input interface for adding new knowledge grains with a single action
4. THE Web UI SHALL provide a semantic search interface that accepts natural language queries
5. THE Web UI SHALL display search results with relevance scores and source attribution for each knowledge grain
6. THE Web UI SHALL visualize connections between related knowledge grains in an interactive graph view
7. THE Web UI SHALL operate entirely locally without requiring cloud services or external servers

### Requirement 2: Multi-Model AI Support

**User Story:** As a node operator, I want to use different embedding models based on my hardware capabilities, so that the network can accommodate devices from phones to servers.

#### Acceptance Criteria

1. THE SynapseNet SHALL support concurrent operation with multiple embedding models of different sizes
2. THE SynapseNet SHALL categorize embedding models into small (MiniLM), medium (BERT, Nomic, E5), and large (LLaMA, Mistral) categories
3. WHEN a node queries the network, THE SynapseNet SHALL automatically select the most capable embedding model available from connected peers
4. THE SynapseNet SHALL allow users to configure which embedding models to load based on available hardware resources
5. THE SynapseNet SHALL maintain compatibility between knowledge grains created with different embedding models through vector space alignment

### Requirement 3: Global P2P Mesh Discovery

**User Story:** As a network participant, I want my node to automatically discover and connect to peers across the internet, so that I can access the global knowledge network without manual configuration.

#### Acceptance Criteria

1. THE SynapseNet SHALL implement DHT-based peer discovery for finding nodes across the internet
2. THE SynapseNet SHALL implement NAT hole-punching to establish direct connections between nodes behind routers
3. THE SynapseNet SHALL maintain backward compatibility with existing mDNS local network discovery
4. THE SynapseNet SHALL maintain backward compatibility with manually configured bootstrap peers
5. THE SynapseNet SHALL automatically cluster peers based on shared knowledge domains or topics
6. WHEN a node joins the network, THE SynapseNet SHALL discover and connect to at least 3 peers within 30 seconds

### Requirement 4: Batch Knowledge Processing

**User Story:** As a power user, I want to import large volumes of knowledge at once, so that I can quickly populate my semantic memory with books, documents, or chat histories.

#### Acceptance Criteria

1. THE SynapseNet SHALL provide a batch import command that accepts directories or file collections as input
2. THE SynapseNet SHALL support batch processing of common formats including TXT, PDF, Markdown, and JSON
3. WHEN batch processing is initiated with GPU acceleration enabled, THE SynapseNet SHALL utilize GPU resources for embedding generation
4. THE SynapseNet SHALL provide a batch push command that distributes processed knowledge grains to the network
5. THE SynapseNet SHALL display progress indicators showing the number of items processed and estimated time remaining during batch operations
6. THE SynapseNet SHALL process at least 100 documents per minute on hardware with GPU acceleration

### Requirement 5: Enhanced PoE Economic Model

**User Story:** As a knowledge contributor, I want to earn rewards based on the novelty, coherence, and usefulness of my contributions, so that valuable knowledge is properly incentivized.

#### Acceptance Criteria

1. THE SynapseNet SHALL calculate PoE rewards using three components: novelty score, coherence score, and reuse score
2. THE SynapseNet SHALL measure novelty score by comparing new knowledge grains against existing network knowledge using semantic distance
3. THE SynapseNet SHALL measure coherence score by analyzing logical connections and contextual relationships between knowledge grains
4. THE SynapseNet SHALL measure reuse score by tracking how many times a knowledge grain is retrieved or referenced by other users
5. THE SynapseNet SHALL distribute NGT tokens as rewards based on the combined PoE score calculation
6. THE SynapseNet SHALL prevent reward gaming by penalizing duplicate or low-quality knowledge submissions
7. THE SynapseNet SHALL update reuse scores asynchronously as knowledge grains are accessed across the network

### Requirement 6: Cross-Platform Compatibility

**User Story:** As a user on any major operating system, I want to run SynapseNet with the Web UI, so that I can participate regardless of my platform choice.

#### Acceptance Criteria

1. THE Web UI SHALL run on Windows, macOS, and Linux operating systems
2. THE Web UI SHALL provide consistent functionality across all supported platforms
3. THE Web UI SHALL use native system APIs through Tauri for file dialogs, notifications, and system tray integration
4. THE Web UI SHALL package as a single executable installer for each platform

### Requirement 7: Data Privacy and Local-First Architecture

**User Story:** As a privacy-conscious user, I want all my data to remain on my device unless I explicitly share it, so that I maintain full control over my knowledge.

#### Acceptance Criteria

1. THE SynapseNet SHALL store all user knowledge grains locally on the user's device
2. THE SynapseNet SHALL only transmit knowledge grains to the P2P network when the user explicitly initiates sharing
3. THE SynapseNet SHALL encrypt all network communications using the existing PQC (Post-Quantum Cryptography) implementation
4. THE Web UI SHALL clearly indicate which knowledge grains are local-only versus shared on the network
5. THE SynapseNet SHALL allow users to delete their knowledge grains from local storage at any time

### Requirement 8: Performance and Scalability

**User Story:** As a user with a large knowledge base, I want the system to remain responsive even with thousands of knowledge grains, so that performance doesn't degrade over time.

#### Acceptance Criteria

1. THE Web UI SHALL render the main interface within 2 seconds of application launch
2. THE SynapseNet SHALL return semantic search results within 500 milliseconds for queries against a local database of 10,000 knowledge grains
3. THE SynapseNet SHALL support local storage of at least 100,000 knowledge grains without performance degradation
4. THE Web UI SHALL remain responsive during background operations such as batch processing or network synchronization
5. THE SynapseNet SHALL limit memory usage to less than 1GB for typical workloads with medium-sized embedding models

### Requirement 9: User Onboarding and Documentation

**User Story:** As a new user, I want clear guidance on how to set up and use SynapseNet, so that I can start benefiting from semantic memory quickly.

#### Acceptance Criteria

1. THE Web UI SHALL display a first-run tutorial explaining core concepts when launched for the first time
2. THE SynapseNet SHALL provide example knowledge grains and sample queries during initial setup
3. THE Web UI SHALL include contextual help tooltips for all major features
4. THE SynapseNet SHALL include comprehensive documentation covering installation, configuration, and usage
5. THE SynapseNet SHALL provide troubleshooting guides for common issues

### Requirement 10: Backward Compatibility

**User Story:** As an existing v0.3 user, I want to upgrade to v0.4 without losing my data or breaking my workflows, so that I can adopt new features smoothly.

#### Acceptance Criteria

1. THE SynapseNet v0.4 SHALL read and migrate knowledge grains from v0.3 storage format
2. THE SynapseNet v0.4 SHALL maintain API compatibility with v0.3 REST endpoints
3. THE SynapseNet v0.4 SHALL support existing v0.3 configuration files with automatic migration to new format
4. THE SynapseNet v0.4 SHALL interoperate with v0.3 nodes on the P2P network during a transition period
