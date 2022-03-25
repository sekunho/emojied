use maud::{html, Markup, DOCTYPE};

fn header(title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        link rel="stylesheet" href="app.css";
        title { (title) }
    }
}

pub fn home() -> Markup {
    html! {
        (header("emojied"))
        h1 class="text-red-400" { "Hello, world!" };

        @let a = 2;

        @if a == 1 {
            h2 class="text-red-500" { "Hey again, world!" }
        }

        form action="/" method="POST" {
            input type="text" name="url";
            " "
            label for="url" { "Enter a URL to shorten" }
            br;
            input type="text" name="identifier";
            " "
            label for="identifier" { "Enter a URL to shorten" }
            button type="submit" { "Submit" }
        }
    }
}

