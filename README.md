# spinning_donut

A 3D spinning donut terminal animation rendered in Rust.

## How to Run

To build and run the animation with full optimizations, use:

```bash
cargo run --release
```

## Customization

You can customize the rendering look and feel in `src/main.rs`:

### Shading Style
Modify the `brightness` vector (around line 11) to switch between different rendering styles:
```rust
// Character / block shading styles:
// let brightness: Vec<char> = "░▒▓█".chars().collect();
let brightness: Vec<char> = ".,-~:;=!*#$@@".chars().collect();
```

### Rendering Density
Adjust the rotation step values inside the nested loop (around line 73) to change drawing density:
```rust
phi += 0.01;      // Smaller values increase density
theta += 0.05;    // Smaller values increase density
```
## Reflection
This was a fun project over the course of 3 days. Learned to pickup rust syntax and delved deep into relevant mathematical concepts, both of which I enjoyed. It didn't click all at once but just kept going at it. Fried my brain at times, but so worth it in the end.
