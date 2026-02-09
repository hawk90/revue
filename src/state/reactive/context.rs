//! Context API for sharing state across the component tree.
//!
//! Provides Vue/React-like context for sharing state without prop drilling.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::reactive::{create_context, provide, use_context};
//!
//! // Define a context
//! let theme_context = create_context::<String>();
//!
//! // Provide a value
//! provide(&theme_context, "dark".to_string());
//!
//! // Consume the value
//! let theme = use_context(&theme_context);
//! assert_eq!(theme, Some("dark".to_string()));
//! ```

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use super::Signal;

// =============================================================================
// Context Types
// =============================================================================

/// Unique identifier for a context
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ContextId(u64);

impl ContextId {
    /// Create a new unique context ID
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

/// A context definition for sharing state
///
/// Created with [`create_context`] and used with [`provide`] and [`use_context`].
#[derive(Debug)]
pub struct Context<T> {
    id: ContextId,
    default: Option<T>,
    _marker: PhantomData<T>,
}

impl<T> Context<T> {
    /// Create a new context without a default value
    pub fn new() -> Self {
        Self {
            id: ContextId::new(),
            default: None,
            _marker: PhantomData,
        }
    }

    /// Create a new context with a default value
    pub fn with_default(default: T) -> Self {
        Self {
            id: ContextId::new(),
            default: Some(default),
            _marker: PhantomData,
        }
    }

    /// Get the context ID
    pub fn id(&self) -> ContextId {
        self.id
    }

    /// Get the default value if set
    pub fn default(&self) -> Option<&T> {
        self.default.as_ref()
    }
}

impl<T> Default for Context<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for Context<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            default: self.default.clone(),
            _marker: PhantomData,
        }
    }
}

// =============================================================================
// Context Provider
// =============================================================================

/// A provider that holds a context value
pub struct Provider<T: Clone + Send + Sync + 'static> {
    context_id: ContextId,
    value: Signal<T>,
}

impl<T: Clone + Send + Sync + 'static> Provider<T> {
    /// Create a new provider
    pub fn new(context: &Context<T>, value: T) -> Self {
        Self {
            context_id: context.id,
            value: Signal::new(value),
        }
    }

    /// Get the current value
    pub fn get(&self) -> T {
        self.value.get()
    }

    /// Set a new value
    pub fn set(&self, value: T) {
        self.value.set(value);
    }

    /// Update the value
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        self.value.update(f);
    }

    /// Get the underlying signal
    pub fn signal(&self) -> &Signal<T> {
        &self.value
    }
}

impl<T: Clone + Send + Sync + 'static> Clone for Provider<T> {
    fn clone(&self) -> Self {
        Self {
            context_id: self.context_id,
            value: self.value.clone(),
        }
    }
}

// =============================================================================
// Context Store (Thread-Local)
// =============================================================================

type ContextValue = Arc<dyn Any + Send + Sync>;

thread_local! {
    /// Stack of context scopes for nested providers
    ///
    /// Each scope is a HashMap of ContextId -> ContextValue.
    /// Scopes are pushed/popped by [`ContextScope`] for automatic cleanup.
    static CONTEXT_STACK: RefCell<Vec<HashMap<ContextId, ContextValue>>> = RefCell::new(Vec::new());

    /// Global context store for root-level providers
    ///
    /// Stores context values that persist until explicitly cleared.
    /// Use [`clear_context`] or [`clear_all_contexts`] for cleanup.
    ///
    /// # Memory Notes
    ///
    /// - Contexts persist for the lifetime of the thread
    /// - Creating many unique contexts over time may consume unbounded memory
    /// - For dynamic/contextual contexts, use [`ContextScope`] instead
    static GLOBAL_CONTEXTS: RefCell<HashMap<ContextId, ContextValue>> = RefCell::new(HashMap::new());
}

// =============================================================================
// Public API
// =============================================================================

/// Create a new context
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::create_context;
///
/// let theme_context = create_context::<String>();
/// ```
pub fn create_context<T>() -> Context<T> {
    Context::new()
}

/// Create a new context with a default value
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::create_context_with_default;
///
/// let theme_context = create_context_with_default("light".to_string());
/// ```
pub fn create_context_with_default<T>(default: T) -> Context<T> {
    Context::with_default(default)
}

/// Provide a context value
///
/// The value will be available to all descendants via [`use_context`].
///
/// # Notes
///
/// - Provided values persist until explicitly cleared with [`clear_context`]
/// - Creating many unique contexts over time may consume unbounded memory
/// - For scoped contexts, consider using [`ContextScope`] instead
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::{create_context, provide};
///
/// let theme = create_context::<String>();
/// provide(&theme, "dark".to_string());
/// ```
pub fn provide<T: Clone + Send + Sync + 'static>(context: &Context<T>, value: T) {
    let signal = Signal::new(value);
    let boxed: ContextValue = Arc::new(signal);

    GLOBAL_CONTEXTS.with(|store| {
        store.borrow_mut().insert(context.id, boxed);
    });
}

/// Provide a context value with a signal for reactive updates
///
/// Returns the signal so you can update the value later.
///
/// # Notes
///
/// - Provided values persist until explicitly cleared with [`clear_context`]
/// - Creating many unique contexts over time may consume unbounded memory
/// - For scoped contexts, consider using [`ContextScope`] instead
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::{create_context, provide_signal};
///
/// let theme = create_context::<String>();
/// let theme_signal = provide_signal(&theme, "dark".to_string());
///
/// // Later, update the theme
/// theme_signal.set("light".to_string());
/// ```
pub fn provide_signal<T: Clone + Send + Sync + 'static>(
    context: &Context<T>,
    value: T,
) -> Signal<T> {
    let signal = Signal::new(value);
    let boxed: ContextValue = Arc::new(signal.clone());

    GLOBAL_CONTEXTS.with(|store| {
        store.borrow_mut().insert(context.id, boxed);
    });

    signal
}

/// Use a context value
///
/// Returns the current value from the nearest provider, or the default value if set.
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::{create_context, provide, use_context};
///
/// let theme = create_context::<String>();
/// provide(&theme, "dark".to_string());
///
/// let current_theme = use_context(&theme);
/// assert_eq!(current_theme, Some("dark".to_string()));
/// ```
pub fn use_context<T: Clone + Send + Sync + 'static>(context: &Context<T>) -> Option<T> {
    // First check the context stack (scoped providers)
    let from_stack = CONTEXT_STACK.with(|stack| {
        let stack = stack.borrow();
        // Search from innermost to outermost scope
        for scope in stack.iter().rev() {
            if let Some(value) = scope.get(&context.id) {
                if let Some(signal) = value.downcast_ref::<Signal<T>>() {
                    return Some(signal.get());
                }
            }
        }
        None
    });

    if from_stack.is_some() {
        return from_stack;
    }

    // Then check global contexts
    let from_global = GLOBAL_CONTEXTS.with(|store| {
        let store = store.borrow();
        if let Some(value) = store.get(&context.id) {
            if let Some(signal) = value.downcast_ref::<Signal<T>>() {
                return Some(signal.get());
            }
        }
        None
    });

    if from_global.is_some() {
        return from_global;
    }

    // Fall back to default
    context.default.clone()
}

/// Use a context signal for reactive updates
///
/// Returns a clone of the signal so updates to the signal trigger reactivity.
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::{create_context, provide, use_context_signal, effect};
///
/// let theme = create_context::<String>();
/// provide(&theme, "dark".to_string());
///
/// let theme_signal = use_context_signal(&theme);
///
/// effect(move || {
///     if let Some(sig) = &theme_signal {
///         println!("Theme changed to: {}", sig.get());
///     }
/// });
/// ```
pub fn use_context_signal<T: Clone + Send + Sync + 'static>(
    context: &Context<T>,
) -> Option<Signal<T>> {
    // First check the context stack
    let from_stack = CONTEXT_STACK.with(|stack| {
        let stack = stack.borrow();
        for scope in stack.iter().rev() {
            if let Some(value) = scope.get(&context.id) {
                if let Some(signal) = value.downcast_ref::<Signal<T>>() {
                    return Some(signal.clone());
                }
            }
        }
        None
    });

    if from_stack.is_some() {
        return from_stack;
    }

    // Then check global contexts
    GLOBAL_CONTEXTS.with(|store| {
        let store = store.borrow();
        if let Some(value) = store.get(&context.id) {
            if let Some(signal) = value.downcast_ref::<Signal<T>>() {
                return Some(signal.clone());
            }
        }
        None
    })
}

/// Check if a context has a provided value
pub fn has_context<T: Clone + Send + Sync + 'static>(context: &Context<T>) -> bool {
    // Check stack
    let in_stack = CONTEXT_STACK.with(|stack| {
        let stack = stack.borrow();
        for scope in stack.iter().rev() {
            if scope.contains_key(&context.id) {
                return true;
            }
        }
        false
    });

    if in_stack {
        return true;
    }

    // Check global
    GLOBAL_CONTEXTS.with(|store| store.borrow().contains_key(&context.id))
}

/// Clear a context value
///
/// Removes the context value from the global store.
/// Use this to free memory when a context is no longer needed.
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::{create_context, provide, clear_context};
///
/// let theme = create_context::<String>();
/// provide(&theme, "dark".to_string());
///
/// // Later, when the theme is no longer needed:
/// clear_context(&theme);
/// ```
pub fn clear_context<T>(context: &Context<T>) {
    GLOBAL_CONTEXTS.with(|store| {
        store.borrow_mut().remove(&context.id);
    });
}

/// Clear all context values
///
/// Removes all context values from both the global store and the scope stack.
/// Useful for testing or resetting the application state.
pub fn clear_all_contexts() {
    GLOBAL_CONTEXTS.with(|store| {
        store.borrow_mut().clear();
    });
    CONTEXT_STACK.with(|stack| {
        stack.borrow_mut().clear();
    });
}

// =============================================================================
// Scoped Context
// =============================================================================

/// A scope for providing context values to a specific subtree
///
/// When the scope is dropped, the context values are removed.
pub struct ContextScope {
    _private: (),
}

impl ContextScope {
    /// Create a new context scope
    pub fn new() -> Self {
        CONTEXT_STACK.with(|stack| {
            stack.borrow_mut().push(HashMap::new());
        });
        Self { _private: () }
    }

    /// Provide a value within this scope
    pub fn provide<T: Clone + Send + Sync + 'static>(&self, context: &Context<T>, value: T) {
        let signal = Signal::new(value);
        let boxed: ContextValue = Arc::new(signal);

        CONTEXT_STACK.with(|stack| {
            let mut stack = stack.borrow_mut();
            if let Some(scope) = stack.last_mut() {
                scope.insert(context.id, boxed);
            }
        });
    }

    /// Provide a signal within this scope
    pub fn provide_signal<T: Clone + Send + Sync + 'static>(
        &self,
        context: &Context<T>,
        value: T,
    ) -> Signal<T> {
        let signal = Signal::new(value);
        let boxed: ContextValue = Arc::new(signal.clone());

        CONTEXT_STACK.with(|stack| {
            let mut stack = stack.borrow_mut();
            if let Some(scope) = stack.last_mut() {
                scope.insert(context.id, boxed);
            }
        });

        signal
    }
}

impl Default for ContextScope {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ContextScope {
    fn drop(&mut self) {
        CONTEXT_STACK.with(|stack| {
            stack.borrow_mut().pop();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Context::new tests
    #[test]
    fn test_context_new() {
        let ctx: Context<i32> = Context::new();
        assert!(ctx.default().is_none());
    }

    #[test]
    fn test_context_new_string() {
        let ctx: Context<String> = Context::new();
        assert!(ctx.default().is_none());
    }

    // Context::with_default tests
    #[test]
    fn test_context_with_default() {
        let ctx: Context<i32> = Context::with_default(42);
        assert_eq!(ctx.default(), Some(&42));
    }

    #[test]
    fn test_context_with_default_string() {
        let ctx: Context<String> = Context::with_default("hello".to_string());
        assert_eq!(ctx.default(), Some(&"hello".to_string()));
    }

    // Context::default trait tests
    #[test]
    fn test_context_default_trait() {
        let ctx: Context<i32> = Default::default();
        assert!(ctx.default().is_none());
    }

    // Context::id tests
    #[test]
    fn test_context_id() {
        let ctx1: Context<i32> = Context::new();
        let ctx2: Context<i32> = Context::new();
        assert_ne!(ctx1.id(), ctx2.id());
    }

    // Context::clone tests
    #[test]
    fn test_context_clone_without_default() {
        let ctx1: Context<i32> = Context::new();
        let ctx2 = ctx1.clone();
        assert_eq!(ctx1.id(), ctx2.id());
        assert!(ctx2.default().is_none());
    }

    #[test]
    fn test_context_clone_with_default() {
        let ctx1: Context<i32> = Context::with_default(42);
        let ctx2 = ctx1.clone();
        assert_eq!(ctx1.id(), ctx2.id());
        assert_eq!(ctx2.default(), Some(&42));
    }

    // create_context tests
    #[test]
    fn test_create_context() {
        let ctx = create_context::<i32>();
        assert!(ctx.default().is_none());
    }

    #[test]
    fn test_create_context_string() {
        let ctx = create_context::<String>();
        assert!(ctx.default().is_none());
    }

    // create_context_with_default tests
    #[test]
    fn test_create_context_with_default() {
        let ctx = create_context_with_default(42);
        assert_eq!(ctx.default(), Some(&42));
    }

    // provide/use_context tests
    #[test]
    fn test_provide_and_use_context() {
        let ctx = create_context::<i32>();
        provide(&ctx, 42);
        assert_eq!(use_context(&ctx), Some(42));
        clear_context(&ctx);
    }

    #[test]
    fn test_provide_and_use_context_string() {
        let ctx = create_context::<String>();
        provide(&ctx, "hello".to_string());
        assert_eq!(use_context(&ctx), Some("hello".to_string()));
        clear_context(&ctx);
    }

    #[test]
    fn test_use_context_not_provided() {
        let ctx = create_context::<i32>();
        assert_eq!(use_context(&ctx), None);
    }

    #[test]
    fn test_use_context_with_default_not_provided() {
        let ctx = create_context_with_default(42);
        assert_eq!(use_context(&ctx), Some(42));
    }

    #[test]
    fn test_use_context_default_overridden_by_provide() {
        let ctx = create_context_with_default(42);
        provide(&ctx, 100);
        assert_eq!(use_context(&ctx), Some(100));
        clear_context(&ctx);
    }

    // provide_signal tests
    #[test]
    fn test_provide_signal_and_use_context() {
        let ctx = create_context::<i32>();
        let signal = provide_signal(&ctx, 42);
        assert_eq!(signal.get(), 42);
        assert_eq!(use_context(&ctx), Some(42));
        clear_context(&ctx);
    }

    #[test]
    fn test_provide_signal_update() {
        let ctx = create_context::<i32>();
        let signal = provide_signal(&ctx, 42);
        assert_eq!(use_context(&ctx), Some(42));

        signal.set(100);
        assert_eq!(use_context(&ctx), Some(100));
        clear_context(&ctx);
    }

    // use_context_signal tests
    #[test]
    fn test_use_context_signal() {
        let ctx = create_context::<i32>();
        provide(&ctx, 42);
        let signal = use_context_signal(&ctx);
        assert!(signal.is_some());
        assert_eq!(signal.unwrap().get(), 42);
        clear_context(&ctx);
    }

    #[test]
    fn test_use_context_signal_not_provided() {
        let ctx = create_context::<i32>();
        let signal = use_context_signal(&ctx);
        assert!(signal.is_none());
    }

    // has_context tests
    #[test]
    fn test_has_context_false() {
        let ctx = create_context::<i32>();
        assert!(!has_context(&ctx));
    }

    #[test]
    fn test_has_context_true() {
        let ctx = create_context::<i32>();
        provide(&ctx, 42);
        assert!(has_context(&ctx));
        clear_context(&ctx);
    }

    // clear_context tests
    #[test]
    fn test_clear_context() {
        let ctx = create_context::<i32>();
        provide(&ctx, 42);
        assert!(has_context(&ctx));

        clear_context(&ctx);
        assert!(!has_context(&ctx));
    }

    // clear_all_contexts tests
    #[test]
    fn test_clear_all_contexts() {
        let ctx1 = create_context::<i32>();
        let ctx2 = create_context::<String>();
        provide(&ctx1, 42);
        provide(&ctx2, "hello".to_string());

        assert!(has_context(&ctx1));
        assert!(has_context(&ctx2));

        clear_all_contexts();

        assert!(!has_context(&ctx1));
        assert!(!has_context(&ctx2));
    }

    // ContextScope tests
    #[test]
    fn test_context_scope_new() {
        let _scope = ContextScope::new();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_context_scope_default() {
        let _scope = ContextScope::default();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_context_scope_provide() {
        let ctx = create_context::<i32>();
        {
            let scope = ContextScope::new();
            scope.provide(&ctx, 42);
            assert_eq!(use_context(&ctx), Some(42));
        }
        // After scope is dropped
        assert_eq!(use_context(&ctx), None);
    }

    #[test]
    fn test_context_scope_provide_signal() {
        let ctx = create_context::<i32>();
        let _signal = {
            let scope = ContextScope::new();
            let signal = scope.provide_signal(&ctx, 42);
            assert_eq!(signal.get(), 42);
            assert_eq!(use_context(&ctx), Some(42));
            signal
        };
        // After scope is dropped, signal still exists but context is gone
        assert_eq!(use_context(&ctx), None);
    }

    #[test]
    fn test_context_scope_nested() {
        let ctx = create_context::<i32>();

        // Outer scope
        let scope1 = ContextScope::new();
        scope1.provide(&ctx, 10);
        assert_eq!(use_context(&ctx), Some(10));

        // Inner scope overrides
        {
            let scope2 = ContextScope::new();
            scope2.provide(&ctx, 20);
            assert_eq!(use_context(&ctx), Some(20));
        }

        // After inner scope dropped, outer value is restored
        assert_eq!(use_context(&ctx), Some(10));
    }

    // Provider tests
    #[test]
    fn test_provider_new() {
        let ctx = create_context::<i32>();
        let provider = Provider::new(&ctx, 42);
        assert_eq!(provider.get(), 42);
    }

    #[test]
    fn test_provider_get() {
        let ctx = create_context::<i32>();
        let provider = Provider::new(&ctx, 42);
        assert_eq!(provider.get(), 42);
    }

    #[test]
    fn test_provider_set() {
        let ctx = create_context::<i32>();
        let provider = Provider::new(&ctx, 42);
        assert_eq!(provider.get(), 42);

        provider.set(100);
        assert_eq!(provider.get(), 100);
    }

    #[test]
    fn test_provider_update() {
        let ctx = create_context::<i32>();
        let provider = Provider::new(&ctx, 42);
        provider.update(|v| *v *= 2);
        assert_eq!(provider.get(), 84);
    }

    #[test]
    fn test_provider_signal() {
        let ctx = create_context::<i32>();
        let provider = Provider::new(&ctx, 42);
        let signal = provider.signal();
        assert_eq!(signal.get(), 42);
    }

    #[test]
    fn test_provider_clone() {
        let ctx = create_context::<i32>();
        let provider1 = Provider::new(&ctx, 42);
        let provider2 = provider1.clone();
        assert_eq!(provider2.get(), 42);
    }

    // Integration tests with different types
    #[test]
    fn test_context_with_bool() {
        let ctx = create_context::<bool>();
        provide(&ctx, true);
        assert_eq!(use_context(&ctx), Some(true));
        clear_context(&ctx);
    }

    #[test]
    fn test_context_with_vec() {
        let ctx = create_context::<Vec<i32>>();
        provide(&ctx, vec![1, 2, 3]);
        assert_eq!(use_context(&ctx), Some(vec![1, 2, 3]));
        clear_context(&ctx);
    }

    #[test]
    fn test_context_with_option() {
        let ctx = create_context::<Option<i32>>();
        provide(&ctx, Some(42));
        assert_eq!(use_context(&ctx), Some(Some(42)));
        clear_context(&ctx);
    }

    // Multiple contexts test
    #[test]
    fn test_multiple_independent_contexts() {
        let ctx1 = create_context::<i32>();
        let ctx2 = create_context::<String>();

        provide(&ctx1, 42);
        provide(&ctx2, "hello".to_string());

        assert_eq!(use_context(&ctx1), Some(42));
        assert_eq!(use_context(&ctx2), Some("hello".to_string()));

        clear_context(&ctx1);
        clear_context(&ctx2);
    }
}

/// Run a function with a scoped context
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::{create_context, with_context_scope, use_context};
///
/// let theme = create_context::<String>();
///
/// with_context_scope(|scope| {
///     scope.provide(&theme, "dark".to_string());
///
///     let value = use_context(&theme);
///     assert_eq!(value, Some("dark".to_string()));
/// });
///
/// // Outside the scope, the value is gone
/// let value = use_context(&theme);
/// assert_eq!(value, None);
/// ```
pub fn with_context_scope<F, R>(f: F) -> R
where
    F: FnOnce(&ContextScope) -> R,
{
    let scope = ContextScope::new();
    f(&scope)
}

// =============================================================================
// Tests
// =============================================================================
