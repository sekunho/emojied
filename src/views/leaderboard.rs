use maud::{html, Markup};
use crate::leaderboard;
use crate::views::wrapper;
use crate::components::icon;

pub fn render(entries: Vec<leaderboard::Entry>) -> Markup {
    let content = html! {
        div class="flex-1 flex flex-col items-center justify-center" {
            h1 class="mt-6 text-2xl sm:text-4xl text-center font-serif font-semibold text-su-fg-1 dark:text-su-dark-fg-1" {
                "Leaderboard"
            }

            div class="my-6 grid grid-cols-3 sm:grid-cols-5 w-full text-xl gap-2 sm:gap-2.5" {
                span class="text-sm text-su-fg-1/[0.8] dark:text-su-dark-fg-1/[0.8] font-semibold" { "Rank" }
                span class="text-sm text-su-fg-1/[0.8] dark:text-su-dark-fg-1/[0.8] font-semibold" { "ID" }
                span class="hidden sm:block text-sm text-su-fg-1/[0.8] dark:text-su-dark-fg-1/[0.8] font-semibold" { "URL" }
                span class="text-sm text-su-fg-1/[0.8] dark:text-su-dark-fg-1/[0.8] font-semibold text-right" { "Clicks" }
                span class="hidden sm:block text-sm text-su-fg-1/[0.8] dark:text-su-dark-fg-1/[0.8] font-semibold text-right" {
                    "Actions"
                }

                @for (rank, entry) in entries.iter().enumerate() {
                    @match rank {
                        0 => span { "🥇" },
                        1 => span {"🥈"},
                        2 => span {"🥉"},
                        _ => span class="text-su-fg-1/[0.8] dark:text-su-dark-fg-1/[0.8]" { (format!("{}", rank + 1)) }
                    }

                    a class="text-su-fg-1 dark:text-su-dark-fg-1 truncate" href=(format!("/{}", entry.identifier)) {
                        (entry.identifier)
                    }

                    span class="hidden sm:block truncate text-su-fg-1/[0.8] dark:text-su-dark-fg-1/[0.8]" title=(entry.url) { (entry.url) }
                    span class="text-su-fg-1 dark:text-su-dark-fg-1 text-right" {
                        span class="bg-indigo-500 rounded shadow-md dark:shadow-black/[0.2] px-2" {
                            (entry.clicks)
                        }
                    }

                    div class="hidden sm:flex space-x-2 justify-end text-su-fg-1/[0.8] dark:text-su-dark-fg-1/[0.8]" {
                        button
                            class="copy-button hidden"
                            title="Copy to clipboard"
                            type="button"
                            data-id = (entry.identifier) {
                            (icon::copy())
                        }

                        a target="_blank" href=(format!("/stats/{}", entry.identifier)) title="View stats" {
                            (icon::chart_bar())
                        }
                    }
                }
            }
        }
    };

    wrapper(&content)
}
