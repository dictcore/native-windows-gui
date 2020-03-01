/**
    Error enums used in the native window gui crate
*/
#[derive(Debug, Clone)]
pub enum NwgError {
    Unknown,
    
    /// Fatal error that may happen when calling low level winapi functionalities
    InitializationError(String),

    /// Error that may happen when creating a control.
    ControlCreationError(String),

    /// Error that may happen when creating a menu.
    MenuCreationError(String),

    /// Error that may happen when creating a resource.
    ResourceCreationError(String),

    /// Error raised by the FileDialog object
    #[cfg(feature = "file-dialog")]
    FileDialogError(String),

    /// Error raised by the ImageDecoder feature
    #[cfg(feature = "image-decoder")]
    ImageDecoderError(i32, String),

    /// Error raised by on of the locale functions
    #[cfg(feature = "winnls")]
    BadLocale(String),
}

impl NwgError {

    pub fn initialization<S: Into<String>>(e: S) -> NwgError {
        NwgError::InitializationError(e.into())
    }

    pub fn control_create<S: Into<String>>(e: S) -> NwgError {
        NwgError::ControlCreationError(e.into())
    }

    pub fn menu_create<S: Into<String>>(e: S) -> NwgError {
        NwgError::MenuCreationError(e.into())
    }

    pub fn resource_create<S: Into<String>>(e: S) -> NwgError {
        NwgError::ResourceCreationError(e.into())
    }

    #[cfg(feature = "file-dialog")]
    pub fn file_dialog<S: Into<String>>(e: S) -> NwgError {
        NwgError::FileDialogError(e.into())
    }

    #[cfg(feature = "winnls")]
    pub fn bad_locale<S: Into<String>>(e: S) -> NwgError {
        NwgError::BadLocale(e.into())
    }

    #[cfg(feature = "image-decoder")]
    pub fn image_decoder<S: Into<String>>(code: i32, e: S) -> NwgError {
        NwgError::ImageDecoderError(code, e.into())
    }

    pub fn no_parent(name: &'static str) -> NwgError {
        NwgError::ControlCreationError(format!("No parent defined for {:?} control", name))
    }

    pub fn no_parent_menu() -> NwgError {
        NwgError::MenuCreationError("No parent defined for menu".to_string())
    }

}