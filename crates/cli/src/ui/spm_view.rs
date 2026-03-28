use crate::core::spm_builder::*;
use crate::ui::theme_colors::*;
use crate::utils::xcode;
use iced::widget::{center_x, checkbox, column, text};
use iced::{Color, Element};

pub type AppElement<'a> = Element<'a, Message>;

#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
	Ios,
	MacOs,
	TvOs,
	WatchOs,
	VisionOs,
}

impl Platform {
	fn as_str(&self) -> &'static str {
		match self {
			Platform::Ios => "iOS",
			Platform::MacOs => "macOS",
			Platform::TvOs => "tvOS",
			Platform::WatchOs => "watchOS",
			Platform::VisionOs => "visionOS",
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestFramework {
	XCTest,
	SwiftTesting,
}

impl TestFramework {
	fn as_str(&self) -> &'static str {
		match self {
			TestFramework::XCTest => "XCTest",
			TestFramework::SwiftTesting => "Swift Testing",
		}
	}
}

#[derive(Debug, Clone)]
pub enum Message {
	ChangelogToggled(bool),
	ReadmeToggled(bool),
	SwiftPackageIndexToggled(bool),
	SwiftLintToggled(bool),
	IosToggled(bool),
	MacOsToggled(bool),
	TvOsToggled(bool),
	WatchOsToggled(bool),
	VisionOsToggled(bool),
	XCTestToggled(bool),
	SwiftTestingToggled(bool),
	InputChanged(String),
	GenerateSPM,
}

#[derive(Default)]
pub struct SpmView {
	changelog: bool,
	readme: bool,
	swift_package_index: bool,
	swiftlint: bool,
	selected_platform: Option<Platform>,
	selected_framework: Option<TestFramework>,
	input_content: String,
	platform_error: bool,
	test_framework_error: bool,
}

/// Entry point to launch the iced application for the SPM GUI
pub fn run() -> iced::Result {
	iced::application(SpmView::default, SpmView::update, SpmView::view).run()
}

impl SpmView {
	/// Handles state updates for each message received from the UI
	pub fn update(&mut self, message: Message) {
		match message {
			Message::ChangelogToggled(val) => self.changelog = val,
			Message::ReadmeToggled(val) => self.readme = val,
			Message::SwiftPackageIndexToggled(val) => self.swift_package_index = val,
			Message::SwiftLintToggled(val) => self.swiftlint = val,

			Message::IosToggled(val) => self.select_platform(val, Platform::Ios),
			Message::MacOsToggled(val) => self.select_platform(val, Platform::MacOs),
			Message::TvOsToggled(val) => self.select_platform(val, Platform::TvOs),
			Message::WatchOsToggled(val) => self.select_platform(val, Platform::WatchOs),
			Message::VisionOsToggled(val) => self.select_platform(val, Platform::VisionOs),

			Message::XCTestToggled(val) => self.select_framework(val, TestFramework::XCTest),
			Message::SwiftTestingToggled(val) => self.select_framework(val, TestFramework::SwiftTesting),

			Message::InputChanged(s) => self.input_content = s,
			Message::GenerateSPM => self.generate(),
		}
	}

	/// Selects the given platform exclusively (radio-button behavior) or deselects if val is false
	fn select_platform(&mut self, val: bool, platform: Platform) {
		if val {
			self.selected_platform = Some(platform);
			self.platform_error = false;
		} else if self.selected_platform.as_ref() == Some(&platform) {
			self.selected_platform = None;
		}
	}

	/// Selects the given test framework exclusively (radio-button behavior) or deselects if val is false
	fn select_framework(&mut self, val: bool, framework: TestFramework) {
		if val {
			self.selected_framework = Some(framework);
			self.test_framework_error = false;
		} else if self.selected_framework.as_ref() == Some(&framework) {
			self.selected_framework = None;
		}
	}

	/// Validates selections, builds the project, and opens it in Xcode
	fn generate(&mut self) {
		let platform = match &self.selected_platform {
			Some(p) => p.as_str(),
			None => {
				self.platform_error = true;
				return;
			}
		};

		self.platform_error = false;

		let test_framework = match &self.selected_framework {
			Some(f) => f.as_str(),
			None => {
				self.test_framework_error = true;
				return;
			}
		};

		self.test_framework_error = false;

		let mut project_name = self.input_content.clone();

		if project_name.trim().is_empty() {
			project_name = "Library".to_string();
		}

		let mut files = Vec::new();

		if self.changelog {
			files.push("Changelog");
		}

		if self.readme {
			files.push("Readme");
		}

		if self.swift_package_index {
			files.push("Swift Package Index");
		}

		if self.swiftlint {
			files.push("SwiftLint");
		}

		tokio::spawn(async move {
			if SpmBuilder::create(&project_name, &files, &[platform], test_framework).is_ok() {
				let _ = xcode::open_xcode(&project_name);
			}
		});
	}

	/// Builds and returns the main application view layout
	pub fn view(&self) -> AppElement<'_> {
		column![
			self.header_view(),
			self.input_view(),
			self.files_title_view(),
			self.files_checkboxes_view(),
			self.platforms_title_view(),
			self.platforms_checkboxes_view(),
			self.error_platform_view(),
			self.test_framework_title_view(),
			self.test_framework_checkboxes_view(),
			self.error_test_framework_view(),
			self.generate_button_view(),
		]
		.spacing(16)
		.into()
	}

	/// Returns the header/title section view
	fn header_view(&self) -> AppElement<'_> {
		column![
			center_x(
				text("SPM Swift Package")
					.color(ThemeColors::ORANGE)
					.size(32)
			)
			.height(20),
			center_x(
				text(
					"Command Line Tools for macOS to create Swift Package Manager projects with desirable files."
				)
				.color(ThemeColors::GRAY)
				.size(14)
			)
			.padding(16),
		]
		.spacing(16)
		.into()
	}

	/// Returns the text input field view for the project name
	fn input_view(&self) -> AppElement<'_> {
		iced::widget::Container::new(
			iced::widget::text_input("Insert the name of the project", &self.input_content)
				.on_input(Message::InputChanged)
				.padding(8)
				.size(18),
		)
		.padding([0, 24])
		.into()
	}

	/// Returns the view for the file selection section's title
	fn files_title_view(&self) -> AppElement<'_> {
		let title = text("Choose the files you want to include in your project:")
			.color(Color::WHITE)
			.size(18);

		iced::widget::Container::new(title).padding([0, 24]).into()
	}

	/// Returns the view for the platform selection section's title
	fn platforms_title_view(&self) -> AppElement<'_> {
		let title = text("Choose the platforms you want to include in your project:")
			.color(Color::WHITE)
			.size(18);

		iced::widget::Container::new(title).padding([0, 24]).into()
	}

	/// Returns the view for the test framework selection section's title
	fn test_framework_title_view(&self) -> AppElement<'_> {
		let title = text("Choose the test framework:")
			.color(Color::WHITE)
			.size(18);

		iced::widget::Container::new(title).padding([0, 24]).into()
	}

	/// Returns the view containing all file checkboxes
	fn files_checkboxes_view(&self) -> AppElement<'_> {
		let mut col = column![];

		for checkbox in self.view_files_checkboxes() {
			col = col.push(checkbox);
		}

		col.spacing(8).into()
	}

	/// Returns the view containing all platform checkboxes
	fn platforms_checkboxes_view(&self) -> AppElement<'_> {
		let mut col = column![];

		for checkbox in self.view_platforms_checkboxes() {
			col = col.push(checkbox);
		}

		col.spacing(8).into()
	}

	/// Returns the view containing all test framework checkboxes
	fn test_framework_checkboxes_view(&self) -> AppElement<'_> {
		let mut col = column![];

		for checkbox in self.view_test_framework_checkboxes() {
			col = col.push(checkbox);
		}

		col.spacing(8).into()
	}

	/// Returns the view for the 'Generate SPM' button
	fn generate_button_view(&self) -> AppElement<'_> {
		iced::widget::Container::new(
			iced::widget::button(text("Generate SPM").size(16).color(Color::WHITE))
				.on_press(Message::GenerateSPM),
		)
		.padding([8, 24])
		.into()
	}

	/// Returns the view for the error message when no platform is selected
	fn error_platform_view(&self) -> AppElement<'_> {
		if self.platform_error {
			iced::widget::Container::new(
				text("Please select at least one platform.")
					.color([1.0, 0.2, 0.2])
					.size(16),
			)
			.padding([0, 24])
			.into()
		} else {
			iced::widget::Container::new("").into()
		}
	}

	/// Returns the view for the error message when no test framework is selected
	fn error_test_framework_view(&self) -> AppElement<'_> {
		if self.test_framework_error {
			iced::widget::Container::new(
				text("Please select a test framework.")
					.color([1.0, 0.2, 0.2])
					.size(16),
			)
			.padding([0, 24])
			.into()
		} else {
			iced::widget::Container::new("").into()
		}
	}

	/// Builds a vector of checkbox rows for each selectable file
	fn view_files_checkboxes(&self) -> Vec<AppElement<'_>> {
		vec![
			self.checkbox_row("Changelog", self.changelog, Message::ChangelogToggled),
			self.checkbox_row("Readme", self.readme, Message::ReadmeToggled),
			self.checkbox_row(
				"Swift Package Index",
				self.swift_package_index,
				Message::SwiftPackageIndexToggled,
			),
			self.checkbox_row("SwiftLint", self.swiftlint, Message::SwiftLintToggled),
		]
	}

	/// Builds a vector of checkbox rows for each selectable platform
	fn view_platforms_checkboxes(&self) -> Vec<AppElement<'_>> {
		vec![
			self.checkbox_row(
				"iOS",
				self.selected_platform == Some(Platform::Ios),
				Message::IosToggled,
			),
			self.checkbox_row(
				"macOS",
				self.selected_platform == Some(Platform::MacOs),
				Message::MacOsToggled,
			),
			self.checkbox_row(
				"tvOS",
				self.selected_platform == Some(Platform::TvOs),
				Message::TvOsToggled,
			),
			self.checkbox_row(
				"watchOS",
				self.selected_platform == Some(Platform::WatchOs),
				Message::WatchOsToggled,
			),
			self.checkbox_row(
				"visionOS",
				self.selected_platform == Some(Platform::VisionOs),
				Message::VisionOsToggled,
			),
		]
	}

	/// Builds a vector of checkbox rows for each test framework
	fn view_test_framework_checkboxes(&self) -> Vec<AppElement<'_>> {
		vec![
			self.checkbox_row(
				"XCTest",
				self.selected_framework == Some(TestFramework::XCTest),
				Message::XCTestToggled,
			),
			self.checkbox_row(
				"Swift Testing",
				self.selected_framework == Some(TestFramework::SwiftTesting),
				Message::SwiftTestingToggled,
			),
		]
	}

	/// Helper to create a row with a single checkbox and label
	fn checkbox_row<'a>(
		&self,
		label: &'a str,
		value: bool,
		on_toggle: fn(bool) -> Message,
	) -> AppElement<'a> {
		iced::widget::Container::new(checkbox(value).label(label).on_toggle(on_toggle))
			.padding([0, 24])
			.into()
	}
}
