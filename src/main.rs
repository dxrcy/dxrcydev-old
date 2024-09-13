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
                    ul {
                        li { a [href=URL_GITHUB] { "My GitHub Profile" } }
                    }
                }
            }

            hr/
            article { h2 #"experience" { "Programming Experience" }
                h3 { "C / C++" }
                p { "C is a great language, and C++ provides a few nice improvements." }
                ul {
                    li { "Compiler and interpreter programming"
                        @minor_link["https://github.com/dxrcy/lasim"]
                    }
                    li { "Low-level programming in Unix environments"
                        @minor_link["https://github.com/dxrcy/vimcurses"]
                    }
                    li { "Command-line tools"
                        @minor_link["https://github.com/dxrcy/xml-parse"]
                    }
                }
                h3 { "Rust" }
                p { "Rust is my favourite language, due to its type system, macros, and speed." }
                ul {
                    li { "Command-line tools"
                        @minor_link["https://github.com/dxrcy/everygarf"]
                    }
                    li { "Static site generation frameworks"
                        @minor_link["https://github.com/dxrcy/ibex"]
                    }
                    li { "Image processing"
                        @minor_link["https://github.com/dxrcy/mcimg"]
                    }
                    li { "GGez (Graphics framework)" }
                    li { "Procedural macros and metaprogramming" }
                }
                h3 { "Javascript / Typescript" }
                p { "A necessary skill for front-end web development." }
                ul {
                    li { "Client-side vanilla javascript with HTML and CSS or SCSS"
                        @minor_link["https://github.com/dxrcy/color"]
                    }
                    li { "React single page applications"
                        @minor_link["https://github.com/dxrcy/trustworthytimes"]
                    }
                    li { "Server-side Node.JS, Express" }
                }
                h3 { "Other languages which I am familiar with" }
                ul {
                    li { "Posix-compliant shell scripting (Bash)" }
                    li { "Some Zig, Java, Lua, and Haskell" }
                    li { "Popular CLI tools, like coreutils, FFmpeg, etc." }
                    li { "A bit of experience in Python, Handlebars, and AutoHotkey." }
                }
            }

            hr/
            article { h2 #"projects" { "Projects" }
                ul ."big-list" {
                    li { h3 { em{"LASIM"}
                            ~ "- "
                            a [href="https://en.wikipedia.org/wiki/Little_Computer_3"] { "LC-3" }
                            " Assembler & Simulator" }
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
                                { "LASIM on GitHub" }
                        }
                    }
                    li { h3 { em{"EveryGarf"} ~ "- Download every"~i{"Garfield"}~"comic as an image" }
                        p {
                            "A CLI tool which scrapes" ~ a [href="https://gocomics.com"] {i{"GoComics.com"}}
                            ~ "to download every"~i{"Garfield"}~"comic concurrently."
                        }
                        blockquote {
                            "Check out"
                            ~ a [href=format!("{URL_GITHUB}/everygarf")]
                                { "EveryGarf on GitHub" }
                            // ", or"
                            // ~ a [href="https://crates.io/crates/everygarf"]
                            //     { "on Crates.io" }
                        }
                    }
                    li { h3 { em{"Ibex"} ~ "- Static site generation framework for Rust" }
                        p {
                            "Write HTML-style templates, which compile to static HTML files."
                            ~ "Perfect for sites without dynamic content, that are only changed occasionally."
                            ~ "This website is actually written in using Ibex."
                            ~ "Similar to" ~ a [href="https://handlebarsjs.com"] {"Handlebars.js"}
                            ~ ", but type-safe!"
                        }
                        blockquote {
                            "Check out"
                            ~ a [href=format!("{URL_GITHUB}/ibex")]
                                { "Ibex on GitHub" }
                            ","
                            ~ a [href=format!("{URL_GITHUB}/ibex-template")]
                                { "a basic SSG template" }
                           ", or"
                            ~ a [href=format!("{URL_GITHUB}/dxrcy.github.io")]
                                { "the source code for"~b{"this"}~"website" }
                        }
                    }
                    li { h3 { em{"Phonet"} ~ "- Declarative"
                            ~ a [href="https://en.wikipedia.org/wiki/Phonotactics"] {"phonotactics"}
                            ~ "validation, using Regex"
                        }
                        p {
                            "Used to create constructed lanugages (conlangs)."
                        }
                        blockquote {
                            "Check out"
                            ~ a [href=format!("{URL_GITHUB}/phonet")]
                                { "Phonet on GitHub" }
                            ", or on"
                            ~ a [href="https://crates.io/crates/phonet"]
                                { "Crates.io" }
                        }
                    }
                    li { h3 { em{"Garf-EO"} ~ "- Garfield comics in Esperanto" }
                        p {
                            "500+ comics translated to Esperanto by your's truly."
                        }
                        blockquote {
                            "Check out"
                            ~ a [href=format!("{URL_DOMAIN}/garfeo")]
                                { "Garfield Esperanto (website)" }
                            ", or"
                            ~ a [href=format!("{URL_GITHUB}/garfeo")]
                                { "the source code" }
                        }
                    }
                    li { h3 { em{"'Apple'"} ~ "- Breakthrough innovation in the fields of web design and the contemporary arts" }
                        p {
                            "As my most ambitious project yet, this website combines both cutting-edge"
                            ~ "technology and artistic expression"
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

                h3 #"other-projects" { "Other projects" }
                ul ."small-list" {
                    li { a [href=format!("{URL_DOMAIN}/color")]
                        { "Colour Sliders" }
                    }
                    li { a [href=format!("{URL_GITHUB}/golad")]
                        { "Conway's Game of Life simulator written in C" }
                    }
                    li { a [href=format!("{URL_GITHUB}/recipe-lang")]
                        { "Programming language written like a cooking recipe" }
                    }
                    li { a [href=format!("{URL_GITHUB}/sorting")]
                        { "Sorting algorithm visualizer" }
                    }
                    li { a [href=format!("{URL_GITHUB}/lisp")]
                        { "Simple"~i{"Lisp"}"-like programming language" }
                    }
                    li { a [href=format!("{URL_GITHUB}/hangman")]
                        { "Collection of command-line hangman examples in a few programming languages." }
                    }
                    li { a [href=format!("{URL_GITHUB}/mcimg")]
                        { "Convert pixels of an image into"~i{"Minecraft"}~"blocks" }
                    }
                    li { a [href=format!("{URL_GITHUB}/vimcurses")]
                        { "Simple single-line text input with vim keybinds in ncurses" }
                    }
                    li { a [href=format!("{URL_GITHUB}/cttab")]
                        { "Customizable 'new tab' page for the browser" }
                    }
                    li { a [href=format!("{URL_GITHUB}/scripts")]
                        { "Some POSIX-compliant shell scripts" }
                    }
                    li { a [href=format!("{URL_GITHUB}/markup-example")]
                        { "Markup-to-html compiler example" }
                    }
                }
            }

            hr/
            article { h2 #"contributions" { "Projects I have contributed to" }
                ul ."big-list" {
                    li { h3 { em{"Git"} "✨" }
                        p {
                            "Fixed a bug relating to integer underflow when"
                            ~"using a commit timestamp close to Unix Epoch with"
                            ~"a positive timezone offset."
                        }
                        blockquote {
                            a [href="https://github.com/git/git/pull/1726"] {
                                "View pull request"
                            }
                        }
                    }
                    li { h3 { em{"Nvim Tree"} }
                        p {
                            "Added support for file filters defined by"
                            ~"arbitrary Lua functions."
                        }
                        blockquote {
                            a [href="https://github.com/nvim-tree/nvim-tree.lua/pull/2655"] {
                                "View pull request"
                            }
                        }
                    }
                    li { h3 { em{"Spotify Downloader"} }
                        p {
                            "Fixed a bug relating to incorrect track numbering"
                            ~"when downloading playlists containing invalid or"
                            ~" or local files"
                        }
                        blockquote {
                            a [href="https://github.com/spotDL/spotify-downloader/pull/2105"] {
                                "View pull request"
                            }
                        }
                    }
                }

                h3 #"other-contributions" { "Other contributions" }
                // p { "Which are either unmaintained or less interesting." }
                ul ."small-list" {
                    li { a [href=format!("https://github.com/zsh-users/zsh-history-substring-search/pull/159")]
                        { "Zsh History Substring Search" }
                    }
                    li { a [href=format!("https://github.com/PolyMeilex/Neothesia/pull/74")]
                        { "Neothesia" }
                    }
                }
            }

            hr/
            article { h2 #"workflow" { "Workflow" }
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
                        ~ a [href="https://github.com/baskerville/bspwm"] { "BSPWM Window Manager" }
                    }
                    li { em{"Programming:"}
                        ~ a [href="https://github.com/tmux/tmux"] { "Tmux" }
                        ~ "+"
                        ~ a [href="https://github.com/neovim/neovim"] { "NeoVim" }
                        ~ "+"
                        ~ a [href="https://github.com/git/git"] { "Git" }
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
        ~ a ."minor-link" [href=url, target="_blank"] {
            i { "example ➚" }
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

