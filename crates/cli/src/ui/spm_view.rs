use crate::domain::builder::spm_builder::*;
use iced::widget::{center_x, checkbox, column, text};
use iced::{Color, Element};

pub type AppElement<'a> = Element<'a, Message>;

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
	InputChanged(String),
	GenerateSPM,
}

#[derive(Default)]
pub struct SpmView {
	changelog: bool,
	readme: bool,
	swift_package_index: bool,
	swiftlint: bool,
	ios: bool,
	macos: bool,
	tvos: bool,
	watchos: bool,
	visionos: bool,
	input_content: String,
	platform_error: bool,
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

			Message::IosToggled(val) => {
				if val {
					self.ios = true;
					self.macos = false;
					self.tvos = false;
					self.watchos = false;
					self.visionos = false;
					self.platform_error = false;
				} else {
					self.ios = false;
				}
			}
			Message::MacOsToggled(val) => {
				if val {
					self.ios = false;
					self.macos = true;
					self.tvos = false;
					self.watchos = false;
					self.visionos = false;
					self.platform_error = false;
				} else {
					self.macos = false;
				}
			}
			Message::TvOsToggled(val) => {
				if val {
					self.ios = false;
					self.macos = false;
					self.tvos = true;
					self.watchos = false;
					self.visionos = false;
					self.platform_error = false;
				} else {
					self.tvos = false;
				}
			}
			Message::WatchOsToggled(val) => {
				if val {
					self.ios = false;
					self.macos = false;
					self.tvos = false;
					self.watchos = true;
					self.visionos = false;
					self.platform_error = false;
				} else {
					self.watchos = false;
				}
			}
			Message::VisionOsToggled(val) => {
				if val {
					self.ios = false;
					self.macos = false;
					self.tvos = false;
					self.watchos = false;
					self.visionos = true;
					self.platform_error = false;
				} else {
					self.visionos = false;
				}
			}
			Message::InputChanged(s) => self.input_content = s,
			Message::GenerateSPM => {
				let mut platforms = Vec::new();
				if self.ios {
					platforms.push("iOS");
				}
				if self.macos {
					platforms.push("macOS");
				}
				if self.tvos {
					platforms.push("tvOS");
				}
				if self.watchos {
					platforms.push("watchOS");
				}
				if self.visionos {
					platforms.push("visionOS");
				}

				if platforms.is_empty() {
					self.platform_error = true;
					return;
				} else {
					self.platform_error = false;
				}

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
					if (SpmBuilder::builder(&project_name, files, platforms).await).is_ok() {
						let command = format!("cd {} && open Package.swift", project_name);
						let _ = std::process::Command::new("sh")
							.arg("-c")
							.arg(&command)
							.spawn();
					}
				});
			}
		}
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
			self.generate_button_view(),
		]
		.spacing(16)
		.into()
	}

	/// Returns the header/title section view
	fn header_view(&self) -> AppElement<'_> {
		column![
			center_x(text("SPM Swift Package").color(Color::WHITE).size(32)).height(20),
			center_x(
				text(
					"Command Line Tools for macOS to create Swift Package Manager projects with desirable files."
				)
				.color(Color::WHITE)
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
			self.checkbox_row("iOS", self.ios, Message::IosToggled),
			self.checkbox_row("macOS", self.macos, Message::MacOsToggled),
			self.checkbox_row("tvOS", self.tvos, Message::TvOsToggled),
			self.checkbox_row("watchOS", self.watchos, Message::WatchOsToggled),
			self.checkbox_row("visionOS", self.visionos, Message::VisionOsToggled),
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
