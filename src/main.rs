use ibex::prelude::*;
use ibex::{routes, ssg};

/// URL of website root, relative to dxrcy.dev or localhost:4000
const URL_ROOT: &str = "/";

/// URL of production website
const URL_DOMAIN: &str = "https://dxrcy.dev";
/// URL of my Github profile
const URL_GITHUB: &str = "https://github.com/dxrcy";

fn main() {
    let routes = routes! [
        (/)    => at_index(),
        (/404) => at_404(),
    ];

    ssg::quick_build(routes).expect("Failed to build");
    println!("\x1b[34;1mBuilt successfully!\x1b[0m");
}

fn at_index() -> Document {
    document! { [lang="en"]
        @use_base []
        div ."header" {
            h1 { "darcy's website" }
            p {
                span { &laquo }
                "Welcome to my website."
                span { &raquo }
            }
        }

        main ."highlight-links" {
            article { h2 #"about" { "About Me" }
                div ."center" {
                    p {
                        "I like programming and languages."
                        ~"I speak English, Esperanto, and Rust."
                    }
                    p {
                        "Interested in low-level and systems programming,"
                        ~"and web development."
                    }
                }
            }

            hr/
            article { h2 #"links" { "Links" }
                div ."center" {
                    ul ."small-list" {
                        li { a [href=URL_GITHUB] { "GitHub Profile" } }
                    }
                }
            }

            hr/
            article { h2 #"experience" { "Programming Experience" }
                h3 { "C / C++" }
                p { "C is a great language, and C++ provides a few nice improvements." }
                ul ."compact-list" {
                    li { "Compiler and interpreter programming"
                        @minor_link[&format!("{URL_GITHUB}/lasim")]
                    }
                    li { "Low-level programming in Unix environments"
                        @minor_link[&format!("{URL_GITHUB}/vimput")]
                    }
                    li { "Command-line tools"
                        @minor_link[&format!("{URL_GITHUB}/xml-highlight")]
                    }
                }
                h3 { "Rust" }
                p { "Rust is my favourite language, for its type system, macros, and efficiency." }
                ul ."compact-list" {
                    li { "Command-line tools"
                        @minor_link[&format!("{URL_GITHUB}/everygarf")]
                    }
                    li { "Static site generation frameworks"
                        @minor_link[&format!("{URL_GITHUB}/ibex")]
                    }
                    li { "Image processing"
                        @minor_link[&format!("{URL_GITHUB}/mcimg")]
                    }
                    li { "Graphics rendering (ggez)" }
                    li { "Procedural macros and metaprogramming" }
                }
                h3 { "JavaScript / TypeScript" }
                p { "A necessary skill for front-end web development." }
                ul ."compact-list" {
                    li { "Client-side vanilla javascript with HTML and CSS or SCSS"
                        @minor_link[&format!("{URL_GITHUB}/color")]
                    }
                    li { "React single page applications"
                        @minor_link[&format!("{URL_GITHUB}/trustworthytimes")]
                    }
                    li { "Server-side Node.JS, Express" }
                }
                h3 { "Other languages which I am familiar with" }
                ul ."small-list" {
                    li { "Posix-compliant shell scripting (Bash)" }
                    li { "Some Zig, Java, Lua, and Haskell" }
                    li { "Popular CLI tools, like coreutils, FFmpeg, etc." }
                    li { "A bit of experience in Python, Handlebars, and AutoHotkey." }
                }
            }

            hr/
            article { h2 #"projects" { "Projects" }
                ul ."big-list" {
                    li { h3 { em{"LASIM"} ~&ndash;~ "LC-3 Assembler & Simulator" }
                        h4 { "Language: C++" }
                        p {
                            "An implementation of both an assembler and simulator for the"
                            ~a [href="https://en.wikipedia.org/wiki/Little_Computer_3"] {
                                i{ "Little Computer 3" }
                            }
                            ~"(LC-3) assembly language."
                        }
                        blockquote {
                            "Check out"
                            ~ a [href=format!("{URL_GITHUB}/lasim")]
                                { i{"LASIM"} ~"on GitHub" }
                        }
                    }
                    li { h3 { em{"EveryGarf"} ~&ndash;~ "Download every"~i{"Garfield"}~"comic as an image" }
                        h4 { "Language: Rust" }
                        p {
                            "A command-line tool which scrapes"
                            ~ a [href="https://gocomics.com/garfield"] {i{"GoComics.com"}}
                            ~ "to download every"~i{"Garfield"}~"comic concurrently."
                        }
                        blockquote {
                            "Check out"
                            ~ a [href=format!("{URL_GITHUB}/everygarf")]
                                { i{"EveryGarf"} ~"on GitHub" }
                            ", or"
                            ~ a [href="https://crates.io/crates/everygarf"]
                                { "on Crates.io" }
                        }
                    }
                    li { h3 { em{"Ibex"} ~&ndash;~ "Static site generation framework for Rust" }
                        h4 { "Language: Rust" }
                        p {
                            "Write HTML-style templates, which compile to static HTML files."
                            ~ "Perfect for sites without dynamic content, that are only changed occasionally."
                            ~ "Similar to" ~ a [href="https://handlebarsjs.com"] {"Handlebars.js"}
                            ", but memory-safe and strongly-typed!"
                        }
                        blockquote {
                            "Check out"
                            ~ a [href=format!("{URL_GITHUB}/ibex")]
                                { i{"Ibex"} ~"on GitHub" }
                            ","
                            ~ a [href=format!("{URL_GITHUB}/ibex-template")]
                                { "a basic SSG template" }
                        }
                    }
                    li { h3 { em{"Phonet"} ~&ndash;~ "Declarative"
                            ~ a [href="https://en.wikipedia.org/wiki/Phonotactics"] {"phonotactics"}
                            ~ "validation, using RegEx"
                        }
                        h4 { "Language: Rust" }
                        p {
                            "Used to create"
                            ~a [href="https://en.wikipedia.org/wiki/Constructed_language"]
                                { "constructed lanugages" }
                            ~"(conlangs)."
                        }
                        blockquote {
                            "Check out"
                            ~ a [href=format!("{URL_GITHUB}/phonet")]
                                { i{"Phonet"} ~"on GitHub" }
                            ", or on"
                            ~ a [href="https://crates.io/crates/phonet"]
                                { "Crates.io" }
                        }
                    }
                    li { h3 { em{"Garf-EO"} ~&ndash;~ "Garfield comics in Esperanto" }
                        h4 { "Language: Rust" }
                        p {
                            "800+ comics translated to"
                            ~a [href="https://en.wikipedia.org/wiki/Esperanto"]
                                { "Esperanto" }
                            ~"by your's truly."
                        }
                        blockquote {
                            "Check out"
                            ~ a [href=format!("{URL_DOMAIN}/garfeo")]
                                { i{"Garfield Esperanto"} ~"(website)" }
                            ", or"
                            ~ a [href=format!("{URL_GITHUB}/garfeo")]
                                { "the source code" }
                        }
                    }
                    li { h3 { em{"'Apple'"} ~&ndash;~ "Breakthrough innovation in the fields of web design and the contemporary arts" }
                        h4 { "Language: JavaScript / HTML" }
                        p {
                            "As my most ambitious project yet, this website combines both cutting-edge"
                            ~ "technology and artistic expression."
                        }
                        blockquote {
                            "Check out"
                            ~ a [href=format!("{URL_DOMAIN}/apple")]
                                { "'Apple'" }
                            ", or"
                            ~ a [href=format!("{URL_GITHUB}/apple")]
                                { "the source code" }
                        }
                    }
                }

                h3 { "This webpage" }
                p {
                    "This portfolio webpage was written in Rust, using a custom web framework '"
                    a [href=format!("{URL_GITHUB}/ibex")]
                        { "Ibex" }
                    "'."
                    blockquote {
                        "Check out"
                        ~ a [href=format!("{URL_GITHUB}/dxrcy.github.io")]
                            { "the source code for"~b{"this"}~"website" }
                    }
                }

                h3 #"other-projects" { "Other projects" }
                ul ."small-list" {
                    li { a [href=format!("{URL_GITHUB}/color")]
                        { "Color generation website" }
                    }
                    li { a [href=format!("{URL_GITHUB}/xml-highlight")]
                        { "Command-line XML syntax highligher" }
                    }
                    li { a [href=format!("{URL_GITHUB}/golad")]
                        { i{"Conway's Game of Life"}~"simulator written in C" }
                    }
                    li { a [href=format!("{URL_GITHUB}/sorting")]
                        { "Sorting algorithm visualizer" }
                    }
                    li { a [href=format!("{URL_GITHUB}/vimput")]
                        { "Simple single-line text input with"~i{"vim"}~"keybinds in"~i{"ncurses"} }
                    }
                    li { a [href=format!("{URL_GITHUB}/cttab")]
                        { "Customizable 'new tab' page for the browser" }
                    }
                    li { a [href=format!("{URL_GITHUB}/mcimg")]
                        { "Convert pixels of an image into"~i{"MineCraft"}~"blocks" }
                    }
                    li { a [href=format!("{URL_GITHUB}/markup-example")]
                        { "Markup-to-html compiler example" }
                    }
                    li { a [href=format!("{URL_GITHUB}/hangman")]
                        { "Collection of command-line"~i{"hangman"}~"examples in a few programming languages." }
                    }
                    // li { a [href=format!("{URL_GITHUB}/lisp")]
                    //     { "Simple"~i{"Lisp"}"-like programming language" }
                    // }
                    // li { a [href=format!("{URL_GITHUB}/recipe-lang")]
                    //     { "Programming language written like a cooking recipe" }
                    // }
                    // li { a [href=format!("{URL_GITHUB}/scripts")]
                    //     { "Some POSIX-compliant shell scripts" }
                    // }
                }
            }

            hr/
            article { h2 #"contributions" { "Projects I Have Contributed To" }
                ul ."big-list" {
                    li { h3 { em{"Git"} ~"✨" }
                        h4 { "Language: C" }
                        p {
                            "Fixed a bug relating to integer underflow when"
                            ~"using a commit timestamp close to Unix Epoch with"
                            ~"a positive timezone offset."
                            ~"GitHub shows the PR as closed/unmerged, because "
                            ~"the Git repo on GitHub is just a mirror."
                        }
                        blockquote {
                            a [href="https://github.com/git/git/pull/1726"] {
                                "View PR:"
                                ~code{ "fix: prevent date underflow when using positive timezone offset #1726" }
                            }
                        }
                    }
                    li { h3 { em{"Lace"} ~&ndash;~ "LC-3 assembly toolchain" }
                        h4 { "Language: Rust" }
                        p {
                            "Implemented an interactive" ~i{"debugger"}~ "interface,"
                            ~"along with many other minor features and fixes."
                        }
                        blockquote {
                            a [href="https://github.com/rozukke/lace/pull/46"] {
                                "View PR:"
                                ~code{ "Debugger #46" }
                            }
                        }
                        blockquote {
                            a [href="https://github.com/rozukke/lace/pulls?q=is%3Apr+is%3Aclosed+author%3Adxrcy"] {
                                "View other pull requests"
                            }
                        }
                    }
                    li { h3 { em{"MCPP"} ~&ndash;~ "MineCraft Interface Library" }
                        h4 { "Language: C++" }
                        p {
                            "Implemented multiple features and code quality improvements."
                        }
                        blockquote {
                            a [href="https://github.com/rozukke/mcpp/pulls?q=is%3Apr+author%3Adxrcy"] {
                                "View pull requests"
                            }
                        }
                    }
                    li { h3 { em{"Nvim-Tree"} ~&ndash;~ "File Explorer for NeoVim" }
                        h4 { "Language: Lua" }
                        p {
                            "Added support for file filters defined by"
                            ~"arbitrary Lua functions."
                        }
                        blockquote {
                            a [href="https://github.com/nvim-tree/nvim-tree.lua/pull/2655"] {
                                "View PR:"
                                ~code{ "filters.custom may be a function #2655" }
                            }
                        }
                    }
                }

                h3 #"other-contributions" { "Other contributions" }
                // p { "Which are either unmaintained or less interesting." }
                ul ."small-list" {
                    li { a [href=format!("https://github.com/sxyazi/yazi/pull/2143")]
                        { "Yazi" ~&ndash;~ "File Explorer (Rust)" }
                    }
                    li { a [href=format!("https://github.com/ferdium/ferdium-app/pull/1972")]
                        { "Ferdium" ~&ndash;~ "App Manager (TypeScript / React / Electron)" }
                    }
                    li { a [href=format!("https://github.com/spotDL/spotify-downloader/pull/2105")]
                        { "SpotDL" ~&ndash;~ "Spotify Downloader (Python)" }
                    }
                    li { a [href=format!("https://github.com/PolyMeilex/Neothesia/pull/74")]
                        { "Neothesia" ~&ndash;~ "MIDI Visualizer (Rust)" }
                    }
                    li { a [href=format!("https://github.com/zsh-users/zsh-history-substring-search/pull/159")]
                        { "Zsh History Substring Search" ~&ndash;~ "Shell Plugin (zsh)" }
                    }
                }
            }

            hr/
            article { h2 #"misc" { "Miscellaneous" }
                div { h3 #"workflow" { "Workflow" }
                    p {
                        "All my"
                        ~a [href="https://wiki.archlinux.org/title/Dotfiles"] { "dotfiles" }
                        ~"are available"
                        ~a[href=format!("{URL_GITHUB}/dotfiles")]{ strong{"here"} }
                        ", if you are interested."
                    }
                    ul ."small-list" {
                        li { em{"Operating System:"}
                            ~ a [href="https://endeavouros.com/"] { "EndevourOS" }
                            ~ "(Arch-based Linux), with"
                            ~ a [href="https://hyprland.org/"] { "Hyprland Compositor" }
                        }
                        li { em{"Programming:"}
                            ~ a [href="https://github.com/neovim/neovim"] { "NeoVim" }
                            ~ "+"
                            ~ a [href="https://github.com/tmux/tmux"] { "Tmux" }
                            ~ "+"
                            ~ a [href="https://github.com/git/git"] { "Git" }
                        }
                    }
                }

                div { h3 #"standards" { "Some of my favourite standards and protocols" }
                    ul ."small-list" {
                        li {
                            a [href="https://en.wikipedia.org/wiki/ISO_8601"] { "ISO-8601" }
                        }
                        li {
                            a [href="https://en.wikipedia.org/wiki/Unicode"] { "Unicode" }
                            ~"and" ~a [href="https://en.wikipedia.org/wiki/UTF-8"] { "UTF-8" }
                        }
                        li {
                            a [href="https://en.wikipedia.org/wiki/POSIX"] { "POSIX" }
                        }
                    }
                }
            }
        }

        footer {
            "Thanks for checking out my website!"
        }
    }
    .into()
}

fn minor_link(url: &str) -> View {
    view! {
        ~ a ."minor-link" [href=url] {
            i { "example➚" }
        }
    }
}

fn at_404() -> Document {
    document! { [lang="en"]
        @use_base []
        div ."center header" {
            h1 { "darcy's website" }
            h2 { "404 - Not found" }
            p {
                a [href=url!()] { "Did you mean to go the main page?" }
            }
        }
    }
    .into()
}

fn use_base() -> View {
    view! {
        HEAD {
            @use_meta [Meta::new()
                .url(URL_DOMAIN)
                .title("darcy's website")
                .desc("Welcome to my website.")
                .image(url!("static/icon.png"))
                .author("darcy")
                .color("#86b1b0")
            ]
            title { "darcy's website" }
            link [rel="shortcut icon", href=url!("static/icon.png")]/
            link [rel="stylesheet", href=url!("css/base.css")]/
            @ssg::use_autoreload []
        }
    }
}
