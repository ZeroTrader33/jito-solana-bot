use crate::{OtelData, PreSampledTracer};
use opentelemetry::{
    trace::{self as otel, noop, TraceContextExt},
    Context as OtelContext, Key, KeyValue, Value,
};
use std::any::TypeId;
use std::fmt;
use std::marker;
use std::time::{Instant, SystemTime};
use tracing_core::span::{self, Attributes, Id, Record};
use tracing_core::{field, Event, Subscriber};
#[cfg(feature = "tracing-log")]
use tracing_log::NormalizeEvent;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

const SPAN_NAME_FIELD: &str = "otel.name";
const SPAN_KIND_FIELD: &str = "otel.kind";
const SPAN_STATUS_CODE_FIELD: &str = "otel.status_code";
const SPAN_STATUS_MESSAGE_FIELD: &str = "otel.status_message";

/// An [OpenTelemetry] propagation layer for use in a project that uses
/// [tracing].
///
/// [OpenTelemetry]: https://opentelemetry.io
/// [tracing]: https://github.com/tokio-rs/tracing
pub struct OpenTelemetryLayer<S, T> {
    tracer: T,
    event_location: bool,
    tracked_inactivity: bool,
    get_context: WithContext,
    _registry: marker::PhantomData<S>,
}

impl<S> Default for OpenTelemetryLayer<S, noop::NoopTracer>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    fn default() -> Self {
        OpenTelemetryLayer::new(noop::NoopTracer::new())
    }
}

/// Construct a layer to track spans via [OpenTelemetry].
///
/// [OpenTelemetry]: https://opentelemetry.io
///
/// # Examples
///
/// ```rust,no_run
/// use tracing_subscriber::layer::SubscriberExt;
/// use tracing_subscriber::Registry;
///
/// // Use the tracing subscriber `Registry`, or any other subscriber
/// // that impls `LookupSpan`
/// let subscriber = Registry::default().with(tracing_opentelemetry::layer());
/// # drop(subscriber);
/// ```
pub fn layer<S>() -> OpenTelemetryLayer<S, noop::NoopTracer>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    OpenTelemetryLayer::default()
}

// this function "remembers" the types of the subscriber so that we
// can downcast to something aware of them without knowing those
// types at the callsite.
//
// See https://github.com/tokio-rs/tracing/blob/4dad420ee1d4607bad79270c1520673fa6266a3d/tracing-error/src/layer.rs
pub(crate) struct WithContext(
    fn(&tracing::Dispatch, &span::Id, f: &mut dyn FnMut(&mut OtelData, &dyn PreSampledTracer)),
);

impl WithContext {
    // This function allows a function to be called in the context of the
    // "remembered" subscriber.
    pub(crate) fn with_context<'a>(
        &self,
        dispatch: &'a tracing::Dispatch,
        id: &span::Id,
        mut f: impl FnMut(&mut OtelData, &dyn PreSampledTracer),
    ) {
        (self.0)(dispatch, id, &mut f)
    }
}

fn str_to_span_kind(s: &str) -> Option<otel::SpanKind> {
    match s {
        s if s.eq_ignore_ascii_case("server") => Some(otel::SpanKind::Server),
        s if s.eq_ignore_ascii_case("client") => Some(otel::SpanKind::Client),
        s if s.eq_ignore_ascii_case("producer") => Some(otel::SpanKind::Producer),
        s if s.eq_ignore_ascii_case("consumer") => Some(otel::SpanKind::Consumer),
        s if s.eq_ignore_ascii_case("internal") => Some(otel::SpanKind::Internal),
        _ => None,
    }
}

fn str_to_status_code(s: &str) -> Option<otel::StatusCode> {
    match s {
        s if s.eq_ignore_ascii_case("unset") => Some(otel::StatusCode::Unset),
        s if s.eq_ignore_ascii_case("ok") => Some(otel::StatusCode::Ok),
        s if s.eq_ignore_ascii_case("error") => Some(otel::StatusCode::Error),
        _ => None,
    }
}

struct SpanEventVisitor<'a>(&'a mut otel::Event);

impl<'a> field::Visit for SpanEventVisitor<'a> {
    /// Record events on the underlying OpenTelemetry [`Span`] from `bool` values.
    ///
    /// [`Span`]: opentelemetry::trace::Span
    fn record_bool(&mut self, field: &field::Field, value: bool) {
        match field.name() {
            "message" => self.0.name = value.to_string().into(),
            // Skip fields that are actually log metadata that have already been handled
            #[cfg(feature = "tracing-log")]
            name if name.starts_with("log.") => (),
            name => {
                self.0.attributes.push(KeyValue::new(name, value));
            }
        }
    }

    /// Record events on the underlying OpenTelemetry [`Span`] from `f64` values.
    ///
    /// [`Span`]: opentelemetry::trace::Span
    fn record_f64(&mut self, field: &field::Field, value: f64) {
        match field.name() {
            "message" => self.0.name = value.to_string().into(),
            // Skip fields that are actually log metadata that have already been handled
            #[cfg(feature = "tracing-log")]
            name if name.starts_with("log.") => (),
            name => {
                self.0.attributes.push(KeyValue::new(name, value));
            }
        }
    }

    /// Record events on the underlying OpenTelemetry [`Span`] from `i64` values.
    ///
    /// [`Span`]: opentelemetry::trace::Span
    fn record_i64(&mut self, field: &field::Field, value: i64) {
        match field.name() {
            "message" => self.0.name = value.to_string().into(),
            // Skip fields that are actually log metadata that have already been handled
            #[cfg(feature = "tracing-log")]
            name if name.starts_with("log.") => (),
            name => {
                self.0.attributes.push(KeyValue::new(name, value));
            }
        }
    }

    /// Record events on the underlying OpenTelemetry [`Span`] from `&str` values.
    ///
    /// [`Span`]: opentelemetry::trace::Span
    fn record_str(&mut self, field: &field::Field, value: &str) {
        match field.name() {
            "message" => self.0.name = value.to_string().into(),
            // Skip fields that are actually log metadata that have already been handled
            #[cfg(feature = "tracing-log")]
            name if name.starts_with("log.") => (),
            name => {
                self.0
                    .attributes
                    .push(KeyValue::new(name, value.to_string()));
            }
        }
    }

    /// Record events on the underlying OpenTelemetry [`Span`] from values that
    /// implement Debug.
    ///
    /// [`Span`]: opentelemetry::trace::Span
    fn record_debug(&mut self, field: &field::Field, value: &dyn fmt::Debug) {
        match field.name() {
            "message" => self.0.name = format!("{:?}", value).into(),
            // Skip fields that are actually log metadata that have already been handled
            #[cfg(feature = "tracing-log")]
            name if name.starts_with("log.") => (),
            name => {
                self.0
                    .attributes
                    .push(KeyValue::new(name, format!("{:?}", value)));
            }
        }
    }
}

struct SpanAttributeVisitor<'a>(&'a mut otel::SpanBuilder);

impl<'a> SpanAttributeVisitor<'a> {
    fn record(&mut self, attribute: KeyValue) {
        debug_assert!(self.0.attributes.is_some());
        if let Some(v) = self.0.attributes.as_mut() {
            v.push(attribute);
        }
    }
}

impl<'a> field::Visit for SpanAttributeVisitor<'a> {
    /// Set attributes on the underlying OpenTelemetry [`Span`] from `bool` values.
    ///
    /// [`Span`]: opentelemetry::trace::Span
    fn record_bool(&mut self, field: &field::Field, value: bool) {
        self.record(KeyValue::new(field.name(), value));
    }

    /// Set attributes on the underlying OpenTelemetry [`Span`] from `f64` values.
    ///
    /// [`Span`]: opentelemetry::trace::Span
    fn record_f64(&mut self, field: &field::Field, value: f64) {
        self.record(KeyValue::new(field.name(), value));
    }

    /// Set attributes on the underlying OpenTelemetry [`Span`] from `i64` values.
    ///
    /// [`Span`]: opentelemetry::trace::Span
    fn record_i64(&mut self, field: &field::Field, value: i64) {
        self.record(KeyValue::new(field.name(), value));
    }

    /// Set attributes on the underlying OpenTelemetry [`Span`] from `&str` values.
    ///
    /// [`Span`]: opentelemetry::trace::Span
    fn record_str(&mut self, field: &field::Field, value: &str) {
        match field.name() {
            SPAN_NAME_FIELD => self.0.name = value.to_string().into(),
            SPAN_KIND_FIELD => self.0.span_kind = str_to_span_kind(value),
            SPAN_STATUS_CODE_FIELD => self.0.status_code = str_to_status_code(value),
            SPAN_STATUS_MESSAGE_FIELD => self.0.status_message = Some(value.to_owned().into()),
            _ => self.record(KeyValue::new(field.name(), value.to_string())),
        }
    }

    /// Set attributes on the underlying OpenTelemetry [`Span`] from values that
    /// implement Debug.
    ///
    /// [`Span`]: opentelemetry::trace::Span
    fn record_debug(&mut self, field: &field::Field, value: &dyn fmt::Debug) {
        match field.name() {
            SPAN_NAME_FIELD => self.0.name = format!("{:?}", value).into(),
            SPAN_KIND_FIELD => self.0.span_kind = str_to_span_kind(&format!("{:?}", value)),
            SPAN_STATUS_CODE_FIELD => {
                self.0.status_code = str_to_status_code(&format!("{:?}", value))
            }
            SPAN_STATUS_MESSAGE_FIELD => {
                self.0.status_message = Some(format!("{:?}", value).into())
            }
            _ => self.record(Key::new(field.name()).string(format!("{:?}", value))),
        }
    }
}

impl<S, T> OpenTelemetryLayer<S, T>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
    T: otel::Tracer + PreSampledTracer + 'static,
{
    /// Set the [`Tracer`] that this layer will use to produce and track
    /// OpenTelemetry [`Span`]s.
    ///
    /// [`Tracer`]: opentelemetry::trace::Tracer
    /// [`Span`]: opentelemetry::trace::Span
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tracing_opentelemetry::OpenTelemetryLayer;
    /// use tracing_subscriber::layer::SubscriberExt;
    /// use tracing_subscriber::Registry;
    ///
    /// // Create a jaeger exporter pipeline for a `trace_demo` service.
    /// let tracer = opentelemetry_jaeger::new_pipeline()
    ///     .with_service_name("trace_demo")
    ///     .install_simple()
    ///     .expect("Error initializing Jaeger exporter");
    ///
    /// // Create a layer with the configured tracer
    /// let otel_layer = OpenTelemetryLayer::new(tracer);
    ///
    /// // Use the tracing subscriber `Registry`, or any other subscriber
    /// // that impls `LookupSpan`
    /// let subscriber = Registry::default().with(otel_layer);
    /// # drop(subscriber);
    /// ```
    pub fn new(tracer: T) -> Self {
        OpenTelemetryLayer {
            tracer,
            event_location: true,
            tracked_inactivity: true,
            get_context: WithContext(Self::get_context),
            _registry: marker::PhantomData,
        }
    }

    /// Set the [`Tracer`] that this layer will use to produce and track
    /// OpenTelemetry [`Span`]s.
    ///
    /// [`Tracer`]: opentelemetry::trace::Tracer
    /// [`Span`]: opentelemetry::trace::Span
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tracing_subscriber::layer::SubscriberExt;
    /// use tracing_subscriber::Registry;
    ///
    /// // Create a jaeger exporter pipeline for a `trace_demo` service.
    /// let tracer = opentelemetry_jaeger::new_pipeline()
    ///     .with_service_name("trace_demo")
    ///     .install_simple()
    ///     .expect("Error initializing Jaeger exporter");
    ///
    /// // Create a layer with the configured tracer
    /// let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    ///
    /// // Use the tracing subscriber `Registry`, or any other subscriber
    /// // that impls `LookupSpan`
    /// let subscriber = Registry::default().with(otel_layer);
    /// # drop(subscriber);
    /// ```
    pub fn with_tracer<Tracer>(self, tracer: Tracer) -> OpenTelemetryLayer<S, Tracer>
    where
        Tracer: otel::Tracer + PreSampledTracer + 'static,
    {
        OpenTelemetryLayer {
            tracer,
            event_location: self.event_location,
            tracked_inactivity: self.tracked_inactivity,
            get_context: WithContext(OpenTelemetryLayer::<S, Tracer>::get_context),
            _registry: self._registry,
        }
    }

    /// Sets whether or not event span's metadata should include detailed location
    /// information, such as the file, module and line number.
    ///
    /// By default, event locations are enabled.
    pub fn with_event_location(self, event_location: bool) -> Self {
        Self {
            event_location,
            ..self
        }
    }

    /// Sets whether or not spans metadata should include the _busy time_
    /// (total time for which it was entered), and _idle time_ (total time
    /// the span existed but was not entered).
    pub fn with_tracked_inactivity(self, tracked_inactivity: bool) -> Self {
        Self {
            tracked_inactivity,
            ..self
        }
    }

    /// Retrieve the parent OpenTelemetry [`Context`] from the current tracing
    /// [`span`] through the [`Registry`]. This [`Context`] links spans to their
    /// parent for proper hierarchical visualization.
    ///
    /// [`Context`]: opentelemetry::Context
    /// [`span`]: tracing::Span
    /// [`Registry`]: tracing_subscriber::Registry
    fn parent_context(&self, attrs: &Attributes<'_>, ctx: &Context<'_, S>) -> OtelContext {
        // If a span is specified, it _should_ exist in the underlying `Registry`.
        if let Some(parent) = attrs.parent() {
            let span = ctx.span(parent).expect("Span not found, this is a bug");
            let mut extensions = span.extensions_mut();
            extensions
                .get_mut::<OtelData>()
                .map(|builder| self.tracer.sampled_context(builder))
                .unwrap_or_default()
        // Else if the span is inferred from context, look up any available current span.
        } else if attrs.is_contextual() {
            ctx.lookup_current()
                .and_then(|span| {
                    let mut extensions = span.extensions_mut();
                    extensions
                        .get_mut::<OtelData>()
                        .map(|builder| self.tracer.sampled_context(builder))
                })
                .unwrap_or_else(OtelContext::current)
        // Explicit root spans should have no parent context.
        } else {
            OtelContext::new()
        }
    }

    fn get_context(
        dispatch: &tracing::Dispatch,
        id: &span::Id,
        f: &mut dyn FnMut(&mut OtelData, &dyn PreSampledTracer),
    ) {
        let subscriber = dispatch
            .downcast_ref::<S>()
            .expect("subscriber should downcast to expected type; this is a bug!");
        let span = subscriber
            .span(id)
            .expect("registry should have a span for the current ID");
        let layer = dispatch
            .downcast_ref::<OpenTelemetryLayer<S, T>>()
            .expect("layer should downcast to expected type; this is a bug!");

        let mut extensions = span.extensions_mut();
        if let Some(builder) = extensions.get_mut::<OtelData>() {
            f(builder, &layer.tracer);
        }
    }
}

impl<S, T> Layer<S> for OpenTelemetryLayer<S, T>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
    T: otel::Tracer + PreSampledTracer + 'static,
{
    /// Creates an [OpenTelemetry `Span`] for the corresponding [tracing `Span`].
    ///
    /// [OpenTelemetry `Span`]: opentelemetry::trace::Span
    /// [tracing `Span`]: tracing::Span
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();

        if self.tracked_inactivity && extensions.get_mut::<Timings>().is_none() {
            extensions.insert(Timings::new());
        }

        let parent_cx = self.parent_context(attrs, &ctx);
        let mut builder = self
            .tracer
            .span_builder(attrs.metadata().name())
            .with_start_time(SystemTime::now())
            // Eagerly assign span id so children have stable parent id
            .with_span_id(self.tracer.new_span_id());

        // Record new trace id if there is no active parent span
        if !parent_cx.has_active_span() {
            builder.trace_id = Some(self.tracer.new_trace_id());
        }

        let builder_attrs = builder
            .attributes
            .get_or_insert(Vec::with_capacity(attrs.fields().len() + 3));

        let meta = attrs.metadata();

        if let Some(filename) = meta.file() {
            builder_attrs.push(KeyValue::new("code.filepath", filename));
        }

        if let Some(module) = meta.module_path() {
            builder_attrs.push(KeyValue::new("code.namespace", module));
        }

        if let Some(line) = meta.line() {
            builder_attrs.push(KeyValue::new("code.lineno", line as i64));
        }

        attrs.record(&mut SpanAttributeVisitor(&mut builder));
        extensions.insert(OtelData { builder, parent_cx });
    }

    fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>) {
        if !self.tracked_inactivity {
            return;
        }

        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();

        if let Some(timings) = extensions.get_mut::<Timings>() {
            let now = Instant::now();
            timings.idle += (now - timings.last).as_nanos() as i64;
            timings.last = now;
        }
    }

    fn on_exit(&self, id: &span::Id, ctx: Context<'_, S>) {
        if !self.tracked_inactivity {
            return;
        }

        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();

        if let Some(timings) = extensions.get_mut::<Timings>() {
            let now = Instant::now();
            timings.busy += (now - timings.last).as_nanos() as i64;
            timings.last = now;
        }
    }

    /// Record OpenTelemetry [`attributes`] for the given values.
    ///
    /// [`attributes`]: opentelemetry::trace::SpanBuilder::attributes
    fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();
        if let Some(data) = extensions.get_mut::<OtelData>() {
            values.record(&mut SpanAttributeVisitor(&mut data.builder));
        }
    }

    fn on_follows_from(&self, id: &Id, follows: &Id, ctx: Context<S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();
        let data = extensions
            .get_mut::<OtelData>()
            .expect("Missing otel data span extensions");

        let follows_span = ctx
            .span(follows)
            .expect("Span to follow not found, this is a bug");
        let mut follows_extensions = follows_span.extensions_mut();
        let follows_data = follows_extensions
            .get_mut::<OtelData>()
            .expect("Missing otel data span extensions");

        let follows_context = self
            .tracer
            .sampled_context(follows_data)
            .span()
            .span_context()
            .clone();
        let follows_link = otel::Link::new(follows_context, Vec::new());
        if let Some(ref mut links) = data.builder.links {
            links.push(follows_link);
        } else {
            data.builder.links = Some(vec![follows_link]);
        }
    }

    /// Records OpenTelemetry [`Event`] data on event.
    ///
    /// Note: an [`ERROR`]-level event will also set the OpenTelemetry span status code to
    /// [`Error`], signaling that an error has occurred.
    ///
    /// [`Event`]: opentelemetry::trace::Event
    /// [`ERROR`]: tracing::Level::ERROR
    /// [`Error`]: opentelemetry::trace::StatusCode::Error
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        // Ignore events that are not in the context of a span
        if let Some(span) = ctx.lookup_current() {
            // Performing read operations before getting a write lock to avoid a deadlock
            // See https://github.com/tokio-rs/tracing/issues/763
            #[cfg(feature = "tracing-log")]
            let normalized_meta = event.normalized_metadata();
            #[cfg(feature = "tracing-log")]
            let meta = normalized_meta.as_ref().unwrap_or_else(|| event.metadata());
            #[cfg(not(feature = "tracing-log"))]
            let meta = event.metadata();

            let target = Key::new("target");

            #[cfg(feature = "tracing-log")]
            let target = if normalized_meta.is_some() {
                target.string(meta.target().to_owned())
            } else {
                target.string(event.metadata().target())
            };

            #[cfg(not(feature = "tracing-log"))]
            let target = target.string(meta.target());

            let mut otel_event = otel::Event::new(
                String::new(),
                SystemTime::now(),
                vec![Key::new("level").string(meta.level().as_str()), target],
                0,
            );
            event.record(&mut SpanEventVisitor(&mut otel_event));

            let mut extensions = span.extensions_mut();
            if let Some(OtelData { builder, .. }) = extensions.get_mut::<OtelData>() {
                if builder.status_code.is_none() && *meta.level() == tracing_core::Level::ERROR {
                    builder.status_code = Some(otel::StatusCode::Error);
                }

                if self.event_location {
                    let builder_attrs = builder.attributes.get_or_insert(Vec::new());

                    #[cfg(not(feature = "tracing-log"))]
                    let normalized_meta: Option<tracing_core::Metadata<'_>> = None;
                    let (file, module) = match &normalized_meta {
                        Some(meta) => (
                            meta.file().map(|s| Value::from(s.to_owned())),
                            meta.module_path().map(|s| Value::from(s.to_owned())),
                        ),
                        None => (
                            event.metadata().file().map(Value::from),
                            event.metadata().module_path().map(Value::from),
                        ),
                    };

                    if let Some(file) = file {
                        builder_attrs.push(KeyValue::new("code.filepath", file));
                    }
                    if let Some(module) = module {
                        builder_attrs.push(KeyValue::new("code.namespace", module));
                    }
                    if let Some(line) = meta.line() {
                        builder_attrs.push(KeyValue::new("code.lineno", line as i64));
                    }
                }

                if let Some(ref mut events) = builder.events {
                    events.push(otel_event);
                } else {
                    builder.events = Some(vec![otel_event]);
                }
            }
        };
    }

    /// Exports an OpenTelemetry [`Span`] on close.
    ///
    /// [`Span`]: opentelemetry::trace::Span
    fn on_close(&self, id: span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();

        if let Some(OtelData {
            mut builder,
            parent_cx,
        }) = extensions.remove::<OtelData>()
        {
            if self.tracked_inactivity {
                // Append busy/idle timings when enabled.
                if let Some(timings) = extensions.get_mut::<Timings>() {
                    let busy_ns = KeyValue::new("busy_ns", timings.busy);
                    let idle_ns = KeyValue::new("idle_ns", timings.idle);

                    if let Some(ref mut attributes) = builder.attributes {
                        attributes.push(busy_ns);
                        attributes.push(idle_ns);
                    } else {
                        builder.attributes = Some(vec![busy_ns, idle_ns]);
                    }
                }
            }

            // Assign end time, build and start span, drop span to export
            builder
                .with_end_time(SystemTime::now())
                .start_with_context(&self.tracer, &parent_cx);
        }
    }

    // SAFETY: this is safe because the `WithContext` function pointer is valid
    // for the lifetime of `&self`.
    unsafe fn downcast_raw(&self, id: TypeId) -> Option<*const ()> {
        match id {
            id if id == TypeId::of::<Self>() => Some(self as *const _ as *const ()),
            id if id == TypeId::of::<WithContext>() => {
                Some(&self.get_context as *const _ as *const ())
            }
            _ => None,
        }
    }
}

struct Timings {
    idle: i64,
    busy: i64,
    last: Instant,
}

impl Timings {
    fn new() -> Self {
        Self {
            idle: 0,
            busy: 0,
            last: Instant::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OtelData;
    use opentelemetry::trace::{noop, SpanKind, TraceFlags};
    use std::borrow::Cow;
    use std::sync::{Arc, Mutex};
    use std::time::SystemTime;
    use tracing_subscriber::prelude::*;

    #[derive(Debug, Clone)]
    struct TestTracer(Arc<Mutex<Option<OtelData>>>);
    impl otel::Tracer for TestTracer {
        type Span = noop::NoopSpan;
        fn start_with_context<T>(&self, _name: T, _context: &OtelContext) -> Self::Span
        where
            T: Into<Cow<'static, str>>,
        {
            noop::NoopSpan::new()
        }
        fn span_builder<T>(&self, name: T) -> otel::SpanBuilder
        where
            T: Into<Cow<'static, str>>,
        {
            otel::SpanBuilder::from_name(name)
        }
        fn build_with_context(
            &self,
            builder: otel::SpanBuilder,
            parent_cx: &OtelContext,
        ) -> Self::Span {
            *self.0.lock().unwrap() = Some(OtelData {
                builder,
                parent_cx: parent_cx.clone(),
            });
            noop::NoopSpan::new()
        }
    }

    impl PreSampledTracer for TestTracer {
        fn sampled_context(&self, _builder: &mut crate::OtelData) -> OtelContext {
            OtelContext::new()
        }
        fn new_trace_id(&self) -> otel::TraceId {
            otel::TraceId::INVALID
        }
        fn new_span_id(&self) -> otel::SpanId {
            otel::SpanId::INVALID
        }
    }

    #[derive(Debug, Clone)]
    struct TestSpan(otel::SpanContext);
    impl otel::Span for TestSpan {
        fn add_event_with_timestamp<T: Into<Cow<'static, str>>>(
            &mut self,
            _: T,
            _: SystemTime,
            _: Vec<KeyValue>,
        ) {
        }
        fn span_context(&self) -> &otel::SpanContext {
            &self.0
        }
        fn is_recording(&self) -> bool {
            false
        }
        fn set_attribute(&mut self, _attribute: KeyValue) {}
        fn set_status(&mut self, _code: otel::StatusCode, _message: String) {}
        fn update_name<T: Into<Cow<'static, str>>>(&mut self, _new_name: T) {}
        fn end_with_timestamp(&mut self, _timestamp: SystemTime) {}
    }

    #[test]
    fn dynamic_span_names() {
        let dynamic_name = "GET http://example.com".to_string();
        let tracer = TestTracer(Arc::new(Mutex::new(None)));
        let subscriber = tracing_subscriber::registry().with(layer().with_tracer(tracer.clone()));

        tracing::subscriber::with_default(subscriber, || {
            tracing::debug_span!("static_name", otel.name = dynamic_name.as_str());
        });

        let recorded_name = tracer
            .0
            .lock()
            .unwrap()
            .as_ref()
            .map(|b| b.builder.name.clone());
        assert_eq!(recorded_name, Some(dynamic_name.into()))
    }

    #[test]
    fn span_kind() {
        let tracer = TestTracer(Arc::new(Mutex::new(None)));
        let subscriber = tracing_subscriber::registry().with(layer().with_tracer(tracer.clone()));

        tracing::subscriber::with_default(subscriber, || {
            tracing::debug_span!("request", otel.kind = %SpanKind::Server);
        });

        let recorded_kind = tracer
            .0
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .builder
            .span_kind
            .clone();
        assert_eq!(recorded_kind, Some(otel::SpanKind::Server))
    }

    #[test]
    fn span_status_code() {
        let tracer = TestTracer(Arc::new(Mutex::new(None)));
        let subscriber = tracing_subscriber::registry().with(layer().with_tracer(tracer.clone()));

        tracing::subscriber::with_default(subscriber, || {
            tracing::debug_span!("request", otel.status_code = ?otel::StatusCode::Ok);
        });
        let recorded_status_code = tracer
            .0
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .builder
            .status_code;
        assert_eq!(recorded_status_code, Some(otel::StatusCode::Ok))
    }

    #[test]
    fn span_status_message() {
        let tracer = TestTracer(Arc::new(Mutex::new(None)));
        let subscriber = tracing_subscriber::registry().with(layer().with_tracer(tracer.clone()));

        let message = "message";

        tracing::subscriber::with_default(subscriber, || {
            tracing::debug_span!("request", otel.status_message = message);
        });

        let recorded_status_message = tracer
            .0
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .builder
            .status_message
            .clone();

        assert_eq!(recorded_status_message, Some(message.into()))
    }

    #[test]
    fn trace_id_from_existing_context() {
        let tracer = TestTracer(Arc::new(Mutex::new(None)));
        let subscriber = tracing_subscriber::registry().with(layer().with_tracer(tracer.clone()));
        let trace_id = otel::TraceId::from(42u128.to_be_bytes());
        let existing_cx = OtelContext::current_with_span(TestSpan(otel::SpanContext::new(
            trace_id,
            otel::SpanId::from(1u64.to_be_bytes()),
            TraceFlags::default(),
            false,
            Default::default(),
        )));
        let _g = existing_cx.attach();

        tracing::subscriber::with_default(subscriber, || {
            tracing::debug_span!("request", otel.kind = %SpanKind::Server);
        });

        let recorded_trace_id = tracer
            .0
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .parent_cx
            .span()
            .span_context()
            .trace_id();
        assert_eq!(recorded_trace_id, trace_id)
    }

    #[test]
    fn includes_timings() {
        let tracer = TestTracer(Arc::new(Mutex::new(None)));
        let subscriber = tracing_subscriber::registry().with(
            layer()
                .with_tracer(tracer.clone())
                .with_tracked_inactivity(true),
        );

        tracing::subscriber::with_default(subscriber, || {
            tracing::debug_span!("request");
        });

        let attributes = tracer
            .0
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .builder
            .attributes
            .as_ref()
            .unwrap()
            .clone();
        let keys = attributes
            .iter()
            .map(|attr| attr.key.as_str())
            .collect::<Vec<&str>>();
        assert!(keys.contains(&"idle_ns"));
        assert!(keys.contains(&"busy_ns"));
    }
}
