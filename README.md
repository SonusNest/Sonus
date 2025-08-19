# Sonus
Sonus is an open-source, lightweight cross-platform desktop application developed with Tauri + Rust. As a private music library management player, it not only manages music stored locally on the running device but also enables localized management of music in home private clouds (NAS) or remote servers via WebDAV and SMB protocols.

The project is currently in the initial development phase, and the production environment is not yet ready.

## Project Development TODO
Currently, this TODO list primarily targets Windows platform development. macOS and Linux support will be added concurrently upon the release of the 1.0 stable version.
### Backend Development

#### Core Infrastructure
- [ ] `src/main.rs`
    - [x] Initialize Tauri application with proper configuration
    - [x] Set up application window dimensions and properties
    - [ ] Register all IPC commands from `ipc::commands`
    - [ ] Configure application lifecycle hooks
    - [ ] Set up error handling for application startup

#### IPC Communication Layer
- [ ] `src/ipc/mod.rs`
    - [ ] Export all IPC modules and public interfaces
    - [ ] Define unified error type for IPC operations

- [ ] `src/ipc/commands.rs`
    - [ ] Implement `play` command with track ID parameter
    - [ ] Implement `pause` command
    - [ ] Implement `stop` command
    - [ ] Implement `next_track` and `previous_track` commands
    - [ ] Implement `set_volume` command with percentage parameter
    - [ ] Implement `seek` command with position parameter
    - [ ] Implement `scan_music_library` command
    - [ ] Implement `create_playlist` command
    - [ ] Implement `add_to_playlist` command
    - [ ] Implement `search_library` command with query parameter

- [ ] `src/ipc/events.rs`
    - [ ] Define `playback_state_changed` event
    - [ ] Define `track_changed` event with new track metadata
    - [ ] Define `progress_updated` event with current position
    - [ ] Define `volume_changed` event
    - [ ] Define `library_scan_started` and `library_scan_completed` events
    - [ ] Implement event emission system

- [ ] `src/ipc/types.rs`
    - [x] Define `Track` struct with serde serialization
    - [ ] Define `PlaybackState` enum (Playing/Paused/Stopped)
    - [x] Define `Playlist` struct
    - [x] Define `LibraryScanProgress` struct
    - [ ] Define `SearchResults` struct
    - [ ] Ensure all types implement proper serialization/deserialization

#### Application Control Layer
- [ ] `src/app/mod.rs`
    - [ ] Export window and tray modules

- [ ] `src/app/window.rs`
    - [x] Implement window minimization functionality
    - [x] Implement window maximization/restoration
    - [ ] Implement window closing behavior (minimize to tray option)
    - [x] Implement window material settings
    - [x] Add borderless window mode support
    - [ ] Overwrite the original close window logic
    - [ ] Implement window transparency settings

- [ ] `src/app/tray.rs`
    - [ ] Create system tray icon
    - [ ] Implement tray context menu (Play/Pause/Exit)
    - [ ] Add tooltip showing current track
    - [ ] Implement "Show/Hide" application window from tray
    - [ ] Add tray icon animation for playback state

#### Core Business Logic
- [ ] `src/core/mod.rs`
    - [ ] Export all core modules
    - [ ] Set up core initialization sequence

- [ ] `src/core/player/mod.rs`
    - [ ] Export player controller, backend and state modules

- [ ] `src/core/player/controller.rs`
    - [ ] Implement play functionality with track loading
    - [ ] Implement pause/resume functionality
    - [ ] Implement stop functionality
    - [ ] Add track seeking capability
    - [ ] Implement volume control
    - [ ] Add track position query method
    - [ ] Implement track duration calculation

- [ ] `src/core/player/audio_backend.rs`
    - [ ] Integrate rodio audio library
    - [ ] Add support for common audio formats (MP3, FLAC, WAV, etc.)
    - [ ] Implement audio stream management
    - [ ] Add error handling for audio playback issues
    - [ ] Optimize audio buffer management

- [ ] `src/core/player/state.rs`
    - [ ] Create playback state struct
    - [ ] Implement current track tracking
    - [ ] Add progress tracking with timestamp updates
    - [ ] Implement volume level storage
    - [ ] Add playback status flags (buffering, error, etc.)

- [ ] `src/core/library/mod.rs`
    - [ ] Export library modules

- [ ] `src/core/library/scanner.rs`
    - [x] Implement directory traversal for music files
    - [ ] WebDAV supported 
    - [ ] SMB supported
    - [ ] Implement scan progress tracking
    - [ ] Implement incremental scanning (only new files)

- [ ] `src/core/library/metadata.rs`
    - [x] Implement audio metadata extraction
    - [x] Parse mostly tags for most common files
    - [x] Extract album art
    - [x] Handle missing or corrupted metadata gracefully
    - [x] Normalize metadata fields across formats
    - [ ] Optimize code to improve robustness

- [ ] `src/core/library/index.rs`
    - [x] Implement index persistence to sqlite
    - [x] Add indexing by artist, album, genre
    - [ ] Add index validation and repair

- [ ] `src/core/library/search.rs`
    - [ ] Implement fuzzy search for tracks
    - [ ] Add search by title, artist, album
    - [ ] Implement advanced search with filters
    - [ ] Add search result ranking
    - [ ] Implement search query parsing
    - [ ] Add search by lyrics

- [ ] `src/core/playlist/mod.rs`
    - [ ] Export playlist modules

- [ ] `src/core/playlist/manager.rs`
    - [x] Implement core playlist logic
    - [ ] Implement playlist creation
    - [ ] Add playlist deletion
    - [x] Implement track addition to playlists
    - [x] Add track removal from playlists
    - [ ] Implement playlist renaming
    - [ ] Add playlist reordering

- [ ] `src/core/playlist/play_mode.rs`
    - [x] Add repeat single track mode
    - [x] Implement repeat all mode
    - [x] Add shuffle mode
    - [ ] Implement smart shuffle (avoid recent repeats)

- [ ] `src/core/playlist/persistence.rs`
    - [ ] Implement playlist saving to sqlite
    - [ ] Add playlist loading from sqlite

- [ ] `src/core/state/mod.rs`
    - [ ] Export global state and observer modules

- [ ] `src/core/state/global.rs`
    - [ ] Create global application state struct
    - [ ] Implement thread-safe access patterns
    - [ ] Add state initialization and reset
    - [ ] Implement state validation
    - [ ] Add state snapshot capability

- [ ] `src/core/state/observer.rs`
    - [ ] Implement observer pattern for state changes
    - [ ] Add subscription system for state events
    - [ ] Implement efficient event propagation
    - [ ] Add unsubscribe functionality
    - [ ] Implement event batching for performance

#### Utilities
- [ ] `src/utils/mod.rs`
    - [ ] Export all utility modules

- [ ] `src/utils/error.rs`
    - [ ] Define custom error enum for application
    - [ ] Implement error conversion from dependencies
    - [ ] Add error messages localization support
    - [ ] Implement error logging functionality
    - [ ] Add user-friendly error descriptions

- [ ] `src/utils/logger.rs`
    - [ ] Set up logging framework
    - [ ] Implement log levels (debug, info, warn, error)
    - [ ] Add file logging capability
    - [x] Implement console logging
    - [ ] Add log rotation for large log files
    - [x] Implement log filtering by module

- [ ] `src/utils/fs.rs`
    - [ ] Implement cross-platform path handling
    - [ ] Add file existence checks
    - [ ] Implement directory creation
    - [ ] Add file deletion functionality
    - [ ] Implement safe file writing (atomic writes)
    - [ ] Add file type detection

- [ ] `src/utils/lifecycle.rs`
    - [ ] Implement application startup sequence
    - [ ] Add graceful shutdown handling
    - [ ] Implement application suspension/resumption
    - [ ] Add cleanup routines
    - [ ] Implement crash recovery

#### Assets
- [ ] `src/assets/icons/`
    - [ ] Create application icons in various sizes
    - [x] Add playback control icons
    - [ ] Create tray icons for different states
    - [ ] Add theme-specific icons (light/dark)
    - [ ] Implement high-DPI icon support

### Frontend Development (Web)

#### Foundation
- [ ] Project setup
    - [x] Configure Vue framework
    - [x] Set up TypeScript integration
    - [ ] Configure build tools for Tauri
    - [x] Set up CSS framework (Tailwind)
    - [x] Configure ESLint and code formatting

- [ ] Routing
    - [x] Implement main application routes
    - [x] Add navigation between views
    - [ ] Implement route guards for protected views
    - [ ] Add history management
    - [ ] Implement deep linking support

- [ ] State management
    - [x] Set up frontend state management
    - [ ] Create store for player state
    - [ ] Implement library state management
    - [ ] Add playlist state management
    - [x] Implement UI state persistence

#### Core Components
- [x] Player controls
    - [x] Create play/pause button component
    - [x] Implement previous/next track buttons
    - [x] Create progress bar with seek functionality
    - [x] Implement volume control slider
    - [x] Add repeat mode toggle
    - [x] Implement shuffle mode toggle

- [x] Now playing display
    - [x] Create current track information panel
    - [x] Implement album art display
    - [x] Add artist/album links
    - [x] Create lyric display component
    - [x] Implement track duration display

- [ ] Music library views
    - [ ] Create album grid view
    - [ ] Implement artist list view
    - [x] Add track list view
    - [ ] Create genre filtering view
    - [x] Implement sortable columns
    - [ ] Add pagination for large libraries

- [ ] Playlist management
    - [ ] Create playlist list component
    - [ ] Implement playlist creation form
    - [ ] Add track addition interface
    - [ ] Create playlist editing interface
    - [ ] Implement drag-and-drop reordering
    - [ ] Add playlist deletion confirmation

- [ ] Search functionality
    - [ ] Create search input component
    - [ ] Implement real-time search results
    - [ ] Add advanced search filters
    - [ ] Create search result categorization
    - [ ] Implement search history

#### Pages
- [x] Main application layout
    - [x] Create sidebar navigation
    - [x] Implement main content area
    - [x] Add footer with player controls
    - [x] Create responsive layout for different window sizes

- [ ] Library page
    - [x] Implement tabbed interface (Songs/Albums/Artists)
    - [x] Add filtering controls
    - [ ] Implement batch operations
    - [ ] Create context menus for tracks

- [ ] Playlist page
    - [ ] Implement playlist track listing
    - [ ] Add track removal controls
    - [ ] Create playlist information panel
    - [ ] Implement track count and total duration display

- [ ] Now playing page
    - [ ] Create expanded player view
    - [ ] Implement large album art display
    - [ ] Add detailed track information
    - [ ] Create enhanced progress visualization

- [ ] Settings page
    - [ ] Implement library location configuration
    - [x] Add appearance settings
    - [ ] Create audio quality settings
    - [ ] Implement shortcut key configuration
    - [x] Add about/version information

#### IPC Integration
- [ ] Backend communication service
    - [ ] Create wrapper for Tauri IPC calls
    - [ ] Implement type-safe IPC interfaces
    - [ ] Add error handling for IPC operations
    - [ ] Create request batching for performance

- [ ] Event handling
    - [ ] Set up listeners for playback events
    - [ ] Implement library update event handlers
    - [ ] Add UI updates for state changes
    - [ ] Create debounced handlers for frequent events

#### UI/UX Enhancements
- [ ] Theming
    - [x] Implement light/dark modes
    - [ ] Add accent color customization
    - [ ] Create high-contrast mode
    - [ ] Implement custom theme support

- [ ] Animations
    - [ ] Add transition between views
    - [ ] Implement playback state animations
    - [ ] Create hover effects for interactive elements
    - [ ] Add loading indicators

- [ ] Responsive design
    - [ ] Optimize for different window sizes
    - [ ] Add touch-friendly controls
    - [ ] Create collapsible UI elements

- [ ] Accessibility
    - [ ] Implement keyboard navigation
    - [ ] Ensure color contrast compliance
    - [ ] Implement focus indicators

#### Testing & Polish
- [ ] Unit tests for components
- [ ] Integration tests for user flows
- [ ] Performance optimization
- [ ] Cross-platform testing
- [ ] UI refinement and bug fixes
- [ ] Localization support for multiple languages

# The journey is long and arduous, and we look forward to your contributions.