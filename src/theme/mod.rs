//! Theme Module
//!
//! This module provides a theme system for styling applications, including a `Theme` struct,
//! utility functions for color manipulation, and a `ThemeProvider` component for Yew applications.

use csscolorparser::Color;
use yew::{
    function_component,
    html,
    Children,
    Html,
    Properties,
};

/// Creates dark and light variants of a given color
///
/// # Arguments
///
/// * `rgb` - A string representation of an RGB color (e.g., "rgb(255,0,0)")
///
/// # Returns
///
/// A tuple containing the original color, a darker variant, and a lighter variant
fn create_color_variants(rgb: &str) -> (String, String, String) {
    let color = Color::from_html(rgb).unwrap();
    let [r, g, b, a] = color.to_rgba8();

    let dark = Color::from_rgba8((r as f32 * 0.9) as u8, (g as f32 * 0.9) as u8, (b as f32 * 0.9) as u8, a);
    let light = Color::from_rgba8(
        (r as f32 * 1.1).min(255.0) as u8,
        (g as f32 * 1.1).min(255.0) as u8,
        (b as f32 * 1.1).min(255.0) as u8,
        a,
    );

    (rgb.to_string(), dark.to_rgb_string(), light.to_rgb_string())
}

/// Represents a complete theme with various color properties
#[derive(Clone, PartialEq)]
pub struct Theme {
    pub background_color: String,
    pub background_color_dark: String,
    pub background_color_light: String,
    pub module_color: String,
    pub module_color_dark: String,
    pub module_color_light: String,
    pub text_color_primary: String,
    pub text_color_secondary: String,
    pub primary_color: String,
    pub primary_color_dark: String,
    pub primary_color_light: String,
    pub secondary_color: String,
    pub secondary_color_dark: String,
    pub secondary_color_light: String,
    pub tertiary_color: String,
    pub tertiary_color_dark: String,
    pub tertiary_color_light: String,
    pub success_color: String,
    pub success_color_dark: String,
    pub success_color_light: String,
    pub warning_color: String,
    pub warning_color_dark: String,
    pub warning_color_light: String,
    pub error_color: String,
    pub error_color_dark: String,
    pub error_color_light: String,
    pub disabled_color: String,
    pub disabled_color_dark: String,
    pub disabled_color_light: String,
}

/// Provides a default theme with predefined colors
impl Default for Theme {
    fn default() -> Self {
        let (background_color, background_color_dark, background_color_light) = create_color_variants("rgb(37,46,66)");
        let (module_color, module_color_dark, module_color_light) = create_color_variants("rgb(53,65,90)");
        let (primary_color, primary_color_dark, primary_color_light) = create_color_variants("rgb(44,105,141)");
        let (secondary_color, secondary_color_dark, secondary_color_light) = create_color_variants("rgb(186,232,232)");
        let (tertiary_color, tertiary_color_dark, tertiary_color_light) = create_color_variants("rgb(227,246,245)");
        let (success_color, success_color_dark, success_color_light) = create_color_variants("rgb(46,204,113)");
        let (warning_color, warning_color_dark, warning_color_light) = create_color_variants("rgb(243,156,18)");
        let (error_color, error_color_dark, error_color_light) = create_color_variants("rgb(231,76,60)");
        let (disabled_color, disabled_color_dark, disabled_color_light) = create_color_variants("rgb(127,140,141)");

        Theme {
            background_color,
            background_color_dark,
            background_color_light,
            module_color,
            module_color_dark,
            module_color_light,
            text_color_primary: "rgb(255,255,255)".to_string(),
            text_color_secondary: "rgb(204,204,204)".to_string(),
            primary_color,
            primary_color_dark,
            primary_color_light,
            secondary_color,
            secondary_color_dark,
            secondary_color_light,
            tertiary_color,
            tertiary_color_dark,
            tertiary_color_light,
            success_color,
            success_color_dark,
            success_color_light,
            warning_color,
            warning_color_dark,
            warning_color_light,
            error_color,
            error_color_dark,
            error_color_light,
            disabled_color,
            disabled_color_dark,
            disabled_color_light,
        }
    }
}

impl Theme {
    /// Sets the background color and its variants
    pub fn set_background_color(mut self, color: &str) -> Self {
        let (background_color, background_color_dark, background_color_light) = create_color_variants(color);
        self.background_color = background_color;
        self.background_color_dark = background_color_dark;
        self.background_color_light = background_color_light;
        self
    }

    /// Sets the module color and its variants
    pub fn set_module_color(mut self, color: &str) -> Self {
        let (module_color, module_color_dark, module_color_light) = create_color_variants(color);
        self.module_color = module_color;
        self.module_color_dark = module_color_dark;
        self.module_color_light = module_color_light;
        self
    }

    /// Sets the primary text color
    pub fn set_text_color_primary(mut self, color: &str) -> Self {
        self.text_color_primary = color.to_string();
        self
    }

    /// Sets the secondary text color
    pub fn set_text_color_secondary(mut self, color: &str) -> Self {
        self.text_color_secondary = color.to_string();
        self
    }

    /// Sets the primary color and its variants
    pub fn set_primary_color(mut self, color: &str) -> Self {
        let (primary_color, primary_color_dark, primary_color_light) = create_color_variants(color);
        self.primary_color = primary_color;
        self.primary_color_dark = primary_color_dark;
        self.primary_color_light = primary_color_light;
        self
    }

    /// Sets the secondary color and its variants
    pub fn set_secondary_color(mut self, color: &str) -> Self {
        let (secondary_color, secondary_color_dark, secondary_color_light) = create_color_variants(color);
        self.secondary_color = secondary_color;
        self.secondary_color_dark = secondary_color_dark;
        self.secondary_color_light = secondary_color_light;
        self
    }

    /// Sets the tertiary color and its variants
    pub fn set_tertiary_color(mut self, color: &str) -> Self {
        let (tertiary_color, tertiary_color_dark, tertiary_color_light) = create_color_variants(color);
        self.tertiary_color = tertiary_color;
        self.tertiary_color_dark = tertiary_color_dark;
        self.tertiary_color_light = tertiary_color_light;
        self
    }

    /// Sets the success color and its variants
    pub fn set_success_color(mut self, color: &str) -> Self {
        let (success_color, success_color_dark, success_color_light) = create_color_variants(color);
        self.success_color = success_color;
        self.success_color_dark = success_color_dark;
        self.success_color_light = success_color_light;
        self
    }

    /// Sets the warning color and its variants
    pub fn set_warning_color(mut self, color: &str) -> Self {
        let (warning_color, warning_color_dark, warning_color_light) = create_color_variants(color);
        self.warning_color = warning_color;
        self.warning_color_dark = warning_color_dark;
        self.warning_color_light = warning_color_light;
        self
    }

    /// Sets the error color and its variants
    pub fn set_error_color(mut self, color: &str) -> Self {
        let (error_color, error_color_dark, error_color_light) = create_color_variants(color);
        self.error_color = error_color;
        self.error_color_dark = error_color_dark;
        self.error_color_light = error_color_light;
        self
    }

    /// Sets the disabled color and its variants
    pub fn set_disabled_color(mut self, color: &str) -> Self {
        let (disabled_color, disabled_color_dark, disabled_color_light) = create_color_variants(color);
        self.disabled_color = disabled_color;
        self.disabled_color_dark = disabled_color_dark;
        self.disabled_color_light = disabled_color_light;
        self
    }
}

/// Properties for the ThemeProvider component
#[derive(Properties, Clone, PartialEq)]
pub struct ThemeProps {
    /// The theme to be applied
    pub theme: Theme,
    /// Child components that will inherit the theme
    pub children: Children,
}

/// ThemeProvider Component
///
/// This component applies the provided theme to its children by injecting CSS variables.
///
/// # Example
///
/// ```
/// use your_crate::{Theme, ThemeProvider};
///
/// let custom_theme = Theme::default()
///     .set_background_color("rgb(0,0,0)")
///     .set_primary_color("rgb(255,0,0)");
///
/// html! {
///     <ThemeProvider theme={custom_theme}>
///         <YourApp />
///     </ThemeProvider>
/// }
/// ```
#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProps) -> Html {
    let theme = &props.theme;

    let style = format!(
        r#"
        :root {{
            --background-color: {};
            --background-color-dark: {};
            --background-color-light: {};
            --module-color: {};
            --module-color-dark: {};
            --module-color-light: {};
            --text-color-primary: {};
            --text-color-secondary: {};
            --color-primary: {};
            --color-primary-dark: {};
            --color-primary-light: {};
            --color-secondary: {};
            --color-secondary-dark: {};
            --color-secondary-light: {};
            --color-tertiery: {};
            --color-tertiery-dark: {};
            --color-tertiery-light: {};
            --color-success: {};
            --color-success-dark: {};
            --color-success-light: {};
            --color-warning: {};
            --color-warning-dark: {};
            --color-warning-light: {};
            --color-error: {};
            --color-error-dark: {};
            --color-error-light: {};
            --color-disabled: {};
            --color-disabled-dark: {};
            --color-disabled-light: {};
        }}
        "#,
        theme.background_color,
        theme.background_color_dark,
        theme.background_color_light,
        theme.module_color,
        theme.module_color_dark,
        theme.module_color_light,
        theme.text_color_primary,
        theme.text_color_secondary,
        theme.primary_color,
        theme.primary_color_dark,
        theme.primary_color_light,
        theme.secondary_color,
        theme.secondary_color_dark,
        theme.secondary_color_light,
        theme.tertiary_color,
        theme.tertiary_color_dark,
        theme.tertiary_color_light,
        theme.success_color,
        theme.success_color_dark,
        theme.success_color_light,
        theme.warning_color,
        theme.warning_color_dark,
        theme.warning_color_light,
        theme.error_color,
        theme.error_color_dark,
        theme.error_color_light,
        theme.disabled_color,
        theme.disabled_color_dark,
        theme.disabled_color_light,
    );

    html! {
    <>
        <style>{ style }</style>
        { props.children.clone() }
    </>
    }
}
