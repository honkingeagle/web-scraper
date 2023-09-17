#[cfg(test)]
mod tests {
    use scraper::{Html, Selector};

    #[test]
    fn select_h1() {
        let html = r#"
            <h1>Hello World!</h1>
        "#;

        let fragment = Html::parse_fragment(html);
        let h1_selector = Selector::parse("h1").unwrap();
        let element = fragment.select(&h1_selector).next().unwrap();
        assert_eq!("h1", element.value().name());
    }

    #[test]
    fn first_li_text_node() {
        let html = r#"
        <ul>
            <li>Foo</li>
            <li>Bar</li>
            <li>Baz</li>
        </ul>
    "#;

        let fragment = Html::parse_fragment(html);
        let selector = Selector::parse("li").unwrap();

        let element = fragment.select(&selector).next().unwrap();
        let text = element.text().collect::<Vec<_>>();
        assert_eq!(vec!["Foo"], text);
    }
}
