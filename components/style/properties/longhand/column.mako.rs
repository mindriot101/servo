/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

<%namespace name="helpers" file="/helpers.mako.rs" />

<% data.new_style_struct("Column", inherited=False) %>

// FIXME: This prop should be animatable.
${helpers.predefined_type("column-width",
                          "length::LengthOrAuto",
                          "Either::Second(Auto)",
                          parse_method="parse_non_negative_length",
                          animatable=False,
                          experimental=True)}


// FIXME: This prop should be animatable.
<%helpers:longhand name="column-count" experimental="True" animatable="False">
    use std::fmt;
    use style_traits::ToCss;
    use values::NoViewportPercentage;

    impl NoViewportPercentage for SpecifiedValue {}

    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "servo", derive(HeapSizeOf))]
    pub enum SpecifiedValue {
        Auto,
        Specified(u32),
    }

    impl ToCss for SpecifiedValue {
        fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            match *self {
                SpecifiedValue::Auto => dest.write_str("auto"),
                SpecifiedValue::Specified(count) => write!(dest, "{}", count),
            }
        }
    }

    pub mod computed_value {
        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "servo", derive(HeapSizeOf))]
        pub struct T(pub Option<u32>);
    }

    impl ToCss for computed_value::T {
        fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            match self.0 {
                None => dest.write_str("auto"),
                Some(count) => write!(dest, "{}", count),
            }
        }
    }

    #[inline]
    pub fn get_initial_value() -> computed_value::T {
        computed_value::T(None)
    }

    impl ToComputedValue for SpecifiedValue {
        type ComputedValue = computed_value::T;

        #[inline]
        fn to_computed_value(&self, _context: &Context) -> computed_value::T {
            match *self {
                SpecifiedValue::Auto => computed_value::T(None),
                SpecifiedValue::Specified(count) =>
                    computed_value::T(Some(count))
            }
        }

        #[inline]
        fn from_computed_value(computed: &computed_value::T) -> Self {
            match *computed {
                computed_value::T(None) => SpecifiedValue::Auto,
                computed_value::T(Some(count)) =>
                    SpecifiedValue::Specified(count)
            }
        }
    }

    pub fn parse(_context: &ParserContext, input: &mut Parser) -> Result<SpecifiedValue, ()> {
        if input.try(|input| input.expect_ident_matching("auto")).is_ok() {
            Ok(SpecifiedValue::Auto)
        } else {
            let count = try!(specified::parse_integer(input));
            // Zero is invalid
            if count <= 0 {
                return Err(())
            }
            Ok(SpecifiedValue::Specified(count as u32))
        }
    }
</%helpers:longhand>

// FIXME: This prop should be animatable.
<%helpers:longhand name="column-gap" experimental="True" animatable="False">
    use std::fmt;
    use style_traits::ToCss;
    use values::HasViewportPercentage;

    impl HasViewportPercentage for SpecifiedValue {
        fn has_viewport_percentage(&self) -> bool {
            match *self {
                SpecifiedValue::Specified(length) => length.has_viewport_percentage(),
                _ => false
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "servo", derive(HeapSizeOf))]
    pub enum SpecifiedValue {
        Normal,
        Specified(specified::Length),
    }

    impl ToCss for SpecifiedValue {
        fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            match *self {
                SpecifiedValue::Normal => dest.write_str("normal"),
                SpecifiedValue::Specified(l) => l.to_css(dest),
            }
        }
    }

    pub mod computed_value {
        use app_units::Au;
        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "servo", derive(HeapSizeOf))]
        pub struct T(pub Option<Au>);
    }

    impl ToCss for computed_value::T {
        fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            match self.0 {
                None => dest.write_str("normal"),
                Some(l) => l.to_css(dest),
            }
        }
    }

    #[inline]
    pub fn get_initial_value() -> computed_value::T {
        computed_value::T(None)
    }

    impl ToComputedValue for SpecifiedValue {
        type ComputedValue = computed_value::T;

        #[inline]
        fn to_computed_value(&self, context: &Context) -> computed_value::T {
            match *self {
                SpecifiedValue::Normal => computed_value::T(None),
                SpecifiedValue::Specified(l) =>
                    computed_value::T(Some(l.to_computed_value(context)))
            }
        }
        #[inline]
        fn from_computed_value(computed: &computed_value::T) -> Self {
            match *computed {
                computed_value::T(None) => SpecifiedValue::Normal,
                computed_value::T(Some(l)) =>
                    SpecifiedValue::Specified(ToComputedValue::from_computed_value(&l))
            }
        }
    }

    pub fn parse(_context: &ParserContext, input: &mut Parser) -> Result<SpecifiedValue, ()> {
        if input.try(|input| input.expect_ident_matching("normal")).is_ok() {
            Ok(SpecifiedValue::Normal)
        } else {
            specified::Length::parse_non_negative(input).map(SpecifiedValue::Specified)
        }
    }
</%helpers:longhand>

${helpers.single_keyword("column-fill", "auto balance",
                         products="gecko", animatable=False)}

// https://drafts.csswg.org/css-multicol-1/#propdef-column-rule-width
<%helpers:longhand name="-moz-column-rule-width" products="gecko" animatable="True">
    use app_units::Au;
    use std::fmt;
    use style_traits::ToCss;
    use values::HasViewportPercentage;
    use values::specified::BorderWidth;

    pub mod computed_value {
        use app_units::Au;
        pub type T = Au;
    }

    pub type SpecifiedValue = BorderWidth;

    #[inline]
    pub fn get_initial_value() -> computed_value::T {
        Au::from_px(3) // medium
    }

    #[inline]
    pub fn get_initial_specified_value() -> SpecifiedValue {
        BorderWidth::Medium
    }

    pub fn parse(context: &ParserContext, input: &mut Parser) -> Result<SpecifiedValue, ()> {
        BorderWidth::parse(context, input)
    }
</%helpers:longhand>

// https://drafts.csswg.org/css-multicol-1/#crc
${helpers.predefined_type("-moz-column-rule-color", "CSSColor",
                          "::cssparser::Color::CurrentColor",
                          products="gecko", gecko_ffi_name="mColumnRuleColor",
                          animatable=True, complex_color=True, need_clone=True)}

// It's not implemented in servo or gecko yet.
// https://drafts.csswg.org/css-multicol-1/#column-span
${helpers.single_keyword("column-span", "none all",
                         products="none", animatable=False)}

${helpers.single_keyword("-moz-column-rule-style",
                         "none hidden dotted dashed solid double groove ridge inset outset",
                         products="gecko",
                         gecko_ffi_name="mColumnRuleStyle",
                         gecko_constant_prefix="NS_STYLE_BORDER_STYLE",
                         animatable=False)}
