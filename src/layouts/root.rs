use maud::{html, Markup};
use rand::prelude::*;

use crate::components::icon;
use crate::layouts::wrapper;

// I could probably make this more domain-specific, but it doesn't feel right.
pub struct RootData {
    pub custom_url: bool,
    pub identifier: Option<String>,
}

/// Renders the `root` page template
pub fn render(data: RootData) -> Markup {
    let home_content = html! {
        main class="space-y-4 flex flex-col items-center justify-center flex-1" {
            (heading())

            (form(data.custom_url))

            noscript class="w-full sm:w-4/5" {
                @if let Some(emojis) = data.identifier {
                    div class="divide-y divide-su-bg-2 dark:divide-su-dark-bg-2 shadow-md px-2.5 bg-su-bg-2 dark:bg-black/[0.3] rounded-md border border-su-dark-bg-2 mx-auto"
                        id="url-list" {
                        div class="py-2 flex justify-between text-su-fg-1 dark:text-su-dark-fg-1" {
                            a href=(format!("/{}", emojis)) {
                                (format!("emojied.net/{}", emojis))
                            }

                            div class="flex space-x-2.5 text-sm" {
                                a href=(format!("/stats/{}", emojis)) {
                                    (icon::chart_bar())
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    wrapper(home_content)
}

fn form(custom_url: bool) -> Markup {
    html! {
        form
            id="url-form"
            class="flex flex-col w-full"
            action=(if custom_url { "/?custom_url=t" } else { "/" })
            method="POST" {
            div class="mx-auto flex w-full sm:w-4/5 shadow-md dark:shadow-black/[0.2]" {
                input
                    id="url"
                    autofocus="true"
                    autocomplete="off"
                    class="flex-1 outline-none placeholder:text-gray-400 dark:placeholder:text-su-dark-fg-1/[0.6] text-su-fg-1 dark:text-su-dark-fg-1 w-10/12 rounded-l-md bg-white dark:bg-su-dark-bg-2 p-2.5 text-lg"
                    type="text"
                    placeholder="https://youtube.com/sekunho"
                    name="url"
                    required;

                button title="Why?" class="hover:bg-gray-100 dark:hover:bg-white/[0.1] flex items-center justify-center flex-none w-12 p-1 border-red-200 rounded-r-md bg-white dark:bg-su-dark-bg-2 text-su-fg-1 dark:text-su-dark-fg-1" type="submit" { (icon::arrow_right()) }
            }

            div id="identifier-field" class=(if custom_url { "" } else { "hidden" }) {
                div class="mx-auto mt-2 text-center font-serif font-semibold text-lg text-su-fg-1 dark:text-su-dark-fg-1" { "to" }

                div class="shadow-md dark:shadow-black/[0.2] mx-auto flex w-full sm:w-4/5 mt-2" {
                  div class="w-2/5 sm:w-1/5 h-full text-lg text-su-fg-1 dark:text-su-dark-fg-1 bg-gray-200 dark:bg-white/[0.3] px-2 py-2 rounded-l-md" {
                      "emojied.net/"
                  }

                  // Set required if user chose to set the custom ID
                  @if custom_url {
                      input
                          id="identifier"
                          required
                          class="w-3/5 sm:w-4/5 text-su-fg-1 dark:text-su-dark-fg-1 rounded-r-md bg-white dark:bg-su-dark-bg-2 p-2 text-lg"
                          type="text"
                          name="identifier"
                          autocomplete="off";
                  } @else {
                      input
                          id="identifier"
                          class="w-3/5 sm:w-4/5 text-su-fg-1 dark:text-su-dark-fg-1 rounded-r-md bg-white dark:bg-su-dark-bg-2 p-2 text-lg"
                          type="text"
                          name="identifier"
                          autocomplete="off";
                  }
                }
            }

            @if custom_url {
                noscript {
                    div class="w-full sm:w-4/5 mt-2 mx-auto text-su-fg-1 dark:text-su-dark-fg-1" {
                        a href="/" type="button" class="font-medium underline" {
                            "Autogenerate a custom URL for me"
                        }
                    }
                }
            } @else {
                noscript {
                    div class="w-full sm:w-4/5 mt-2 mx-auto text-su-fg-1 dark:text-su-dark-fg-1" {
                        a href="?custom_url=t" type="button" class="font-medium underline" {
                            "Custom URL"
                        }
                    }
                }
            }
        }
    }
}

fn heading() -> Markup {
    html! {
        div class="mb-4 sm:mb-8 flex flex-col justify-center" {
            a href="/" class="text-5xl sm:text-7xl font-serif font-semibold text-su-fg-1 dark:text-su-dark-fg-1 text-center" {
                "em"
                span class="text-3xl sm:text-5xl" { (get_random_emoji()) }
                "jied!"
            }

            p class="text-center text-lg" {
                span class="text-su-fg-1 dark:text-su-dark-fg-1/[0.7]" {
                    "Shorten your links with "
                }
                span class="text-su-fg-1 dark:text-su-dark-fg-1/[0.7] underline" {
                    "only"
                }
                span class="text-su-fg-1 dark:text-su-dark-fg-1/[0.7]" {
                    " emojis "
                }
                "💖✨"
            }
        }
    }
}

/// Randomly picks out an emoji from the list
fn get_random_emoji() -> String {
    let mut rng = thread_rng();

    ['🐵', '🍆', '🎱', '😶']
        .into_iter()
        .map(|el| String::from(el))
        .choose(&mut rng)
        .unwrap()
}
