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
                    "I like programming and languages."
                    ~"I speak English, Esperanto, and Rust."
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
                h3 { "Rust" }
                p { "Rust is my favourite language, due to its type system, macros, and speed." }
                ul {
                    li { "CLI apps and simple tools" }
                    li { "Static site generation frameworks" }
                    li { "Procedural macros" }
                    li { "Egui (UI framework)" }
                    li { "GGez (Graphics framework)" }
                }
                h3 { "Javascript / Typescript" }
                ul {
                    li { "Client-side vanilla javascript with HTML and CSS or SCSS" }
                    li { "Server-side Node.JS, Express" }
                    li { "React single page applications" }
                }
                h3 { "Other" }
                ul {
                    li { "Posix-compliant shell scripting (like Bash)" }
                    li { "Some Lua, Haskell, and Java" }
                    li { "Popular CLI tools, like coreutils, FFmpeg, etc." }
                    li { "Previously, Python, Handlebars, and AutoHotkey, but I have not used them in a while." }
                }
                p { "I am currently learning Zig and Go" }
            }

            hr/
            article { h2 #"projects" { "Projects" }
                ul ."big-list" {
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
                    li { h3 { em{"CTTab"} ~ "- A customizable 'new tab' page for the browser" }
                        p {
                            "Add a background, shortcuts, and notes to your browser's homepage."
                            ~ "Supports Chromium and Firefox based browsers."
                        }
                        blockquote {
                            "Check out"
                            ~ a [href=format!("{URL_GITHUB}/cttab")]
                                { "CTTab on GitHub" }
                           ", or"
                            ~ a [href=format!("{URL_DOMAIN}/cttab")]
                                { "a live example" }
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
                    li { h3 { em{"The Trustworthy Times"} ~ "- A satirical news website (funny)" }
                        p {
                            "A silly little static website with funny fake news articles."
                            ~ "I might one day port it to"~i{"Ibex"}"."
                        }
                        blockquote {
                            "Check out"
                            ~ a [href=format!("{URL_DOMAIN}/trustworthytimes")]
                                { i{"The Trustworthy Times"} }
                            ", or"
                            ~ a [href=format!("{URL_GITHUB}/trustworthytimes")]
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
                p { "Which are either unmaintained or less interesting." }
                ul ."small-list" {
                    li { a [href=format!("{URL_DOMAIN}/color")]
                        { "Colour Sliders" }
                    }
                    li { a [href=format!("{URL_GITHUB}/recipe-lang")]
                        { "Programming language written like a cooking recipe" }
                    }
                    li { a [href=format!("{URL_GITHUB}/mcimg")]
                        { "Convert pixels of an image into"~i{"Minecraft"}~"blocks" }
                    }
                    li { a [href=format!("{URL_GITHUB}/unreact")]
                        { i{"Unreact"}~"- An old SSG framework using Handlebars. Predecesor to"~i{"Ibex"}"" }
                    }
                    li { a [href=format!("{URL_GITHUB}/lisp")]
                        { "Simple"~i{"Lisp"}"-like programming language" }
                    }
                    li { a [href=format!("{URL_GITHUB}/scripts")]
                        { "Some POSIX-compliant shell scripts" }
                    }
                }
            }

            hr/
            article { h2 #"workflow" { "Workflow" }
                p {
                    "All my"~i{"dotfiles"}~"are available"
                    ~a[href=format!("{URL_GITHUB}/dotfiles")]{"here"}
                    ", if you are interested."
                }
                p { strong{"Operating System:"} ~ "EndevourOS (Arch-based Linux), with i3 Window Manager." }
                p { strong{"Programming:"}      ~ "Tmux + Neovim + Git" }
            }
        }

        footer {
            "Thanks for checking out my website!"
        }
    }
    .into()
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

