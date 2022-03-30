use maud::{Markup, html};
use super::wrapper;

pub fn not_found() -> Markup {
    let content = html! {
        div class="w-full flex-1 flex items-center justify-center flex flex-col" {
            h1 class="text-8xl font-bold text-su-fg-1 dark:text-su-dark-fg-1" {
                "NOT FOUND"
            }

            a href="/" class="underline text-su-fg-1 dark:text-su-dark-fg-1/[0.7] mt-6" {
                "Take me back home!"
            }
        }
    };

    wrapper(content)
}
