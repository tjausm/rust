#![feature(path_try_exists)]

use fluent_bundle::FluentResource;
use fluent_syntax::parser::ParserError;
use rustc_data_structures::sync::Lrc;
use rustc_macros::{Decodable, Encodable};
use rustc_span::Span;
use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use tracing::{instrument, trace};

pub use fluent_bundle::{FluentArgs, FluentError, FluentValue};
pub use unic_langid::{langid, LanguageIdentifier};

static FALLBACK_FLUENT_RESOURCE: &'static str = include_str!("../locales/en-US/diagnostics.ftl");

pub type FluentBundle = fluent_bundle::FluentBundle<FluentResource>;

#[derive(Debug)]
pub enum TranslationBundleError {
    /// Failed to read from `.ftl` file.
    ReadFtl(io::Error),
    /// Failed to parse contents of `.ftl` file.
    ParseFtl(ParserError),
    /// Failed to add `FluentResource` to `FluentBundle`.
    AddResource(FluentError),
    /// `$sysroot/share/locale/$locale` does not exist.
    MissingLocale(io::Error),
    /// Cannot read directory entries of `$sysroot/share/locale/$locale`.
    ReadLocalesDir(io::Error),
    /// Cannot read directory entry of `$sysroot/share/locale/$locale`.
    ReadLocalesDirEntry(io::Error),
    /// `$sysroot/share/locale/$locale` is not a directory.
    LocaleIsNotDir,
}

impl fmt::Display for TranslationBundleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TranslationBundleError::ReadFtl(e) => write!(f, "could not read ftl file: {}", e),
            TranslationBundleError::ParseFtl(e) => {
                write!(f, "could not parse ftl file: {}", e)
            }
            TranslationBundleError::AddResource(e) => write!(f, "failed to add resource: {}", e),
            TranslationBundleError::MissingLocale(e) => {
                write!(f, "missing locale directory: {}", e)
            }
            TranslationBundleError::ReadLocalesDir(e) => {
                write!(f, "could not read locales dir: {}", e)
            }
            TranslationBundleError::ReadLocalesDirEntry(e) => {
                write!(f, "could not read locales dir entry: {}", e)
            }
            TranslationBundleError::LocaleIsNotDir => {
                write!(f, "`$sysroot/share/locales/$locale` is not a directory")
            }
        }
    }
}

impl Error for TranslationBundleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TranslationBundleError::ReadFtl(e) => Some(e),
            TranslationBundleError::ParseFtl(e) => Some(e),
            TranslationBundleError::AddResource(e) => Some(e),
            TranslationBundleError::MissingLocale(e) => Some(e),
            TranslationBundleError::ReadLocalesDir(e) => Some(e),
            TranslationBundleError::ReadLocalesDirEntry(e) => Some(e),
            TranslationBundleError::LocaleIsNotDir => None,
        }
    }
}

impl From<(FluentResource, Vec<ParserError>)> for TranslationBundleError {
    fn from((_, mut errs): (FluentResource, Vec<ParserError>)) -> Self {
        TranslationBundleError::ParseFtl(errs.pop().expect("failed ftl parse with no errors"))
    }
}

impl From<Vec<FluentError>> for TranslationBundleError {
    fn from(mut errs: Vec<FluentError>) -> Self {
        TranslationBundleError::AddResource(
            errs.pop().expect("failed adding resource to bundle with no errors"),
        )
    }
}

/// Returns Fluent bundle with the user's locale resources from
/// `$sysroot/share/locale/$requested_locale/*.ftl`.
///
/// If `-Z additional-ftl-path` was provided, load that resource and add it  to the bundle
/// (overriding any conflicting messages).
#[instrument(level = "trace")]
pub fn fluent_bundle(
    sysroot: &Path,
    requested_locale: Option<LanguageIdentifier>,
    additional_ftl_path: Option<&Path>,
) -> Result<Option<Lrc<FluentBundle>>, TranslationBundleError> {
    if requested_locale.is_none() && additional_ftl_path.is_none() {
        return Ok(None);
    }

    // If there is only `-Z additional-ftl-path`, assume locale is "en-US", otherwise use user
    // provided locale.
    let locale = requested_locale.clone().unwrap_or_else(|| langid!("en-US"));
    trace!(?locale);
    let mut bundle = FluentBundle::new(vec![locale]);

    // Fluent diagnostics can insert directionality isolation markers around interpolated variables
    // indicating that there may be a shift from right-to-left to left-to-right text (or
    // vice-versa). These are disabled because they are sometimes visible in the error output, but
    // may be worth investigating in future (for example: if type names are left-to-right and the
    // surrounding diagnostic messages are right-to-left, then these might be helpful).
    bundle.set_use_isolating(false);

    if let Some(requested_locale) = requested_locale {
        let mut sysroot = sysroot.to_path_buf();
        sysroot.push("share");
        sysroot.push("locale");
        sysroot.push(requested_locale.to_string());
        trace!(?sysroot);

        let _ = sysroot.try_exists().map_err(TranslationBundleError::MissingLocale)?;

        if !sysroot.is_dir() {
            return Err(TranslationBundleError::LocaleIsNotDir);
        }

        for entry in sysroot.read_dir().map_err(TranslationBundleError::ReadLocalesDir)? {
            let entry = entry.map_err(TranslationBundleError::ReadLocalesDirEntry)?;
            let path = entry.path();
            trace!(?path);
            if path.extension().and_then(|s| s.to_str()) != Some("ftl") {
                trace!("skipping");
                continue;
            }

            let resource_str = fs::read_to_string(path).map_err(TranslationBundleError::ReadFtl)?;
            let resource =
                FluentResource::try_new(resource_str).map_err(TranslationBundleError::from)?;
            trace!(?resource);
            bundle.add_resource(resource).map_err(TranslationBundleError::from)?;
        }
    }

    if let Some(additional_ftl_path) = additional_ftl_path {
        let resource_str =
            fs::read_to_string(additional_ftl_path).map_err(TranslationBundleError::ReadFtl)?;
        let resource =
            FluentResource::try_new(resource_str).map_err(TranslationBundleError::from)?;
        trace!(?resource);
        bundle.add_resource_overriding(resource);
    }

    let bundle = Lrc::new(bundle);
    Ok(Some(bundle))
}

/// Return the default `FluentBundle` with standard "en-US" diagnostic messages.
#[instrument(level = "trace")]
pub fn fallback_fluent_bundle() -> Result<Lrc<FluentBundle>, TranslationBundleError> {
    let fallback_resource = FluentResource::try_new(FALLBACK_FLUENT_RESOURCE.to_string())
        .map_err(TranslationBundleError::from)?;
    trace!(?fallback_resource);
    let mut fallback_bundle = FluentBundle::new(vec![langid!("en-US")]);
    // See comment in `fluent_bundle`.
    fallback_bundle.set_use_isolating(false);
    fallback_bundle.add_resource(fallback_resource).map_err(TranslationBundleError::from)?;
    let fallback_bundle = Lrc::new(fallback_bundle);
    Ok(fallback_bundle)
}

/// Identifier for the Fluent message/attribute corresponding to a diagnostic message.
type FluentId = Cow<'static, str>;

/// Abstraction over a message in a diagnostic to support both translatable and non-translatable
/// diagnostic messages.
///
/// Intended to be removed once diagnostics are entirely translatable.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Encodable, Decodable)]
pub enum DiagnosticMessage {
    /// Non-translatable diagnostic message.
    // FIXME(davidtwco): can a `Cow<'static, str>` be used here?
    Str(String),
    /// Identifier for a Fluent message corresponding to the diagnostic message.
    FluentIdentifier(FluentId, Option<FluentId>),
}

impl DiagnosticMessage {
    /// Returns the `String` contained within the `DiagnosticMessage::Str` variant, assuming that
    /// this diagnostic message is of the legacy, non-translatable variety. Panics if this
    /// assumption does not hold.
    ///
    /// Don't use this - it exists to support some places that do comparison with diagnostic
    /// strings.
    pub fn expect_str(&self) -> &str {
        match self {
            DiagnosticMessage::Str(s) => s,
            _ => panic!("expected non-translatable diagnostic message"),
        }
    }

    /// Create a `DiagnosticMessage` for the provided Fluent identifier.
    pub fn fluent(id: impl Into<Cow<'static, str>>) -> Self {
        DiagnosticMessage::FluentIdentifier(id.into(), None)
    }

    /// Create a `DiagnosticMessage` for the provided Fluent identifier and attribute.
    pub fn fluent_attr(
        id: impl Into<Cow<'static, str>>,
        attr: impl Into<Cow<'static, str>>,
    ) -> Self {
        DiagnosticMessage::FluentIdentifier(id.into(), Some(attr.into()))
    }
}

/// `From` impl that enables existing diagnostic calls to functions which now take
/// `impl Into<DiagnosticMessage>` to continue to work as before.
impl<S: Into<String>> From<S> for DiagnosticMessage {
    fn from(s: S) -> Self {
        DiagnosticMessage::Str(s.into())
    }
}

/// A span together with some additional data.
#[derive(Clone, Debug)]
pub struct SpanLabel {
    /// The span we are going to include in the final snippet.
    pub span: Span,

    /// Is this a primary span? This is the "locus" of the message,
    /// and is indicated with a `^^^^` underline, versus `----`.
    pub is_primary: bool,

    /// What label should we attach to this span (if any)?
    pub label: Option<DiagnosticMessage>,
}

/// A collection of `Span`s.
///
/// Spans have two orthogonal attributes:
///
/// - They can be *primary spans*. In this case they are the locus of
///   the error, and would be rendered with `^^^`.
/// - They can have a *label*. In this case, the label is written next
///   to the mark in the snippet when we render.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Encodable, Decodable)]
pub struct MultiSpan {
    primary_spans: Vec<Span>,
    span_labels: Vec<(Span, DiagnosticMessage)>,
}

impl MultiSpan {
    #[inline]
    pub fn new() -> MultiSpan {
        MultiSpan { primary_spans: vec![], span_labels: vec![] }
    }

    pub fn from_span(primary_span: Span) -> MultiSpan {
        MultiSpan { primary_spans: vec![primary_span], span_labels: vec![] }
    }

    pub fn from_spans(mut vec: Vec<Span>) -> MultiSpan {
        vec.sort();
        MultiSpan { primary_spans: vec, span_labels: vec![] }
    }

    pub fn push_span_label(&mut self, span: Span, label: impl Into<DiagnosticMessage>) {
        self.span_labels.push((span, label.into()));
    }

    /// Selects the first primary span (if any).
    pub fn primary_span(&self) -> Option<Span> {
        self.primary_spans.first().cloned()
    }

    /// Returns all primary spans.
    pub fn primary_spans(&self) -> &[Span] {
        &self.primary_spans
    }

    /// Returns `true` if any of the primary spans are displayable.
    pub fn has_primary_spans(&self) -> bool {
        self.primary_spans.iter().any(|sp| !sp.is_dummy())
    }

    /// Returns `true` if this contains only a dummy primary span with any hygienic context.
    pub fn is_dummy(&self) -> bool {
        let mut is_dummy = true;
        for span in &self.primary_spans {
            if !span.is_dummy() {
                is_dummy = false;
            }
        }
        is_dummy
    }

    /// Replaces all occurrences of one Span with another. Used to move `Span`s in areas that don't
    /// display well (like std macros). Returns whether replacements occurred.
    pub fn replace(&mut self, before: Span, after: Span) -> bool {
        let mut replacements_occurred = false;
        for primary_span in &mut self.primary_spans {
            if *primary_span == before {
                *primary_span = after;
                replacements_occurred = true;
            }
        }
        for span_label in &mut self.span_labels {
            if span_label.0 == before {
                span_label.0 = after;
                replacements_occurred = true;
            }
        }
        replacements_occurred
    }

    /// Returns the strings to highlight. We always ensure that there
    /// is an entry for each of the primary spans -- for each primary
    /// span `P`, if there is at least one label with span `P`, we return
    /// those labels (marked as primary). But otherwise we return
    /// `SpanLabel` instances with empty labels.
    pub fn span_labels(&self) -> Vec<SpanLabel> {
        let is_primary = |span| self.primary_spans.contains(&span);

        let mut span_labels = self
            .span_labels
            .iter()
            .map(|&(span, ref label)| SpanLabel {
                span,
                is_primary: is_primary(span),
                label: Some(label.clone()),
            })
            .collect::<Vec<_>>();

        for &span in &self.primary_spans {
            if !span_labels.iter().any(|sl| sl.span == span) {
                span_labels.push(SpanLabel { span, is_primary: true, label: None });
            }
        }

        span_labels
    }

    /// Returns `true` if any of the span labels is displayable.
    pub fn has_span_labels(&self) -> bool {
        self.span_labels.iter().any(|(sp, _)| !sp.is_dummy())
    }
}

impl From<Span> for MultiSpan {
    fn from(span: Span) -> MultiSpan {
        MultiSpan::from_span(span)
    }
}

impl From<Vec<Span>> for MultiSpan {
    fn from(spans: Vec<Span>) -> MultiSpan {
        MultiSpan::from_spans(spans)
    }
}
