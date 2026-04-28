# Error Handling & Logging

Software fails — networks drop, inputs surprise us, services return errors,
and disks fill up. The difference between a fragile program and a reliable
one is often not whether errors occur, but how the program responds when
they do. Thoughtful error handling turns silent failures into actionable
signals: it lets a process recover when recovery is possible, fail loudly
and clearly when it is not, and leave behind enough context that the next
person (or the next on-call shift) can understand what happened. Logging
is the other half of that story — it is the durable record of what your
code did, what it tried, and where it went wrong, and it is what makes
debugging in production possible at all.

This repo collects templates and snippets for writing informative error
handling and logging in a few common languages.

- [Python](/python/README.md)
- [`bash`](/bash/README.md)
- [Rust](/rust/README.md)