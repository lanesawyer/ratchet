# Ratchet

Ratchet is a tool for fixing up legacy codebases by adding new rules without requiring current violations to be immediately fixed. It takes heavy inspiration from [Betterer](https://phenomnomnominal.github.io/betterer/), but aims to be compatible and/or extensible to any programming language.

## Installation

TODO: Figure out distribution method. Probably multiple, depending on the language.

## Usage

There are currently three commands available:
1. `ratchet init` - Initializes a new ratchet configuration file
2. `ratchet turn` - Goes through the codebase for any violations and updates the tracking document accordingly
3. `ratchet check` - Checks the current codebase against the previous tracking document to see if any new violations have been added (useful for CI jobs)

More will be added over time and existing ones enhanced as the project matures.

## Why Ratchet?
Most code is "legacy" code. At some point, you'll be tasked with maintaining a codebase that lacks linting rules, is missing new language features, has a million TODOs, etc.. Ratchet helps you to fix up your codebase over time by introducing new rules without requiring the current code to follow those rules. This allows you to slowly improve the codebase without needing to stop all development to fix everything at once.

Ratchet's ultimate goal is to be uninstalled from your codebase. As you've fixed each rule, you can remove it from Ratchet and add it to your tool's official configuration.

## Contributing

TODO: Add contributing guidelines and figure out the license
