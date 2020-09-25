Novel
=====

Novel is a new html generation tool.
 Currently, "server side" rendering is the only thing possible,
  but potentially in the future Novel could be compiled to Webassembly and used for client side rendering. This would create a united codebase, but that is not currently the focus.

Novel does not use normal html, but rather S-Experssions (Similiar to lisp syntax).
 This is due to how procedural macros are given a token stream, and my inability to "fix" it.

Currently, Novel only supports flat elements, as in <\p>, <\h>.
 Body text is the only thing currently supported, But in the future I plan to work to increase this.
