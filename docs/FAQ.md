# FAQ

### Does Auto use one format for every image?
No. The decision engine evaluates image features and may select different candidates.

### Can I target a file size?
Yes. Use `target` with values such as `500KB` or `2MB`.

### Can I define operating hours?
Yes. Store weekday/time windows in JSON and use `schedule estimate` to calculate whether the queue is currently inside a window and how long until the next window.

### Does a broken image stop a batch?
No. Batch failures are isolated to individual files.

### Is the GUI dependent on the Rust engine?
Yes by design: both CLI automation and Qt GUI share the same Rust engine executable and JSON contract.
