use ibex::prelude::*;
use ibex::{routes, ssg};

const URL_ROOT: &str = "/";

fn main() {
    let routes = routes! [
        (/)    => at_index(),
        (/404) => at_404(),
    ];

    ssg::quick_build(routes).unwrap();
    println!("All done!");
}

fn at_index() -> Document {
    view! {
        @use_base[]
        h1 { "darcy's website" }
        p {
            "Welcome to my website."
        }

        hr/
        div { h2 { "About Me" }
            p {
                "I like programming and languages."
                ~"I speak English, Esperanto, and Rust."
            }
        }
        
        hr/
        div { h2 { "Links" }
            ul {
                li { a [href="https://github.com/darccyy"] { "My GitHub Profile" } }
            }
        }

        hr/
        div { h2 { "Programming Experience" }
            h3 { "Rust" }
            p { "Rust is my favourite language, due to its type system, macros, and speed." }
            ul {
                li { "CLI apps and simple tools" }
                li { "Static site generation frameworks" }
                li { "Procedural macros" }
                li { i{"Egui"}~"(UI framework)" }
                li { i{"GGez"}~" (Graphics framework)" }
            }
            h3 { "Javascript / Typescript" }
            ul {
                li { "Client-side vanilla javascript with HTML and CSS or SCSS" }
                li { "Server-side"~i{"Node.JS"}","~i{"Express"} }
                li { i{"React"}~"single page applications" }
            }
            h3 { "Other" }
            ul {
                li { "Posix-compliant shell scripting (like"~i{"Bash"}")" }
                li { "Popular CLI tools, like"~i{"coreutils"}","~i{"FFmpeg"}", etc." }
                li { "Some "~i{"Lua"}","~i{"Haskell"}", and"~i{"Handlebars"} }
                li { "Previously,"~i{"Python"}~"and"~i{"AutoHotkey"}", but I have not used them in a while." }
            }
            p {
                "I also plan to learn"~i{"Go"}~"and"~i{"C"}"."
            }
        }

        hr/
        div { h2 { "Projects" }
            ul {
                li { h3 { i{"Ibex"} ~ "- Static site generation framework for Rust" }
                    p {
                        "Write HTML-style templates, which compile to static HTML files."
                        ~ "Perfect for sites without dynamic content, that are only changed occasionally."
                        ~ "This website is actually written in using Ibex."
                        ~ "Similar to" ~ a [href="https://handlebarsjs.com"] {"Handlebars.js"}
                        ~ ", but type-safe!"
                    }
                    blockquote {
                        "Check out"
                        ~ a [href="https://github.com/darccyy/ibex"]
                            { "Ibex on GitHub" }
                        ","
                        ~ a [href="https://github.com/darccyy/ibex-template"]
                            { "a basic SSG template" }
                       ", or"
                        ~ a [href="https://github.com/darccyy/darccyy.github.io"]
                            { "the source code for"~b{"this"}~"website" }
                    }
                }
                li { h3 { i{"CTTab"} ~ "- A customizable 'new tab' page for the browser" }
                    p {
                        "Add a background, shortcuts, and notes to your browser's homepage."
                        ~ "Supports Chromium and Firefox based browsers."
                    }
                    blockquote {
                        "Check out"
                        ~ a [href="https://github.com/darccyy/cttab"]
                            { "CTTab on GitHub" }
                       ", or"
                        ~ a [href="https://darccyy.github.io/cttab"]
                            { "a live example" }
                    }
                }
                li { h3 { i{"EveryGarf"} ~ "- Download every"~i{"Garfield"}~"comic as an image" }
                    p {
                        "A CLI tool which scrapes" ~ a [href="https://gocomics.com"] {i{"GoComics.com"}}
                        ~ "to download every"~i{"Garfield"}~"comic."
                    }
                    blockquote {
                        "Check out"
                        ~ a [href="https://github.com/darccyy/everygarf"]
                            { "EveryGarf on GitHub" }
                        // ", or"
                        // ~ a [href="https://crates.io/crates/everygarf"]
                        //     { "on Crates.io" }
                    }
                }
                li { h3 { i{"Phonet"} ~ "- Declarative"
                        ~ a [href="https://en.wikipedia.org/wiki/Phonotactics"] {"phonotactics"}
                        ~ "validation, using Regex"
                    }
                    p {
                        "Used to create constructed lanugages (conlangs)."
                    }
                    blockquote {
                        "Check out"
                        ~ a [href="https://github.com/darccyy/phonet"]
                            { "Phonet on GitHub" }
                        ", or on"
                        ~ a [href="https://crates.io/crates/phonet"]
                            { "Crates.io" }
                    }
                }
                li { h3 { i{"Garf-EO"} ~ "- Garfield comics in Esperanto" }
                    p {
                        "500+ comics translated to Esperanto by your's truly."
                    }
                    blockquote {
                        "Check out"
                        ~ a [href="https://darccyy.github.io/garfeo"]
                            { "Garfield Esperanto (website)" }
                        ", or"
                        ~ a [href="https://github.com/darccyy/garfeo"]
                            { "the source code" }
                    }
                }
                li { h3 { i{"The Trustworthy Times"} ~ "- A satirical news website (funny)" }
                    p {
                        "A silly little static website with funny fake news articles."
                        ~ "I might one day port it to"~i{"Ibex"}"."
                    }
                    blockquote {
                        "Check out"
                        ~ a [href="https://trustworthytimes.com"]
                            { i{"The Trustworthy Times"} }
                        ", or"
                        ~ a [href="https://github.com/trustworthytimes/trustworthytimes.github.io"]
                            { "the source code" }
                    }
                }
                li { h3 { i{"'Apple'"} ~ "- Breakthrough innovation in the fields of web design and the contemporary arts" }
                    p {
                        "As my most ambitious project yet, this website combines both cutting-edge"
                        ~ "technology and artistic expression"
                    }
                    blockquote {
                        "Check out"
                        ~ a [href="https://darccyy.github.io/apple"]
                            { "'Apple'" }
                        ", or"
                        ~ a [href="https://github.com/darccyy/apple"]
                            { "the source code" }
                    }
                }
            }

            h3 { "Other projects" }
            p { "Which are either unmaintained or less interesting." }
            ul {
                li { a [href="https://github.com/darccyy/recipe-lang"]
                    { "Programming language written like a cooking recipe" }
                }
                li { a [href="https://github.com/darccyy/mcimg"]
                    { "Convert pixels of an image into"~i{"Minecraft"}~"blocks" }
                }
                li { a [href="https://github.com/darccyy/unreact"]
                    { i{"Unreact"}~"- An old SSG framework using Handlebars. Predecesor to"~i{"Ibex"}"" }
                }
                li { a [href="https://github.com/darccyy/lisp"]
                    { "Simple"~i{"Lisp"}"-like programming language" }
                }
            }
        }

        hr/
        div { h2 { "My Workflow" }
            p {
                "All my"~i{"dotfiles"}~"are available"
                ~a[href="https://github.com/darccyy/dotfiles"]{"here"}
                ", if you are interested."
            }
            p { b{"Operating System:"} ~ "EndevourOS (Arch-based Linux), with i3 Window Manager." }
            p { b{"Programming:"}      ~ "Tmux + Neovim + Git" }
        }

        hr/
        footer {
            "Thanks for checking out my website!"
        }
    }
    .into()
}

fn at_404() -> Document {
    view! {
        @use_base[]
        h1 { "404 - Not found" }
        a [href=url!()] { "Did you mean to go the main page?" }
    }
    .into()
}

fn use_base() -> View {
    view! {
        HEAD {
            @use_meta [Meta::new()
                .title("my website")
            ]
            title { "my website" }
            link [rel="shortcut icon", href=url!("static/icon.png")]/
            link [rel="stylesheet", href=url!("css/base.css")]/
        }
    }
}
