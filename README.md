# nodecode-terminal-kit

Reusable terminal UI components for the Nodecode TUI.

The main Nodecode TUI product lives at:
- `https://github.com/nodecode-ai/nodecode`

Common local checkout path:
- `~/nodecode/nodecode`

This repository contains reusable, shared building blocks extracted for that TUI.

## Crates

- `nodecode-terminal-kit` (`nodecode_terminal_kit`): design-focused UI components, layout, theme, and primitives.
- `nodecode-terminal-kit-runtime` (`nodecode_terminal_kit_runtime`): lightweight runtime loop (`Model/Msg/Command/Program`) used by kit-based apps.

## Scope

- Reusable components and theming primitives for terminal UI.
- Building blocks intended for composition inside Nodecode TUI and related kit consumers.
- No network/server orchestration or product-specific app logic.

## License

MIT
