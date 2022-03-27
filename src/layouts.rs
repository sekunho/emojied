pub mod root;

use maud::{html, Markup, DOCTYPE, PreEscaped};

fn header(title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        link rel="stylesheet" href="app.css";
        link rel="preconnect" href="https://fonts.googleapis.com";
        link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
        link href="https://fonts.googleapis.com/css2?family=Vollkorn:wght@600&display=swap&text=emojidt" rel="stylesheet";

        script src="purify.min.js" {}

        title { (title) }
    }
}

fn footer() -> Markup {
    html! {
        footer class="bg-gray-100 dark:bg-su-dark-bg-1" {
            div class="max-w-7xl mx-auto pb-6 pt-2 px-4 sm:px-6 md:flex md:items-center md:justify-between lg:px-8 text-su-fg-1 dark:text-su-dark-fg-1" {
                span {
                    "Made with "
                    span class="font-bold " { "regret" }
                    " by "
                    a href="https://twitter.com/sekunho_" target="_blank" class="underline decoration-wavy decoration-red-500 hover:font-bold" {
                        "SEKUN"
                    }
                }

                nav class="space-x-4" {
                    a href="#" {
                        "Twitter"
                    }

                    a href="#" {
                        "YouTube"
                    }

                    a href="#" {
                        "GitHub"
                    }
                }
            }
        }

    }
}

pub fn wrapper(inner_content: Markup) -> Markup {
    html! {
        (header("emojied - Emoji URL Shortener"))

        body class="min-h-screen bg-gray-100 dark:bg-su-dark-bg-1 flex flex-col" {
            main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 w-full flex flex-col flex-1" {
                div class="w-full max-w-3xl mx-auto flex-1 flex flex-col" {
                    (inner_content)
                }
            }

            (footer())

            script type="application/javascript" {
                (PreEscaped("let BASE_URL = 'localhost:3000';\nlet SCHEME = 'http';"))
            }

            script src="app.js" {}
        }
    }
}


