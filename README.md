# Cassette
Declarative web DSL language based on the Templar declarative UI language.

This project re-thinks how web development might work, disagreeing with some core assumptions about the design of the modern web, taking the Templar methodology and applying it to website generation.

It is compiled, implicitly typed, and makes use of macros, known as overlays. A very basic example of a templar project could look like this:

```haml
page "index.html" "Blah"
    h1 "hello"
        p "hi"
```

Templar makes heavy use of DSLs, and encourages developers to extend these. Each command above (page, h1, p) are actually overlays, which internally tell the compiler what to do with each node. For example in this case, we are generating a single HTML document with a few tags. A single templar document could produce an entire website of multiple pages, styles, and js, all with tight output control and a simple, readable format that allows you to create future abstractions.

For example, a more complicated document might look like this:

```haml
page "/"
    .title "page title"

    row
        h1 "hello!"
			.background red

    jumbotron
        .headline "an example header"
    
    main-body
        markdown(./main.md)
        cta "click here!"
```

No, those aren't fancy new html tag specifications. They are overlays, which you may have defined, or they may be available in the standard library. Everything is cleanly abstracted - only css styles that actually get USED are collected into the core stylesheet, and you don't have to think about it.

You can build up your own set of abstractions underneath, and overwrite the standard ones that come with Cassette too.

Speaking of which, if we were to expand the default overlays inside the std, they'd look similar to this:

```haml
page "index.html" "Blah"
    h1 "hello"
        p "hi"

:h1 tag
    .background red
    .type "h1"
    $1
    $0

:p tag
    .type "p"
    $1
```

## Style, Layout, and Behavior

Style sheets are painful. Ever have so many orphan styles in a project you have no idea what's being used? Javascript that doesn't play nice with other javascript, and includes way too much? Constantly mess with web font files and syntax? Realise all too late that a CSS rule was mistyped?

None of these are problems in Cassette.

Some inspiration was taken from projects like elm, but it is not designed to be as complex as elm. The ideal user would be someone who wants to rapidly create mostly static or semi-static websites (though support for more dynamic SPAs is coming).

The core of Cassette is a compiler and standard library built on top of the Templar language specifically designed for web work. (Or, in compiler-nerd-speak, if templar was a GCC interpreter, Cassette would be a GCC target).

## Templar/Cassette is NOT:

- a turing complete language
- production ready (yet)

## Future Work

The rest of this document specifies features that may not yet be in the product here on github, but will be very soon.



https://amethyst.rs/doc
