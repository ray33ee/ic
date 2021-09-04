# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### To Do
- Rewrite the code as it is messy. But first, implement...
  - Figure out the command line options, then add them via Clap
  - Implement Stop system (via key event and sigterm) 

### Unfinished Ideas

## [0.1.2] - 2021-09-02
### Added
- We now Seek to the position in the destination file instead of appending
- `Copier` class combined with `Manager` and renamed to `IC`
- Overhaul of entire project
  - We now use channels instead of mutexes for communication
  - We have two main classes that communicate (instead of 3)

### Fixed
- Progress bars now start at the correct position when resuming

## [0.1.1] - 2021-08-31
### Added
- Added new .rs files for the new objects including
  - FolderSignature
  - GUI
  - Options
  - GUIManager
  - IC
- Basic interruptible copier implemented    
- Allows user to play and pause copying

## [0.1.0] - 2021-08-30
### Added
- Initial commit