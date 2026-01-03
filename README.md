# Open Ragnarok

This repository is intended to be a collection of tools and libraries for the Ragnarok Online game file formats.  The goal is to provide documentation for the assorted filetypes and formats used by the game, with the eventual goal of a complete implementation of parsers (and maybe writers/editors) to manipulate it.

The longer term goal is to eventually make some form of server and client clone of the game to help me with learning to develop games, learn some more Rust, and generally define my hobby.

NOTE: I provide absolutely zero warranty or support for any of the tools or libraries in this repository, nor do I promise that it will be completed.

## Milestones

Here's a list of milestone projects for this repository that I'd like to create.  I will expand the list with sub-items to try to break down some subprojects needed to hit the larger goal, as I get to them.

- [ ] File Parsers - Read Only
  - [ ] ACT Files
  - [ ] EBM Files
  - [ ] GAT Files
  - [ ] GND Files
  - [ ] GR2 Files
  - [ ] GRF Files
    - [x] Basic Header Parsing
    - [ ] 0x1XX Version handling
    - [ ] 0x200 Version handling
    - [ ] Finding if a file exists
    - [ ] Reading contents of a file
    - [ ] Lazy loading
  - [ ] IMF Files
  - [ ] PAL Files
  - [ ] RGZ Files
  - [ ] RSW Files
  - [ ] SPR Files
- [ ] File Writers - Allowing manipulation of some of the above formats
  

## Sources of Note

I will try to provide sources in assorted README files closer to the place they are used, but some sources have already stood out as being useful from the start:

- Ragnarok Research Lab - https://github.com/RagnarokResearchLab/ - Some examples of Arcturus+Ragnarok file formats and parsers, primarily in Lua, to use as examples-- as well as a RagLite server example.
