# Requirements Document - SynapseNet v1.0 "Public Genesis"

## Introduction

SynapseNet v1.0 represents the transition from developer tool to human tool - the moment when decentralized intelligence becomes accessible to everyone without technical barriers. This release combines three pillars: accessibility (one-click node deployment), incentives (real-time NGT rewards), and philosophy (Genesis Manifest explaining our purpose).

## Glossary

- **SynapseNet Node**: The core software that participates in the decentralized knowledge network
- **Desktop GUI**: The Tauri-based graphical user interface for running and managing a node
- **Grain**: A unit of knowledge stored and shared in the network
- **NGT (Network Growth Token)**: The reward token earned through Proof-of-Emergence contributions
- **PoE (Proof-of-Emergence)**: The consensus mechanism that rewards valuable knowledge contributions
- **Genesis Manifest**: The philosophical foundation document explaining SynapseNet's purpose and principles
- **Peer**: Another node in the decentralized network
- **Swarm**: Collective intelligence mechanism for distributed consensus

## Requirements

### Requirement 1: One-Click Node Deployment

**User Story:** As a non-technical user, I want to start a SynapseNet node with a single click, so that I can participate in the network without command-line knowledge.

#### Acceptance Criteria

1. WHEN the user clicks "Start Node" button, THE Desktop GUI SHALL initialize and connect the SynapseNet Node to the P2P network within 30 seconds
2. WHILE the SynapseNet Node is starting, THE Desktop GUI SHALL display real-time progress indicators showing connection status
3. IF the SynapseNet Node fails to start, THEN THE Desktop GUI SHALL display a clear error message with troubleshooting steps
4. THE Desktop GUI SHALL persist the node state across application restarts
5. WHEN the SynapseNet Node is running, THE Desktop GUI SHALL display live network statistics including peer count and grain count

### Requirement 2: Real-Time Reward Visualization

**User Story:** As a network participant, I want to see my NGT rewards in real-time, so that I understand the value of my contributions immediately.

#### Acceptance Criteria

1. WHEN a user adds a novel grain, THE Desktop GUI SHALL display a reward notification showing the NGT amount earned within 2 seconds
2. THE Desktop GUI SHALL maintain a visible NGT balance counter that updates in real-time
3. WHEN a user's grain is validated by peers, THE Desktop GUI SHALL show a notification with the validation reward amount
4. THE Desktop GUI SHALL provide a reward history view showing all earned NGT with timestamps and reasons
5. WHILE the user participates in swarm consensus, THE Desktop GUI SHALL track and display participation rewards

### Requirement 3: Knowledge Search and Discovery

**User Story:** As a user, I want to search the knowledge network, so that I can find information stored across all nodes.

#### Acceptance Criteria

1. THE Desktop GUI SHALL provide a search interface that queries both local and network grains
2. WHEN a user enters a search query, THE Desktop GUI SHALL return relevant results within 3 seconds
3. THE Desktop GUI SHALL display search results with grain content, author, timestamp, and confidence score
4. THE Desktop GUI SHALL support filtering search results by confidence threshold
5. WHEN search results are displayed, THE Desktop GUI SHALL show the source (local or peer) of each grain

### Requirement 4: Content Contribution

**User Story:** As a knowledge contributor, I want to add my insights to the network, so that I can share knowledge and earn rewards.

#### Acceptance Criteria

1. THE Desktop GUI SHALL provide a simple text input interface for adding new grains
2. WHEN a user submits a grain, THE Desktop GUI SHALL validate the content is not empty before accepting
3. WHEN a grain is successfully added, THE Desktop GUI SHALL broadcast it to the network within 5 seconds
4. THE Desktop GUI SHALL show confirmation when a grain is successfully stored locally and broadcasted
5. WHEN a grain is added, THE Desktop GUI SHALL immediately calculate and display the novelty-based NGT reward

### Requirement 5: Network Status Monitoring

**User Story:** As a node operator, I want to monitor my node's network health, so that I can ensure optimal participation.

#### Acceptance Criteria

1. THE Desktop GUI SHALL display the current number of connected peers in real-time
2. THE Desktop GUI SHALL show the total number of grains stored locally
3. THE Desktop GUI SHALL display node uptime since last start
4. THE Desktop GUI SHALL show network synchronization progress as a percentage
5. WHEN network connectivity is lost, THE Desktop GUI SHALL display a warning indicator

### Requirement 6: Cross-Platform Desktop Support

**User Story:** As a user on any major operating system, I want to run the SynapseNet node, so that I can participate regardless of my platform.

#### Acceptance Criteria

1. THE Desktop GUI SHALL run on Windows 10 and later versions
2. THE Desktop GUI SHALL run on macOS 10.15 (Catalina) and later versions
3. THE Desktop GUI SHALL run on Linux distributions with GTK 3.0 or later
4. THE Desktop GUI SHALL have a bundle size under 100MB for all platforms
5. THE Desktop GUI SHALL use less than 200MB of RAM during normal operation

### Requirement 7: Genesis Manifest Integration

**User Story:** As a new user, I want to understand SynapseNet's philosophy, so that I can decide if I want to participate.

#### Acceptance Criteria

1. THE Desktop GUI SHALL include a "Philosophy" section that displays the Genesis Manifest
2. THE Desktop GUI SHALL make the Genesis Manifest accessible before requiring node startup
3. THE Genesis Manifest SHALL explain the core principles of decentralized intelligence
4. THE Genesis Manifest SHALL describe the Homo Conexus vision
5. THE Genesis Manifest SHALL be available in plain text format for sharing

### Requirement 8: Automatic PoE Reward Distribution

**User Story:** As a contributor, I want to receive NGT automatically for my actions, so that I don't need to manually claim rewards.

#### Acceptance Criteria

1. WHEN a user adds a novel grain, THE PoE System SHALL automatically calculate and award NGT based on novelty score
2. WHEN a user validates another peer's grain, THE PoE System SHALL award validation rewards within 10 seconds
3. WHEN a user's node answers a query, THE PoE System SHALL award query response rewards
4. WHEN a user participates in swarm consensus, THE PoE System SHALL award consensus participation rewards
5. THE PoE System SHALL maintain a local ledger of all rewards before blockchain submission

### Requirement 9: Offline-First Operation

**User Story:** As a user with intermittent connectivity, I want the node to work offline, so that I can continue using it without constant internet access.

#### Acceptance Criteria

1. THE Desktop GUI SHALL allow searching local grains when network connectivity is unavailable
2. THE Desktop GUI SHALL queue new grains for broadcast when offline
3. WHEN network connectivity is restored, THE Desktop GUI SHALL automatically sync queued operations
4. THE Desktop GUI SHALL clearly indicate offline mode status
5. THE Desktop GUI SHALL allow viewing reward history and balance while offline

### Requirement 10: Public Website

**User Story:** As someone who discovered SynapseNet, I want to learn about it and download the node, so that I can start participating.

#### Acceptance Criteria

1. THE Website SHALL be accessible at synapsenet.org domain
2. THE Website SHALL have a homepage explaining what SynapseNet is and why it exists
3. THE Website SHALL provide download links for Windows, macOS, and Linux versions
4. THE Website SHALL host the complete Genesis Manifest as a readable page
5. THE Website SHALL include documentation on how the network works
6. THE Website SHALL have a minimalist design with white background and black text
7. THE Website SHALL load in under 2 seconds on standard broadband connections

### Requirement 11: Installation Simplicity

**User Story:** As a new user, I want to install SynapseNet quickly, so that I can start using it without technical setup.

#### Acceptance Criteria

1. THE Installer SHALL complete installation in under 5 minutes on standard hardware
2. THE Installer SHALL not require administrator privileges for user-space installation
3. THE Installer SHALL create desktop shortcuts automatically
4. THE Installer SHALL be digitally signed for security verification
5. WHEN installation completes, THE Installer SHALL offer to launch the application immediately

### Requirement 12: Data Export and Portability

**User Story:** As a user concerned about data ownership, I want to export my data, so that I maintain control over my contributions.

#### Acceptance Criteria

1. THE Desktop GUI SHALL provide an export function for all local grains
2. THE Desktop GUI SHALL support exporting data in JSON format
3. THE Desktop GUI SHALL support exporting data in CSV format
4. WHEN exporting data, THE Desktop GUI SHALL include all grain metadata (timestamps, authors, confidence scores)
5. THE Desktop GUI SHALL allow exporting reward history separately

---

**Status:** Requirements Complete  
**Next Phase:** Design Document  
**Version:** 1.0.0  
**Date:** 2024-11-01
