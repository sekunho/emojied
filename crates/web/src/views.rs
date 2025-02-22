pub mod leaderboard;
pub mod root;
pub mod status;
pub mod url;

use crate::components::icon;
use maud::{html, Markup, DOCTYPE};

fn header(title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        link rel="stylesheet" href="/assets/app.css";
        link rel="preconnect" href="https://fonts.googleapis.com";
        link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
        link href="https://fonts.googleapis.com/css2?family=Vollkorn:wght@600&display=swap&text=emojidtladrbo" rel="stylesheet";
        meta name="viewport" content="width=device-width, initial-scale=1.0";

        script src="/assets/purify.min.js" {}

        title { (title) }
    }
}

fn footer() -> Markup {
    html! {
        footer class="bg-gray-100 dark:bg-su-dark-bg-1" {
            div class="max-w-7xl mx-auto pb-6 pt-2 px-4 sm:px-6 flex flex-col-reverse items-center md:flex-row md:items-center md:justify-between lg:px-8 text-su-fg-1 dark:text-su-dark-fg-1" {
                span class="mt-4 md:mt-0" {
                    "Made with "
                    span class="font-bold " { "regret" }
                    " by "
                    a href="https://sekun.dev" target="_blank" class="underline decoration-wavy decoration-red-500" {
                        "SEKUN"
                    }
                    " © 2022"
                }

                nav class="space-y-1 sm:space-y-0 space-x-5 flex items-end" {
                    a href="/" {
                        "Home"
                    }

                    a href="/leaderboard" {
                        "Rank"
                    }

                    a target="_blank" href="https://ko-fi.com/sekun" {
                        "Donate"
                    }

                    a
                        target="_blank"
                        href="mailto:report@emojied.net?subject=Malicious%20Url"
                        title="Report a malicious URL" {
                        ("Report")
                    }

                    a target="_blank" href="https://twitter.com/sekunho_" {
                        (icon::twitter())
                    }

                    a target="_blank" href="https://youtube.com/sekunho" {
                        (icon::youtube())
                    }

                    a target="_blank" href="https://github.com/sekunho/emojied" {
                        (icon::github())
                    }
                }
            }
        }

    }
}

pub fn wrapper(inner_content: &Markup) -> Markup {
    html! {
        (header("emojied - Emoji URL Shortener"))

        body class="min-h-screen bg-gray-100 dark:bg-su-dark-bg-1 flex flex-col" {
            main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 w-full flex flex-col flex-1" {
                div class="w-full max-w-3xl mx-auto flex-1 flex flex-col" {
                    (inner_content)
                }
            }

            (footer())

            script src="/assets/app.js" {}
        }
    }
}

// fn nav() -> Markup {
//     html! {
//         nav class="w-full max-w-7xl mx-auto p-2.5 flex items-end justify-end text-su-fg-1 dark:text-su-dark-fg-1 font-medium" {
//             a href="/leaderboard" {
//                 "Leaderboard"
//             }
//         }
//     }
// }
