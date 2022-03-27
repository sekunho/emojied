use maud::{html, Markup, PreEscaped};

pub fn arrow_sm_down() -> Markup {
    html! {
        (PreEscaped("\
            <svg xmlns=\"http://www.w3.org/2000/svg\" class=\"h-6 w-6\" fill=\"none\" viewBox=\"0 0 24 24\" stroke=\"currentColor\" stroke-width=\"2\">
              <path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M17 13l-5 5m0 0l-5-5m5 5V6\" />
            </svg>
         "))

    }
}

pub fn arrow_right() -> Markup {
    html! {
        (PreEscaped("
            <svg xmlns=\"http://www.w3.org/2000/svg\" class=\"h-6 w-6\" fill=\"none\" viewBox=\"0 0 24 24\" stroke=\"currentColor\" stroke-width=\"2\">
              <path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M13 9l3 3m0 0l-3 3m3-3H8m13 0a9 9 0 11-18 0 9 9 0 0118 0z\" />
            </svg>
        "))
    }
}

pub fn copy() -> Markup {
    html! {
        (PreEscaped("
            <svg xmlns=\"http://www.w3.org/2000/svg\" class=\"h-6 w-6\" fill=\"none\" viewBox=\"0 0 24 24\" stroke=\"currentColor\" stroke-width=\"2\">
              <path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z\" />
            </svg>
                    "))
    }
}

pub fn chart_bar() -> Markup {
    html! {
        (PreEscaped("
            <svg xmlns=\"http://www.w3.org/2000/svg\" class=\"h-6 w-6\" fill=\"none\" viewBox=\"0 0 24 24\" stroke=\"currentColor\" stroke-width=\"2\">
              <path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z\" />
            </svg>
                    "))
    }
}
