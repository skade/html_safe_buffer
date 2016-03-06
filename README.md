# html\_safe\_buffer

[![Clippy Linting Result](http://clippy.bashy.io/github/skade/html_safe_buffer/master/badge.svg)](http://clippy.bashy.io/github/skade/html_safe_buffer/master/log)

`html\_safe\_buffer` is an implementation of ActiveSupports SafeBuffer. It is a String buffer with a twist: if you write a normal String to the Buffer, it is HTML-escaped first. If you write an escaped String to the buffer, it will be left untouched. This prevents accidental writes of untrusted data to HTML buffers.