use crate::parser::Html5Parser;
use cow_utils::CowUtils;
use gosub_interface::config::HasDocument;
use gosub_interface::node::QuirksMode;

impl<C: HasDocument> Html5Parser<'_, C> {
    // returns the correct quirk mode for the given doctype
    pub(crate) fn identify_quirks_mode(
        &self,
        name: &Option<String>,
        pub_identifier: Option<String>,
        sys_identifier: Option<String>,
        force_quirks: bool,
    ) -> QuirksMode {
        if force_quirks || name.as_ref().map_or("", |s| &s[..]) != "html" {
            return QuirksMode::Quirks;
        }

        if let Some(value) = pub_identifier {
            let pub_id = value.cow_to_lowercase();
            if QUIRKS_PUB_IDENTIFIER_EQ.contains(&pub_id.as_ref()) {
                return QuirksMode::Quirks;
            }

            if QUIRKS_PUB_IDENTIFIER_PREFIX
                .iter()
                .any(|&prefix| pub_id.as_ref().starts_with(prefix))
            {
                return QuirksMode::Quirks;
            }

            if sys_identifier.is_some()
                && LIMITED_QUIRKS_PUB_IDENTIFIER_PREFIX_NOT_MISSING_SYS
                    .iter()
                    .any(|&prefix| pub_id.as_ref().starts_with(prefix))
            {
                return QuirksMode::LimitedQuirks;
            }

            if sys_identifier.is_none()
                && QUIRKS_PUB_IDENTIFIER_PREFIX_MISSING_SYS
                    .iter()
                    .any(|&prefix| pub_id.as_ref().starts_with(prefix))
            {
                return QuirksMode::Quirks;
            }

            if LIMITED_QUIRKS_PUB_IDENTIFIER_PREFIX
                .iter()
                .any(|&prefix| pub_id.as_ref().starts_with(prefix))
            {
                return QuirksMode::LimitedQuirks;
            }
        }

        if let Some(value) = sys_identifier {
            let sys_id = value.cow_to_lowercase();
            if QUIRKS_SYS_IDENTIFIER_EQ
                .iter()
                .any(|&prefix| sys_id.as_ref().starts_with(prefix))
            {
                return QuirksMode::Quirks;
            }
        }

        QuirksMode::NoQuirks
    }
}

static QUIRKS_PUB_IDENTIFIER_EQ: &[&str] = &[
    "-//w3o//dtd w3 html strict 3.0//en//",
    "-/w3c/dtd html 4.0 transitional/en",
    "html",
];

static QUIRKS_PUB_IDENTIFIER_PREFIX: &[&str] = &[
    "+//silmaril//dtd html pro v0r11 19970101//",
    "-//as//dtd html 3.0 aswedit + extensions//",
    "-//advasoft ltd//dtd html 3.0 aswedit + extensions//",
    "-//ietf//dtd html 2.0 level 1//",
    "-//ietf//dtd html 2.0 level 2//",
    "-//ietf//dtd html 2.0 strict level 1//",
    "-//ietf//dtd html 2.0 strict level 2//",
    "-//ietf//dtd html 2.0 strict//",
    "-//ietf//dtd html 2.0//",
    "-//ietf//dtd html 2.1e//",
    "-//ietf//dtd html 3.0//",
    "-//ietf//dtd html 3.2 final//",
    "-//ietf//dtd html 3.2//",
    "-//ietf//dtd html 3//",
    "-//ietf//dtd html level 0//",
    "-//ietf//dtd html level 1//",
    "-//ietf//dtd html level 2//",
    "-//ietf//dtd html level 3//",
    "-//ietf//dtd html strict level 0//",
    "-//ietf//dtd html strict level 1//",
    "-//ietf//dtd html strict level 2//",
    "-//ietf//dtd html strict level 3//",
    "-//ietf//dtd html strict//",
    "-//ietf//dtd html//",
    "-//metrius//dtd metrius presentational//",
    "-//microsoft//dtd internet explorer 2.0 html strict//",
    "-//microsoft//dtd internet explorer 2.0 html//",
    "-//microsoft//dtd internet explorer 2.0 tables//",
    "-//microsoft//dtd internet explorer 3.0 html strict//",
    "-//microsoft//dtd internet explorer 3.0 html//",
    "-//microsoft//dtd internet explorer 3.0 tables//",
    "-//netscape comm. corp.//dtd html//",
    "-//netscape comm. corp.//dtd strict html//",
    "-//o'reilly and associates//dtd html 2.0//",
    "-//o'reilly and associates//dtd html extended 1.0//",
    "-//o'reilly and associates//dtd html extended relaxed 1.0//",
    "-//sq//dtd html 2.0 hotmetal + extensions//",
    "-//softquad software//dtd hotmetal pro 6.0::19990601::extensions to html 4.0//",
    "-//softquad//dtd hotmetal pro 4.0::19971010::extensions to html 4.0//",
    "-//spyglass//dtd html 2.0 extended//",
    "-//sun microsystems corp.//dtd hotjava html//",
    "-//sun microsystems corp.//dtd hotjava strict html//",
    "-//w3c//dtd html 3 1995-03-24//",
    "-//w3c//dtd html 3.2 draft//",
    "-//w3c//dtd html 3.2 final//",
    "-//w3c//dtd html 3.2//",
    "-//w3c//dtd html 3.2s draft//",
    "-//w3c//dtd html 4.0 frameset//",
    "-//w3c//dtd html 4.0 transitional//",
    "-//w3c//dtd html experimental 19960712//",
    "-//w3c//dtd html experimental 970421//",
    "-//w3c//dtd w3 html//",
    "-//w3o//dtd w3 html 3.0//",
    "-//webtechs//dtd mozilla html 2.0//",
    "-//webtechs//dtd mozilla html//",
];

static QUIRKS_PUB_IDENTIFIER_PREFIX_MISSING_SYS: &[&str] = &[
    "-//w3c//dtd html 4.01 frameset//",
    "-//w3c//dtd html 4.01 transitional//",
];

static QUIRKS_SYS_IDENTIFIER_EQ: &[&str] = &["http://www.ibm.com/data/dtd/v11/ibmxhtml1-transitional.dtd"];

static LIMITED_QUIRKS_PUB_IDENTIFIER_PREFIX: &[&str] = &[
    "-//w3c//dtd xhtml 1.0 frameset//",
    "-//w3c//dtd xhtml 1.0 transitional//",
];

static LIMITED_QUIRKS_PUB_IDENTIFIER_PREFIX_NOT_MISSING_SYS: &[&str] = &[
    "-//w3c//dtd html 4.01 frameset//",
    "-//w3c//dtd html 4.01 transitional//",
];

#[cfg(test)]
mod tests {
    use crate::document::document_impl::DocumentImpl;
    use crate::document::fragment::DocumentFragmentImpl;
    use crate::parser::Html5Parser;
    use crate::parser::QuirksMode;
    use crate::DocumentBuilderImpl;
    use gosub_css3::system::Css3System;
    use gosub_interface::config::{HasCssSystem, HasDocument};
    use gosub_interface::document::DocumentBuilder;
    use gosub_shared::byte_stream::{ByteStream, Encoding, Location};

    #[derive(Clone, Debug, PartialEq)]
    struct Config;

    impl HasCssSystem for Config {
        type CssSystem = Css3System;
    }
    impl HasDocument for Config {
        type Document = DocumentImpl<Self>;
        type DocumentFragment = DocumentFragmentImpl<Self>;
        type DocumentBuilder = DocumentBuilderImpl;
    }

    type Parser<'a> = Html5Parser<'a, Config>;

    #[test]
    fn test_quirks_mode() {
        let mut stream = ByteStream::new(Encoding::UTF8, None);
        let mut document = <DocumentBuilderImpl as DocumentBuilder<Config>>::new_document(None);
        let parser = Parser::new_parser(&mut stream, &mut document, Location::default());

        assert_eq!(
            parser.identify_quirks_mode(&None, None, None, false),
            QuirksMode::Quirks
        );
        assert_eq!(
            parser.identify_quirks_mode(&Some("html".to_string()), None, None, false),
            QuirksMode::NoQuirks
        );
        assert_eq!(
            parser.identify_quirks_mode(&Some("HTML".to_string()), None, None, false),
            QuirksMode::Quirks
        );
        assert_eq!(
            parser.identify_quirks_mode(&Some("HTml".to_string()), None, None, false),
            QuirksMode::Quirks
        );
        assert_eq!(
            parser.identify_quirks_mode(
                &Some("html".to_string()),
                Some("-//W3O//DTD W3 HTML Strict 3.0//EN//".to_string()),
                None,
                false
            ),
            QuirksMode::Quirks
        );
        assert_eq!(
            parser.identify_quirks_mode(
                &Some("html".to_string()),
                Some("-//W3C//DTD HTML 4.0 Transitional//EN".to_string()),
                None,
                false
            ),
            QuirksMode::Quirks
        );
        assert_eq!(
            parser.identify_quirks_mode(
                &Some("html".to_string()),
                Some("-/W3C/DTD HTML 4.0 Transitional/EN".to_string()),
                None,
                false
            ),
            QuirksMode::Quirks
        );
        assert_eq!(
            parser.identify_quirks_mode(
                &Some("html".to_string()),
                Some("-/W3C/DTD HTML 4.0 Transitional/EN".to_string()),
                None,
                false
            ),
            QuirksMode::Quirks
        );
        assert_eq!(
            parser.identify_quirks_mode(
                &Some("html".to_string()),
                Some("-//W3C//DTD HTML 4.01 Frameset//".to_string()),
                None,
                false
            ),
            QuirksMode::Quirks
        );
        assert_eq!(
            parser.identify_quirks_mode(
                &Some("html".to_string()),
                Some("-//W3C//DTD HTML 4.01 Transitional//".to_string()),
                None,
                false
            ),
            QuirksMode::Quirks
        );
        assert_eq!(
            parser.identify_quirks_mode(
                &Some("html".to_string()),
                Some("-//W3C//DTD XHTML 1.0 Frameset//EN".to_string()),
                None,
                false
            ),
            QuirksMode::LimitedQuirks
        );
    }

    #[test]
    fn test_quirks_mode_force() {
        let mut stream = ByteStream::new(Encoding::UTF8, None);
        let mut document = <DocumentBuilderImpl as DocumentBuilder<Config>>::new_document(None);
        let parser = Parser::new_parser(&mut stream, &mut document, Location::default());

        assert_eq!(
            parser.identify_quirks_mode(&Some("html".to_string()), None, None, true),
            QuirksMode::Quirks
        );
        assert_eq!(
            parser.identify_quirks_mode(
                &Some("html".to_string()),
                Some("-//W3O//DTD W3 HTML Strict 3.0//EN//".to_string()),
                None,
                true
            ),
            QuirksMode::Quirks
        );
        assert_eq!(
            parser.identify_quirks_mode(
                &Some("html".to_string()),
                Some("-//W3C//DTD HTML 4.0 Transitional//EN".to_string()),
                None,
                true
            ),
            QuirksMode::Quirks
        );
        assert_eq!(
            parser.identify_quirks_mode(
                &Some("html".to_string()),
                Some("-/W3C/DTD HTML 4.0 Transitional/EN".to_string()),
                None,
                true
            ),
            QuirksMode::Quirks
        );
        assert_eq!(
            parser.identify_quirks_mode(
                &Some("html".to_string()),
                Some("-/W3C/DTD HTML 4.0 Transitional/EN".to_string()),
                None,
                true
            ),
            QuirksMode::Quirks
        );
        assert_eq!(
            parser.identify_quirks_mode(
                &Some("html".to_string()),
                Some("-//W3C//DTD HTML 4.01 Frameset//".to_string()),
                None,
                true
            ),
            QuirksMode::Quirks
        );
        assert_eq!(
            parser.identify_quirks_mode(
                &Some("html".to_string()),
                Some("-//W3C//DTD HTML 4.01 Transitional//".to_string()),
                None,
                true
            ),
            QuirksMode::Quirks
        );
        assert_eq!(
            parser.identify_quirks_mode(
                &Some("html".to_string()),
                Some("-//W3C//DTD XHTML 1.0 Frameset//".to_string()),
                None,
                true
            ),
            QuirksMode::Quirks
        );
    }

    #[test]
    fn test_quirks_mode_sys() {
        let mut stream = ByteStream::new(Encoding::UTF8, None);
        let mut document = <DocumentBuilderImpl as DocumentBuilder<Config>>::new_document(None);
        let parser = Parser::new_parser(&mut stream, &mut document, Location::default());

        assert_eq!(
            parser.identify_quirks_mode(
                &Some("html".to_string()),
                Some("-//W3C//DTD HTML 4.0 Transitional//EN".to_string()),
                Some("http://www.w3.org/TR/html4/loose.dtd".to_string()),
                false
            ),
            QuirksMode::Quirks
        );
        assert_eq!(
            parser.identify_quirks_mode(
                &Some("html".to_string()),
                Some("-//W3C//DTD HTML 4.01 Frameset//".to_string()),
                Some("http://www.w3.org/TR/html4/frameset.dtd".to_string()),
                false
            ),
            QuirksMode::LimitedQuirks
        );
    }

    #[test]
    fn test_quirks_mode_sys_missing() {
        let mut stream = ByteStream::new(Encoding::UTF8, None);
        let mut document = <DocumentBuilderImpl as DocumentBuilder<Config>>::new_document(None);
        let parser = Parser::new_parser(&mut stream, &mut document, Location::default());

        assert_eq!(
            parser.identify_quirks_mode(
                &Some("html".to_string()),
                Some("-//W3C//DTD HTML 4.01 Frameset//".to_string()),
                None,
                false
            ),
            QuirksMode::Quirks
        );
    }
}
