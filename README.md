# Tui-Brûlée - a Taffy layout manager for Ratatui

Tui-Brûlée is a layout manager crate for [Ratatui] that allows you to use Taffy for laying out
flexbox and grid layouts. It's experimental right now and not fit for human (or rodent) consumption.

[Ratatui]: https://ratatui.rs

The purpose of this repo is to explore some ideas around layout and taffy. These might include but
are not limited to:

- Implementing a LayoutTree specific to the way widgets are layed out in Ratatui. (I have some
  unpushed work on this already to clean up and push).
- Implementing conversions between Ratatui / Taffy items (partial implementation of this is done)
- Implementing content aware measurements (how long is this text, and what dimensions does it need)
- Implementing additional layout methods / traits to help layout widgets (E.g. fn layout(), or trait
  SizeHint)
- Thinking about how borders, padding, margins, etc. can be brought into the layout rather than in
  the Rect / Block
- Specifically handling border areas in ways that can be used to collapse borders
- Scrollbar positioning / and content overflow ideas (feeding back into the scroll RFC issue in
  Ratatui)
- Handling composite widgets / containers that incorporate layout and display
- (Lowest priority) replacing cassowary with Taffy (i.e. rebuilding a like for like replacement of
  Ratatui's current layout implementation using this library)
- Backporting / including this into Ratatui as an alternative to the built-in layout manager.
  Perhaps as `ratatui-layout` and then imported under `ratatui::layout::taffy` or something similar.

There are some ideas in various Ratatui issues that are relevant to this:

- <https://github.com/ratatui-org/ratatui/issues/125>
- <https://github.com/ratatui-org/ratatui/issues/899>
- <https://github.com/ratatui-org/ratatui/issues/374>
- <https://github.com/ratatui-org/ratatui/issues/780>
- <https://github.com/ratatui-org/ratatui/issues/135>
- <https://github.com/ratatui-org/ratatui/issues/605>
- <https://github.com/ratatui-org/ratatui/issues/132>

## Contributing

As this is mostly experimental right now, contributions are likely difficult to do, but I'd accept
ideas in the form of self contained example binaries that explore an idea using taffy in ways that
can be used to pull out pieces that make sense (e.g. to grow a library of necessary parts).
Alternatively work on specifying use cases in issues would be helpful in guiding towards the right
place.
