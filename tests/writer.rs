extern crate safe_buffer;

#[allow(unused_must_use)]
mod test {
    use safe_buffer::Escapable;
    use safe_buffer::SafeWriter;
    use safe_buffer::SafeWrite;

    #[test]
    fn escape_unsafe_string() {
        let mut buffer = SafeWriter::new(vec![]);

        buffer.write_str(String::from("<hello>&world</hello>"));
        assert_eq!("&lt;hello&gt;&amp;world&lt;&#x2F;hello&gt;", String::from_utf8(buffer.into_inner()).unwrap());
    }

    #[test]
    fn escape_unsafe_str() {
        let mut buffer = SafeWriter::new(vec![]);

        buffer.write_str("<hello>&world</hello>");
        assert_eq!("&lt;hello&gt;&amp;world&lt;&#x2F;hello&gt;", String::from_utf8(buffer.into_inner()).unwrap());
    }

    #[test]
    fn escape_safe_str() {
        let mut buffer = SafeWriter::new(vec![]);
        let s = "<hello>&world</hello>".escape();

        buffer.write_str(s);
        assert_eq!("&lt;hello&gt;&amp;world&lt;&#x2F;hello&gt;", String::from_utf8(buffer.into_inner()).unwrap());
    }

    #[test]
    fn mark_string_safe() {
        let mut buffer = SafeWriter::new(vec![]);

        buffer.write_str("<hello>&world</hello>".safe());
        assert_eq!("<hello>&world</hello>", String::from_utf8(buffer.into_inner()).unwrap());
    }
}