//! The event loop: running the app, dispatching events and hot reload

#[cfg(feature = "hot-reload")]
use super::HotReloadEvent;
use super::{is_quit_key, is_tab_key, signals, App};
use crate::constants::FRAME_DURATION_60FPS;
use crate::event::{Event, Key, KeyEvent, MouseButton, MouseEventKind};
use crate::render::Terminal;
use crate::widget::View;
use std::io::stdout;
use std::time::Instant;

impl App {
    /// Run the application with a root view and event handler
    ///
    /// # Arguments
    ///
    /// * `view` - The root view component to render
    /// * `handler` - Callback for handling events, returns whether to redraw
    ///
    /// # Shutdown
    ///
    /// `run` returns `Ok(())` after [`quit`](Self::quit), after plugins have
    /// unmounted and the terminal has been restored. On unix, `SIGTERM`,
    /// `SIGHUP` and `SIGINT` take the same path while `run` is running; a
    /// second signal during that shutdown kills the process. Ctrl+C is not a
    /// signal here - in raw mode it arrives as a key event. Windows has no
    /// equivalent: Ctrl+Break or closing the console ends the process without
    /// unmounting plugins.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Terminal initialization fails (e.g., not a TTY)
    /// - Mouse capture initialization fails
    /// - Drawing to terminal fails
    /// - Event reading fails (e.g., terminal disconnected)
    /// - Terminal restoration fails
    /// - Hot-reload CSS parsing fails (when `hot-reload` feature is enabled)
    ///
    /// # Example
    ///
    /// ```ignore
    /// use revue::prelude::*;
    ///
    /// let mut app = App::new();
    /// app.run(MyView::new(), |event, view, app| {
    ///     match event {
    ///         Event::Key(key) if key.key == 'q' => app.quit(),
    ///         _ => {}
    ///     }
    ///     false
    /// });
    /// ```
    pub fn run<V, H>(&mut self, mut view: V, mut handler: H) -> crate::Result<()>
    where
        V: View,
        H: FnMut(&Event, &mut V, &mut Self) -> bool,
    {
        use crate::event::EventReader;

        // From here on `kill`, a closed terminal window or `kill -INT` ends the
        // loop like `quit()` does, so plugins unmount and the terminal is
        // restored. In raw mode Ctrl+C is a key event and never reaches this.
        // Listening starts before the terminal changes mode, so there is no
        // window in which a signal kills the process mid-TUI.
        let signals = signals::ShutdownSignals::listen();

        let mut terminal = Terminal::new(stdout())?;
        terminal.init_with_mouse(self.mouse_capture)?;

        // Update plugin context with terminal size
        let (width, height) = terminal.size();
        self.plugins.update_terminal_size(width, height);

        // Mount plugins
        if let Err(e) = self.plugins.mount() {
            crate::log_warn!("Plugin mount failed: {}", e);
        }

        self.running = true;
        self.last_tick = Instant::now();

        self.dom.build(&view);
        self.draw(&view, &mut terminal, true)?;

        let reader = EventReader::new(FRAME_DURATION_60FPS);

        while self.running {
            if signals.requested() {
                self.quit();
                break;
            }

            // Check for hot reload events
            #[cfg(feature = "hot-reload")]
            {
                if let Some(should_reload) = self.check_hot_reload() {
                    if should_reload {
                        self.needs_force_redraw = true;
                        self.draw(&view, &mut terminal, true)?;
                    }
                }
            }

            let event = reader.read()?;
            let should_draw = self.handle_event(event, &mut view, &mut handler);

            if should_draw {
                self.draw(&view, &mut terminal, false)?;
            }
        }

        // Unmount plugins before exit
        if let Err(e) = self.plugins.unmount() {
            crate::log_warn!("Plugin unmount failed: {}", e);
        }

        terminal.restore()?;
        Ok(())
    }

    /// Run the application with a simplified key event handler
    ///
    /// This is a convenience method that wraps `run` with a simpler handler signature
    /// that only receives `KeyEvent` instead of all `Event` types.
    ///
    /// # Arguments
    ///
    /// * `view` - The root view component
    /// * `handler` - A function that handles key events and returns whether to redraw
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Terminal initialization fails (e.g., not a TTY)
    /// - Mouse capture initialization fails
    /// - Drawing to terminal fails
    /// - Event reading fails (e.g., terminal disconnected)
    /// - Terminal restoration fails
    /// - Hot-reload CSS parsing fails (when `hot-reload` feature is enabled)
    ///
    /// # Example
    ///
    /// ```ignore
    /// use revue::prelude::*;
    ///
    /// app.run_with_handler(my_view, |key_event, view| {
    ///     view.handle_key(&key_event.key)
    /// })
    /// ```
    pub fn run_with_handler<V, H>(&mut self, view: V, mut handler: H) -> crate::Result<()>
    where
        V: View,
        H: FnMut(&KeyEvent, &mut V) -> bool,
    {
        self.run(view, move |event, view, _app| match event {
            Event::Key(key_event) => handler(key_event, view),
            _ => false,
        })
    }

    /// Handle a single event
    pub(super) fn handle_event<V, H>(&mut self, event: Event, view: &mut V, handler: &mut H) -> bool
    where
        V: View,
        H: FnMut(&Event, &mut V, &mut Self) -> bool,
    {
        let mut should_draw = handler(&event, view, self);

        match event {
            Event::Key(key) if is_quit_key(&key) => {
                self.quit();
                return false;
            }
            Event::Resize(w, h) => {
                self.buffers[0].resize(w, h);
                self.buffers[1].resize(w, h);
                self.plugins.update_terminal_size(w, h);
                self.needs_force_redraw = true;
                self.needs_layout_rebuild = true; // Resize requires full layout rebuild
                should_draw = true;
            }
            Event::Key(key) if self.tab_navigation && is_tab_key(&key) => {
                should_draw |= self.track_tab_focus(&key);
            }
            Event::Mouse(ref mouse) => {
                if self.track_hover(mouse.x, mouse.y) {
                    should_draw = true;
                }
                if matches!(mouse.kind, MouseEventKind::Down(MouseButton::Left))
                    && self.track_click_focus(mouse.x, mouse.y)
                {
                    should_draw = true;
                }
            }
            Event::Tick => {
                let now = Instant::now();
                let delta = now.duration_since(self.last_tick);
                self.last_tick = now;
                // Update both legacy and node-aware transitions
                self.transitions.update(delta);
                self.transitions.update_nodes(delta);
                // Tick plugins
                if let Err(e) = self.plugins.tick(delta) {
                    crate::log_warn!("Plugin tick failed: {}", e);
                }
                if self.transitions.has_active() {
                    should_draw = true;
                }
            }
            _ => {}
        }

        should_draw || self.needs_force_redraw
    }

    /// Drain pending hot reload events and, if a stylesheet file changed,
    /// rebuild the stylesheet. Returns `Some(true)` when a redraw is needed,
    /// `None` when hot reload is off.
    #[cfg(feature = "hot-reload")]
    fn check_hot_reload(&mut self) -> Option<bool> {
        let hr = self.hot_reload.as_mut()?;

        let mut touched = false;
        while let Some(event) = hr.poll() {
            match event {
                // Which path an event names depends on the platform and on how
                // the editor saved (in place, or via a temp file and a rename),
                // so any change in a watched directory re-reads the files;
                // `reload_styles` ignores files whose text did not change.
                HotReloadEvent::StylesheetChanged(path) | HotReloadEvent::FileCreated(path) => {
                    crate::log_debug!("Hot reload: {:?} changed", path);
                    touched = true;
                }
                HotReloadEvent::FileDeleted(path) => {
                    crate::log_debug!("Hot reload: {:?} deleted", path);
                }
                HotReloadEvent::Error(e) => {
                    crate::log_warn!("Hot reload error: {}", e);
                }
            }
        }

        Some(touched && self.reload_styles())
    }

    /// Re-read the stylesheet files and, if any changed, replace the
    /// stylesheet with one rebuilt from all sources. Returns whether it did.
    #[cfg(feature = "hot-reload")]
    fn reload_styles(&mut self) -> bool {
        if !self.style_sources.reload_files() {
            return false;
        }
        self.dom.set_stylesheet(self.style_sources.stylesheet());
        self.needs_force_redraw = true;
        true
    }

    /// Move `:hover` to whatever the pointer is over.
    ///
    /// Returns `true` if it moved, which is when the frame has to be redrawn.
    ///
    /// The map it queries is filled by the paint pass, so this is inert unless
    /// [`dom_from_render`](crate::dom::DomRenderer::dom_from_render) is on -
    /// without it no node below the root is ever associated with an area, and
    /// `:hover` never matched anything anyway.
    fn track_hover(&mut self, x: u16, y: u16) -> bool {
        if !self.dom.dom_from_render() {
            return false;
        }
        let target = self.dom.node_at(x, y);
        self.dom.set_hover_node(target)
    }

    /// Drive [`handle_event`](Self::handle_event) with a no-op user handler.
    ///
    /// The harness needs the real dispatch, not a reimplementation of it -
    /// otherwise a test can agree with itself while the event loop ignores the
    /// event entirely.
    pub(crate) fn dispatch_for_test<V: View>(&mut self, event: Event, view: &mut V) -> bool {
        let mut handler = |_: &Event, _: &mut V, _: &mut Self| false;
        self.handle_event(event, view, &mut handler)
    }

    /// Move focus to whatever focusable thing was clicked.
    ///
    /// Returns `true` if focus moved. A click on nothing focusable - a label,
    /// a plain container - leaves focus where it was, rather than clearing it;
    /// that is what every other toolkit does and what users expect.
    ///
    /// This sets `NodeState.focused` and nothing else. Widgets still read their
    /// own `focused` field, so no widget *behavior* changes yet - what changes
    /// is that `:focus` rules finally match. Handing widgets the node's state
    /// is Phase 2-2.
    fn track_click_focus(&mut self, x: u16, y: u16) -> bool {
        if !self.dom.dom_from_render() {
            return false;
        }
        match self.dom.focus_target_at(x, y) {
            Some(target) => self.dom.set_focus_node(Some(target)),
            None => false,
        }
    }

    /// Move focus to the next or previous focusable node.
    ///
    /// Returns `true` if focus moved. Terminals send Shift+Tab as its own key
    /// (`BackTab`) rather than Tab with a shift flag, but some send the flag
    /// instead, so both are read as backwards.
    fn track_tab_focus(&mut self, key: &KeyEvent) -> bool {
        let backwards = key.key == Key::BackTab || key.shift;
        if backwards {
            self.dom.focus_prev()
        } else {
            self.dom.focus_next()
        }
    }

    /// [`track_hover`](Self::track_hover), for the pipeline harness.
    ///
    /// The harness drives the same code the event loop does; without this it
    /// would have to reimplement the hit test and could then agree with itself
    /// while disagreeing with production.
    pub(crate) fn track_hover_for_test(&mut self, x: u16, y: u16) -> bool {
        self.track_hover(x, y)
    }
}
