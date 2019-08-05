#[macro_export]
macro_rules! apply_methods {
    ($this:expr, {}) => {
        $this
    };
    ($this:expr, { .$name:ident!($($args:tt)*) $($rest:tt)* }) => {
        $crate::apply_methods!({
            let this = $this;
            $name!(this, $($args)*)
        }, { $($rest)* })
    };
    ($this:expr, { .$name:ident($($args:expr),*) $($rest:tt)* }) => {
        $crate::apply_methods!($this.$name($($args),*), { $($rest)* })
    };
}


#[doc(hidden)]
#[macro_export]
macro_rules! __internal_builder {
    ($default:ty, $make:ident, $kind:expr) => {
        $crate::__internal_builder!($default, $make, $kind => $default, {})
    };
    ($default:ty, $make:ident, $kind:expr, $($rest:tt)*) => {
        $crate::__internal_builder!($default, $make, $kind => $default, $($rest)*)
    };
    ($default:ty, $make:ident, $kind:expr => $t:ty) => {
        $crate::__internal_builder!($default, $make, $kind => $t, {})
    };
    ($default:ty, $make:ident, $kind:expr => $t:ty, $($methods:tt)*) => {{
        let builder = $crate::DomBuilder::<$t>::$make($kind);
        let output = $crate::apply_methods!(builder, $($methods)*);
        $crate::DomBuilder::into_dom(output)
    }};
}


#[macro_export]
macro_rules! with_node {
    ($this:ident, $name:ident => { $($methods:tt)* }) => {{
        let $name = $crate::DomBuilder::__internal_element(&$this);
        $crate::apply_methods!($this, { $($methods)* })
    }};
}


#[macro_export]
macro_rules! html {
    ($($args:tt)+) => {
        $crate::__internal_builder!($crate::HtmlElement, new_html, $($args)+)
    };
}


#[macro_export]
macro_rules! svg {
    ($($args:tt)+) => {
        $crate::__internal_builder!($crate::SvgElement, new_svg, $($args)+)
    };
}


#[macro_export]
macro_rules! stylesheet {
    ($rule:expr) => {
        $crate::stylesheet!($rule, {})
    };
    ($rule:expr, { $($methods:tt)* }) => {
        $crate::StylesheetBuilder::__internal_done($crate::apply_methods!($crate::StylesheetBuilder::__internal_new($rule), { $($methods)* }))
    };
}


#[macro_export]
macro_rules! class {
    ($($methods:tt)*) => {{
        $crate::ClassBuilder::__internal_done($crate::apply_methods!($crate::ClassBuilder::__internal_new(), { $($methods)* }))
    }};
}


#[macro_export]
macro_rules! pseudo {
    ($this:ident, $rules:expr) => {
        $crate::pseudo!($this, $rules, {})
    };
    ($this:ident, $rules:expr, { $($methods:tt)* }) => {{
        $crate::stylesheet!($crate::__internal::Pseudo::new($crate::ClassBuilder::__internal_class_name(&$this), $rules), { $($methods)* });
        $this
    }};
}


// TODO this is pretty inefficient, it iterates over the token tree one token at a time
#[doc(hidden)]
#[macro_export]
macro_rules! __internal_clone_split {
    (($($x:ident)*), $t:ident => $y:expr) => {{
        $(let $x = ::std::clone::Clone::clone(&$x);)*
        let $t = ::std::clone::Clone::clone(&$t);
        $y
    }};
    (($($x:ident)*), $t:ident, $($after:tt)*) => {
        $crate::__internal_clone_split!(($($x)* $t), $($after)*)
    };
}

// TODO move into gloo ?
#[macro_export]
macro_rules! clone {
    ($($input:tt)*) => { $crate::__internal_clone_split!((), $($input)*) };
}
