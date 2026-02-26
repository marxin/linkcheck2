use codespan::Span;
use pulldown_cmark::{BrokenLink, CowStr, Event, Options, Parser, Tag};

/// A scanner that uses [`pulldown_cmark`] to extract all links from markdown.
///
/// # Examples
///
/// ```rust
/// # use codespan::Span;
/// let src = "This is a [link](https://example.com/) and an ![Image](img.png)";
///
/// let got: Vec<_> = linkcheck2::scanners::markdown(src).collect();
///
/// assert_eq!(got.len(), 2);
/// let (href, span) = &got[0];
/// assert_eq!(href, "https://example.com/");
/// assert_eq!(*span, Span::new(10, 38));
/// ```
pub fn markdown(src: &str) -> impl Iterator<Item = (String, Span)> + '_ {
    markdown_with_broken_link_callback(src, None)
}

/// The callback passed to `pulldown-cmark` whenever a broken link is
/// encountered.
pub type BrokenLinkCallback<'src> = dyn FnMut(BrokenLink<'_>) -> std::option::Option<(CowStr<'src>, CowStr<'src>)>
    + 'src;

/// A scanner that uses [`pulldown_cmark`] to extract all links from markdown,
/// using the supplied callback to try and fix broken links.
pub fn markdown_with_broken_link_callback<'a>(
    src: &'a str,
    on_broken_link: Option<&'a mut BrokenLinkCallback<'a>>,
) -> impl Iterator<Item = (String, Span)> + 'a {
    Parser::new_with_broken_link_callback(
        src,
        Options::ENABLE_FOOTNOTES,
        on_broken_link,
    )
    .into_offset_iter()
    .filter_map(|(event, range)| match event {
        Event::Start(Tag::Link { dest_url, .. })
        | Event::Start(Tag::Image { dest_url, .. }) => Some((
            dest_url.to_string(),
            Span::new(range.start as u32, range.end as u32),
        )),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Range;

    #[track_caller]
    fn check_links(
        input: &str,
        expected_links: &[(&'static str, Span)],
        expected_broken_links: &[(&'static str, Range<usize>)],
    ) {
        let mut actual_broken_links = Vec::new();
        let actual_links: Vec<_> = markdown_with_broken_link_callback(
            input,
            Some(&mut |broken_link| {
                actual_broken_links.push((
                    broken_link.reference.to_string(),
                    broken_link.span,
                ));
                None
            }),
        )
        .collect();

        let actual_links_refs: Vec<_> = actual_links
            .iter()
            .map(|(s, span)| (s.as_str(), *span))
            .collect();
        let actual_broken_links: Vec<_> = actual_broken_links
            .iter()
            .map(|(s, span)| (s.as_str(), span.clone()))
            .collect();
        assert_eq!(actual_links_refs, expected_links);
        assert_eq!(actual_broken_links, expected_broken_links);
    }

    #[test]
    fn detect_common_links_in_markdown() {
        let src = r#"
# Some Heading

[this](https://example.com) is a link [to nowhere][nowhere]. But
[this](../README.md) points somewhere on disk.

![Look, an image!](https://imgur.com/gallery/f28OkrB)

[nowhere]: https://dev.null/
        "#;
        check_links(
            src,
            &[
                ("https://example.com", Span::new(17, 44)),
                ("https://dev.null/", Span::new(55, 76)),
                ("../README.md", Span::new(82, 102)),
                ("https://imgur.com/gallery/f28OkrB", Span::new(130, 183)),
            ],
            &[],
        );
    }
}
