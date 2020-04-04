# Cassette
Rapid web frontend generator based on the Templar declarative UI language.

This project re-thinks how web development might work, disagreeing with some core assumptions about the design of the modern web, taking the Templar methodology and applying it to website generation.

Development has become cumbersome and slow due to feature bloat and technology isolation. Taking cue from languages such as elm and moving them even further into a declarative pipeline structure, throwing away flexibility for speed and ease of use, one might arrive at a rapid solution such as this.

The modern web separates development into:
- scripting (javascript)
- styling and layout (css)
- content, styling, layout, config (html)

This setup inevitably produces awful convoluted messes.

Templar declarative structures:
- eliminate a whole class of bugs and code bloat, preventing orphan code and incorrect project states.
- allow for huge module libraries, while resultant output only includes modules actually used.

## Templar

Cassette renders templar structures, but theoretically it could use a different declarative language that adheres to the same methodology.

## Project Structure

Projects are separated into three declarative structures.
- layout
- content
- style

### Alpha version goals
- interpret very simple layout structure (styling later?)

layout (no styling)
- each takes (json? indexmap?) inputs
- blocks/objects
	- classes (for styling), patterns/rules (dictate layout patterns)
	- anonymous

templar(Declarative Code -> AST / node graph -> Templar interpreted structure) -> cassette(renders as html/css/js)

Idea:
- model/state, layout/interaction, config/styling

Idea:
- Index module is the core, and internal links write pages. (what about unrelated landing pages?)
- Optional config file can generate other indexes / pages by linking a new output (which is a templar object).

/layout.templar
```
index
	rules expanded-x fixed-height(30)
	content "My Site"
```

/style.templar
```
style expanded-x
	width 100%
```

/state.templar


here, index is essentially a 'function name', with no arguments. inside are function calls, with arguments that are also functions. The goal is to build a large declarative structure with no repetition of things like style rules etc.

"My Site" is treated as an anonymous function that returns a static string.

shorthand:
```
index: expanded-x, fixed-height(30) ; "My Site"
```

https://amethyst.rs/doc
