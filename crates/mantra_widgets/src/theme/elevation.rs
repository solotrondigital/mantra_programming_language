//! Elevation levels and box shadow tokens.

/// Elevation depth levels corresponding to Material & modern flat UI design standards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Elevation {
    None,
    Low,
    Medium,
    High,
    Highest,
    Modal,
}

impl Elevation {
    pub fn to_box_shadow(&self, is_dark_mode: bool) -> &'static str {
        if is_dark_mode {
            match self {
                Elevation::None => "none",
                Elevation::Low => "0 1px 3px 0 rgba(0, 0, 0, 0.4), 0 1px 2px -1px rgba(0, 0, 0, 0.4)",
                Elevation::Medium => "0 4px 6px -1px rgba(0, 0, 0, 0.4), 0 2px 4px -2px rgba(0, 0, 0, 0.4)",
                Elevation::High => "0 10px 15px -3px rgba(0, 0, 0, 0.4), 0 4px 6px -4px rgba(0, 0, 0, 0.4)",
                Elevation::Highest => "0 20px 25px -5px rgba(0, 0, 0, 0.5), 0 8px 10px -6px rgba(0, 0, 0, 0.5)",
                Elevation::Modal => "0 25px 50px -12px rgba(0, 0, 0, 0.7)",
            }
        } else {
            match self {
                Elevation::None => "none",
                Elevation::Low => "0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px -1px rgba(0, 0, 0, 0.1)",
                Elevation::Medium => "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -2px rgba(0, 0, 0, 0.1)",
                Elevation::High => "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -4px rgba(0, 0, 0, 0.1)",
                Elevation::Highest => "0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 8px 10px -6px rgba(0, 0, 0, 0.1)",
                Elevation::Modal => "0 25px 50px -12px rgba(0, 0, 0, 0.25)",
            }
        }
    }
}

