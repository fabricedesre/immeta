use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Seek, SeekFrom};
use std::path::Path;
use std::result;

use crate::formats::{gif, jpeg, png, webp};
use crate::generic::markers::MetadataMarker;
use crate::traits::LoadableMetadata;
use crate::types::{Dimensions, Result};

/// Contains metadata marker types.
///
/// Metadata markers is a convenient way to access metadata loading functions for particular
/// image types. They are also integrated with `GenericMetadata`, providing a convenient
/// syntax to downcast a `GenericMetadata` value to a specific metadata type.
///
/// Metadata marker types can be used directly, for example:
/// ```ignore
/// use immeta::markers::Jpeg;
///
/// let metadata = Jpeg::load_from_file("kitty.jpg").unwrap();
/// ```
///
/// They can also be used together with `GenericMetadata`:
/// ```ignore
/// use immeta::markers::Jpeg;
///
/// let gmd = immeta::load_from_file("kitty.jpg").unwrap();
/// let jpeg_metadata: Jpeg::Metadata = gmd.into::<Jpeg>().unwrap();
/// ```
///
/// Alternatively, you can use `as_ref()`:
/// ```ignore
/// let jpeg_metadata: &Jpeg::Metadata = gmd.as_ref::<Jpeg>().unwrap();
/// ```
///
/// `MetadataMarker::Metadata` associated type always points to concrete metadata type
/// from one of `immeta::formats` submodules.
pub mod markers {
    use std::io::{BufRead, Seek};
    use std::path::Path;
    use std::result;

    use crate::formats::{gif, jpeg, png, webp};
    use crate::generic::GenericMetadata;
    use crate::types::Result;

    /// A marker trait for a specific metadata type.
    pub trait MetadataMarker {
        type Metadata;

        /// Tries to convert the given `GenericMetadata` instance into a concrete metadata type.
        ///
        /// If the generic value really contains the associated metadata type, then `Ok` variant
        /// is returned; otherwise `Err` variant containing the original value is returned.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use immeta::markers::Jpeg;
        /// use immeta::formats::jpeg;
        /// use immeta::GenericMetadata;
        ///
        /// let generic = immeta::load_from_file("kitty.jpg").unwrap();
        /// let concrete: Result<jpeg::Metadata, GenericMetadata> = generic.into::<Jpeg>();
        /// assert!(concrete.is_ok());
        /// ```
        ///
        /// ```no_run
        /// use immeta::markers::Jpeg;
        /// use immeta::formats::jpeg;
        /// use immeta::GenericMetadata;
        ///
        /// let generic = immeta::load_from_file("kitty.png").unwrap();
        /// let concrete: Result<jpeg::Metadata, GenericMetadata> = generic.into::<Jpeg>();
        /// assert!(concrete.is_err());
        /// ```
        fn from_generic(gmd: GenericMetadata) -> result::Result<Self::Metadata, GenericMetadata>;

        /// Tries to extract a reference to a concrete metadata type from the given
        /// `GenericMetadata` reference.
        ///
        /// Behaves similarly to `from_generic()`, except using references instead of immediate
        /// values.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use immeta::markers::Jpeg;
        /// use immeta::formats::jpeg;
        ///
        /// let generic = immeta::load_from_file("kitty.jpg").unwrap();
        /// let concrete: Option<&jpeg::Metadata> = generic.as_ref::<Jpeg>();
        /// assert!(concrete.is_some());
        /// ```
        ///
        /// ```no_run
        /// use immeta::markers::Jpeg;
        /// use immeta::formats::jpeg;
        ///
        /// let generic = immeta::load_from_file("kitty.png").unwrap();
        /// let concrete: Option<&jpeg::Metadata> = generic.as_ref::<Jpeg>();
        /// assert!(concrete.is_none());
        /// ```
        fn from_generic_ref(gmd: &GenericMetadata) -> Option<&Self::Metadata>;

        /// Attempts to load metadata for an image of a concrete type from the provided reader.
        ///
        /// Invokes `LoadableMetadata::load()` for the associated metadata type. Use this
        /// method instead of calling `load()` on the metadata type directly.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use std::io::{self, BufReader};
        /// use immeta::markers::{MetadataMarker, Jpeg};
        ///
        /// let data = io::stdin();
        /// let metadata = Jpeg::load(&mut data.lock());
        /// ```
        fn load<R: ?Sized + BufRead>(r: &mut R) -> Result<Self::Metadata>;

        /// Attempts to load metadata for an image of a concrete type from the provided
        /// seekable reader.
        ///
        /// Invokes `LoadableMetadata::load_from_seek()` for the associated metadata type. Use
        /// this method instead of calling `load_from_seek()` on the metadata type directly.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use std::io::{Read, BufReader, Cursor};
        /// use immeta::markers::{MetadataMarker, Jpeg};
        ///
        /// # fn obtain_image() -> Vec<u8> { unimplemented!() }
        ///
        /// let data: Vec<u8> = obtain_image();
        /// let metadata = Jpeg::load(&mut BufReader::new(Cursor::new(data)));
        /// ```
        fn load_from_seek<R: ?Sized + BufRead + Seek>(r: &mut R) -> Result<Self::Metadata>;

        /// Attempts to load metadata for an image of a concrete type from a file identified
        /// by the provided path.
        ///
        /// Invokes `LoadableMetadata::load_from_file()` for the associated metadata type. Use this
        /// method instead of calling `load_from_file()` on the metadata type directly.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use immeta::markers::{MetadataMarker, Jpeg};
        ///
        /// let metadata = Jpeg::load_from_file("kitty.jpg");
        /// ```
        fn load_from_file<P: AsRef<Path>>(p: P) -> Result<Self::Metadata>;

        /// Attempts to load metadata for an image of a concrete type from the provided byte
        /// buffer.
        ///
        /// Invokes `LoadableMetadata::load_from_buf()` for the associated metadata type. Use this
        /// method instead of calling `load_from_buf()` on the metadata type directly.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use immeta::markers::{MetadataMarker, Jpeg};
        ///
        /// let buf: &[u8] = &[1, 2, 3, 4];   // pretend that this is an actual image
        /// let metadata = Jpeg::load_from_buf(buf);
        /// ```
        fn load_from_buf(b: &[u8]) -> Result<Self::Metadata>;
    }

    macro_rules! impl_metadata_marker {
        ($name:ident, $gvar:ident, $mtpe:ty) => {
            pub enum $name {}

            impl MetadataMarker for $name {
                type Metadata = $mtpe;

                #[inline]
                fn from_generic(gmd: GenericMetadata) -> result::Result<$mtpe, GenericMetadata> {
                    match gmd {
                        $crate::generic::GenericMetadata::$gvar(md) => Ok(md),
                        gmd => Err(gmd),
                    }
                }

                #[inline]
                fn from_generic_ref(gmd: &GenericMetadata) -> Option<&$mtpe> {
                    match *gmd {
                        $crate::generic::GenericMetadata::$gvar(ref md) => Some(md),
                        _ => None,
                    }
                }

                #[inline]
                fn load<R: ?Sized + BufRead>(r: &mut R) -> Result<$mtpe> {
                    $crate::traits::LoadableMetadata::load(r)
                }

                #[inline]
                fn load_from_seek<R: ?Sized + BufRead + Seek>(r: &mut R) -> Result<$mtpe> {
                    $crate::traits::LoadableMetadata::load_from_seek(r)
                }

                #[inline]
                fn load_from_file<P: AsRef<Path>>(p: P) -> Result<$mtpe> {
                    $crate::traits::LoadableMetadata::load_from_file(p)
                }

                #[inline]
                fn load_from_buf(b: &[u8]) -> Result<$mtpe> {
                    $crate::traits::LoadableMetadata::load_from_buf(b)
                }
            }
        };
    }

    impl_metadata_marker! { Jpeg, Jpeg, jpeg::Metadata }
    impl_metadata_marker! { Png, Png, png::Metadata }
    impl_metadata_marker! { Gif, Gif, gif::Metadata }
    impl_metadata_marker! { Webp, Webp, webp::Metadata }
}

/// Represents metadata loaded from a file whose format was determined automatically.
///
/// Values of this type are obtained via `immeta::load()` function and its derivatives.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GenericMetadata {
    Png(png::Metadata),
    Gif(gif::Metadata),
    Jpeg(jpeg::Metadata),
    Webp(webp::Metadata),
}

impl GenericMetadata {
    /// Returns image dimensions from the contained metadata.
    pub fn dimensions(&self) -> Dimensions {
        match *self {
            GenericMetadata::Png(ref md) => md.dimensions,
            GenericMetadata::Gif(ref md) => md.dimensions,
            GenericMetadata::Jpeg(ref md) => md.dimensions,
            GenericMetadata::Webp(ref md) => md.dimensions(),
        }
    }

    /// Returns a MIME type string for the image type of the contained metadata.
    pub fn mime_type(&self) -> &'static str {
        match *self {
            GenericMetadata::Png(_) => "image/png",
            GenericMetadata::Gif(_) => "image/gif",
            GenericMetadata::Jpeg(_) => "image/jpeg",
            GenericMetadata::Webp(_) => "image/webp",
        }
    }

    /// Attemts to convert this value to the specific metadata type by value.
    ///
    /// This method is needed only to provide a convenient syntax and it is not necessary
    /// because one may just `match` on the `GenericMetadata` value.
    #[inline]
    pub fn into<T: MetadataMarker>(self) -> result::Result<T::Metadata, GenericMetadata> {
        <T as MetadataMarker>::from_generic(self)
    }

    /// Attempts to convert this value to the sepcific metadata type by reference.
    ///
    /// This method is needed only to provide a convenient syntax and it is not necessary
    /// because one may just `match` on the `GenericMetadata` value.
    #[inline]
    pub fn as_ref<T: MetadataMarker>(&self) -> Option<&T::Metadata> {
        <T as MetadataMarker>::from_generic_ref(self)
    }
}

/// Attempts to load metadata for an image contained in the provided input stream.
///
/// This method automatically determines the format of the contained image. Because it may
/// need to read the stream from the beginning several times, a `Seek` bound is necessary
/// on the input stream. This may cause problems only with network streams as they are
/// naturally not seekable, so one would need to buffer the data from them first.
pub fn load<R: ?Sized + BufRead + Seek>(r: &mut R) -> Result<GenericMetadata> {
    // try png
    r.seek(SeekFrom::Start(0))?;
    if let Ok(md) = png::Metadata::load_from_seek(r) {
        return Ok(GenericMetadata::Png(md));
    }

    // try gif
    r.seek(SeekFrom::Start(0))?;
    if let Ok(md) = gif::Metadata::load_from_seek(r) {
        return Ok(GenericMetadata::Gif(md));
    }

    // try webp
    r.seek(SeekFrom::Start(0))?;
    if let Ok(md) = webp::Metadata::load_from_seek(r) {
        return Ok(GenericMetadata::Webp(md));
    }

    // try jpeg
    r.seek(SeekFrom::Start(0))?;
    if let Ok(md) = jpeg::Metadata::load_from_seek(r) {
        return Ok(GenericMetadata::Jpeg(md));
    }

    Err(invalid_format!("unknown or unsupported image type"))
}

/// Attempts to load metadata for an image contained in a file identified by the provided path.
///
/// This method delegates to `load()` method and, consequently, also determines the image format
/// automatically.
pub fn load_from_file<P: AsRef<Path>>(p: P) -> Result<GenericMetadata> {
    let mut f = BufReader::new(File::open(p)?);
    load(&mut f)
}

/// Attempts to load metadata for an image contained in an in-memory buffer.
///
/// This method delegates to `load()` method and, consequently, also determines the image format
/// automatically.
pub fn load_from_buf(b: &[u8]) -> Result<GenericMetadata> {
    load(&mut Cursor::new(b))
}
