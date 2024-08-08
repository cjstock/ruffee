use askama::Template;

#[derive(Template)]
#[template(path = "components/title_input.html")]
struct TitleInput<'a> {
    text_color: &'a str,
}
