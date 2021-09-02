# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### To Do
- Figure out the command line options, then add them via Clap
- Create folder structure before the copy comenses
- Implement Stop system (via key event and sigterm) 
- Don't append when writing to the destination file, `Seek` instead

### Unfinished Ideas

## [0.1.1] - 2021-08-31
### Added
- Added new .rs files for the new objects includin
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