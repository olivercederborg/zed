// This file was generated by the `theme_importer`.
// Be careful when modifying it by hand.

use gpui::rgba;

#[allow(unused)]
use crate::{
    Appearance, StatusColorsRefinement, ThemeColorsRefinement, UserFontStyle, UserFontWeight,
    UserHighlightStyle, UserSyntaxTheme, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn night_owl() -> UserThemeFamily {
    UserThemeFamily {
        name: "Night Owl".into(),
        author: "Sarah Drasner (sdras)".into(),
        themes: vec![
            UserTheme {
                name: "Night Owl".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0x5f7e97ff).into()),
                        border_variant: Some(rgba(0x5f7e97ff).into()),
                        border_focused: Some(rgba(0x122d42ff).into()),
                        border_selected: Some(rgba(0x5f7e97ff).into()),
                        border_transparent: Some(rgba(0x5f7e97ff).into()),
                        border_disabled: Some(rgba(0x5f7e97ff).into()),
                        elevated_surface_background: Some(rgba(0x011627ff).into()),
                        surface_background: Some(rgba(0x011627ff).into()),
                        background: Some(rgba(0x011627ff).into()),
                        element_background: Some(rgba(0x7e57c2cc).into()),
                        element_hover: Some(rgba(0x011627ff).into()),
                        element_selected: Some(rgba(0x234d708c).into()),
                        drop_target_background: Some(rgba(0x011627ff).into()),
                        ghost_element_hover: Some(rgba(0x011627ff).into()),
                        text: Some(rgba(0xd6deebff).into()),
                        status_bar_background: Some(rgba(0x011627ff).into()),
                        title_bar_background: Some(rgba(0x011627ff).into()),
                        toolbar_background: Some(rgba(0x011627ff).into()),
                        tab_bar_background: Some(rgba(0x011627ff).into()),
                        tab_inactive_background: Some(rgba(0x01111dff).into()),
                        tab_active_background: Some(rgba(0x0b2942ff).into()),
                        editor_background: Some(rgba(0x011627ff).into()),
                        editor_gutter_background: Some(rgba(0x011627ff).into()),
                        editor_line_number: Some(rgba(0x4b6479ff).into()),
                        editor_active_line_number: Some(rgba(0xd6deebff).into()),
                        terminal_ansi_bright_black: Some(rgba(0x575656ff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xef5350ff).into()),
                        terminal_ansi_bright_green: Some(rgba(0x22da6eff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0xffeb95ff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x82aaffff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0xc792eaff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0x7fdbcaff).into()),
                        terminal_ansi_bright_white: Some(rgba(0xffffffff).into()),
                        terminal_ansi_black: Some(rgba(0x011627ff).into()),
                        terminal_ansi_red: Some(rgba(0xef5350ff).into()),
                        terminal_ansi_green: Some(rgba(0x22da6eff).into()),
                        terminal_ansi_yellow: Some(rgba(0xc5e478ff).into()),
                        terminal_ansi_blue: Some(rgba(0x82aaffff).into()),
                        terminal_ansi_magenta: Some(rgba(0xc792eaff).into()),
                        terminal_ansi_cyan: Some(rgba(0x21c7a8ff).into()),
                        terminal_ansi_white: Some(rgba(0xffffffff).into()),
                        ..Default::default()
                    },
                    status: StatusColorsRefinement {
                        deleted: Some(rgba(0xef5350ff).into()),
                        error: Some(rgba(0xef5350ff).into()),
                        hidden: Some(rgba(0x5f7e97ff).into()),
                        hint: Some(rgba(0x969696ff).into()),
                        ..Default::default()
                    },
                    syntax: Some(UserSyntaxTheme {
                        highlights: vec![
                            (
                                "attribute".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xc5e478ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "boolean".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x82aaffff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x637777ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment.doc".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x637777ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constant".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x82aaffff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "function".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x82aaffff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "keyword".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xc792eaff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "number".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf78c6cff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "operator".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x7fdbcaff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "property".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x80cbc4ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x7fdbcaff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.bracket".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x7fdbcaff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.delimiter".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x7fdbcaff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.list_marker".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x7fdbcaff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x7fdbcaff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xecc48dff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.escape".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x82aaffff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "tag".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcaece6ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "text.literal".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xecc48dff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "type".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xc5e478ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xd7dbe0ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x7fdbcaff).into()),
                                    ..Default::default()
                                },
                            ),
                        ],
                    }),
                },
            },
            UserTheme {
                name: "Night Owl Light".into(),
                appearance: Appearance::Light,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0xd9d9d9ff).into()),
                        border_variant: Some(rgba(0xd9d9d9ff).into()),
                        border_focused: Some(rgba(0x93a1a1ff).into()),
                        border_selected: Some(rgba(0xd9d9d9ff).into()),
                        border_transparent: Some(rgba(0xd9d9d9ff).into()),
                        border_disabled: Some(rgba(0xd9d9d9ff).into()),
                        elevated_surface_background: Some(rgba(0xf0f0f0ff).into()),
                        surface_background: Some(rgba(0xf0f0f0ff).into()),
                        background: Some(rgba(0xfbfbfbff).into()),
                        element_background: Some(rgba(0x2aa298ff).into()),
                        element_hover: Some(rgba(0xd3e8f8ff).into()),
                        element_selected: Some(rgba(0xd3e8f8ff).into()),
                        ghost_element_hover: Some(rgba(0xd3e8f8ff).into()),
                        text: Some(rgba(0x403f53ff).into()),
                        status_bar_background: Some(rgba(0xf0f0f0ff).into()),
                        title_bar_background: Some(rgba(0xf0f0f0ff).into()),
                        toolbar_background: Some(rgba(0xf0f0f0ff).into()),
                        tab_bar_background: Some(rgba(0xf0f0f0ff).into()),
                        tab_inactive_background: Some(rgba(0xf0f0f0ff).into()),
                        tab_active_background: Some(rgba(0xf6f6f6ff).into()),
                        editor_background: Some(rgba(0xfbfbfbff).into()),
                        editor_gutter_background: Some(rgba(0xfbfbfbff).into()),
                        editor_line_number: Some(rgba(0x90a7b2ff).into()),
                        editor_active_line_number: Some(rgba(0x403f53ff).into()),
                        terminal_background: Some(rgba(0xf6f6f6ff).into()),
                        terminal_ansi_bright_black: Some(rgba(0x403f53ff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xde3d3bff).into()),
                        terminal_ansi_bright_green: Some(rgba(0x08916aff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0xdaaa01ff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x288ed7ff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0xd6438aff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0x2aa298ff).into()),
                        terminal_ansi_bright_white: Some(rgba(0xf0f0f0ff).into()),
                        terminal_ansi_black: Some(rgba(0x403f53ff).into()),
                        terminal_ansi_red: Some(rgba(0xde3d3bff).into()),
                        terminal_ansi_green: Some(rgba(0x08916aff).into()),
                        terminal_ansi_yellow: Some(rgba(0xe0af02ff).into()),
                        terminal_ansi_blue: Some(rgba(0x288ed7ff).into()),
                        terminal_ansi_magenta: Some(rgba(0xd6438aff).into()),
                        terminal_ansi_cyan: Some(rgba(0x2aa298ff).into()),
                        terminal_ansi_white: Some(rgba(0xf0f0f0ff).into()),
                        ..Default::default()
                    },
                    status: StatusColorsRefinement {
                        deleted: Some(rgba(0x403f53ff).into()),
                        error: Some(rgba(0x403f53ff).into()),
                        hidden: Some(rgba(0x403f53ff).into()),
                        hint: Some(rgba(0x969696ff).into()),
                        warning: Some(rgba(0xdaaa01ff).into()),
                        ..Default::default()
                    },
                    syntax: Some(UserSyntaxTheme {
                        highlights: vec![
                            (
                                "attribute".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x4876d6ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "boolean".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x4876d6ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x989fb1ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment.doc".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x989fb1ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constant".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x4876d6ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "function".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x4876d6ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "keyword".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x994cc3ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "number".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xaa0982ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "operator".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x0c969bff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "property".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x0c969bff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x994cc3ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.bracket".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x994cc3ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.delimiter".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x994cc3ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.list_marker".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x994cc3ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x994cc3ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x4876d6ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.escape".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x4876d6ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "tag".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x994cc3ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "text.literal".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x4876d6ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "type".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x4876d6ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x403f53ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x0c969bff).into()),
                                    ..Default::default()
                                },
                            ),
                        ],
                    }),
                },
            },
        ],
    }
}
