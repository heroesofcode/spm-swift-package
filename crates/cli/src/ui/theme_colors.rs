use colored::Color as TermColor;
use iced::Color as IcedColor;

/// Centralized color palette used for both Iced UI and colored terminal output
pub struct ThemeColors;

impl ThemeColors {
	// ICED COLORS

	/// Hex: `#F05138`
	pub const ORANGE: IcedColor = IcedColor::from_rgb8(240, 81, 56);

	/// Hex: `#808080`
	pub const GRAY: IcedColor = IcedColor::from_rgb8(128, 128, 128);

	// TERMINAL COLORS

	/// Terminal equivalent of `ORANGE`
	pub const ORANGE_TERM: TermColor = TermColor::TrueColor {
		r: 240,
		g: 81,
		b: 56,
	};
}
