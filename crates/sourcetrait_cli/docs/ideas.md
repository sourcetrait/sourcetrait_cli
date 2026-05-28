ideas: srctrait
================================================================================

## sandbox

### features (image creation)

- `--basic` - "Basic (No Features)"
- `--features [snake,snake, snake,..]`
- `--preset <snake>` - Feature A (snake)

```rust
enum FeatureKind {
    Claude,
}

impl FeatureKind {
    pub const CLAUDE: &'static Feature = Feature { /* ... */ }
    pub const fn feature(&self) -> &'static Feature { /* ... */ }
}

struct Feature {
    snake: &'static str,
    name: &'static str,
    dependencies: Option<&'static [FeatureKind]>,
}

enum FeaturePresetKind {
    Claude,
}

impl FeaturePresetKind {
    pub const CLAUDE: &'static FeaturePreset = FeaturePreset { /* ... */ }
    pub const fn preset(&self) -> &'static FeaturePreset { /* ... */ }
}

struct FeaturePreset {
    snake: &'static str,
    name: &'static str,
    features: &'static [FeatureKind],
}
```

If `--basic` xor `--preset` xor `--features` is not specified, then:  
CLI pops up a TUI selection box for features:
- `[ ] Feature A (a)` - Not selected (action: select)
- `[x] Feature B (b)` - Selected (action: unselect)
- `[-] Feature C (c)` - Dependency (action: none (can't unselect)) 
- Buttons: `[cancel] [next]`
- Shortcuts:
  - Space - action
  - Esc - cancel
  - Enter - next

Review TUI:
- Features: comma separated list of feature names
- Buttons: `[back] [ok]`
- Shortcuts:
  - Esc - back
  - Enter - next

