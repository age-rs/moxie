use {
    crate::*,
    std::fmt::{Debug, Formatter, Result as FmtResult},
};

pub trait Event: AsRef<web_sys::Event> {
    const NAME: &'static str;
}

struct Callback {
    cb: Closure<dyn FnMut()>,
}

impl Callback {
    fn new<Ev, State, Updater>(key: Key<State>, updater: Updater) -> Self
    where
        Ev: Event,
        Updater: FnMut(&State, Ev) -> Option<State>,
    {
        let cb = Closure::wrap(Box::new(move || {
            debug!("callback called");
        }) as Box<dyn FnMut()>);
        Self { cb }
    }

    fn as_fn(&self) -> &js_sys::Function {
        self.cb.as_ref().unchecked_ref()
    }
}

#[must_use]
pub struct EventHandle {
    target: web_sys::EventTarget,
    callback: Callback,
    name: &'static str,
}

impl EventHandle {
    fn new<Ev, State, Updater>(
        target: web_sys::EventTarget,
        key: Key<State>,
        updater: Updater,
    ) -> Self
    where
        Ev: Event,
        Updater: FnMut(&State, Ev) -> Option<State>,
    {
        let callback = Callback::new(key, updater);
        debug!("binding event listener");
        let name = Ev::NAME;
        target
            .add_event_listener_with_callback(name, callback.as_fn())
            .unwrap();
        Self {
            target,
            callback,
            name,
        }
    }
}

impl Drop for EventHandle {
    fn drop(&mut self) {
        debug!("removing event listener");
        self.target
            .remove_event_listener_with_callback(self.name, self.callback.as_fn())
            .unwrap();
    }
}

pub trait EventTarget: Sized {
    fn handlers(&mut self) -> &mut Handlers;

    fn on<Ev, State, Updater>(mut self, key: Key<State>, updater: Updater) -> Self
    where
        Ev: 'static + Event,
        State: 'static,
        Updater: 'static + FnMut(&State, Ev) -> Option<State>,
    {
        self.handlers().add_listener(key, updater);
        self
    }
}

#[derive(Default)]
pub struct Handlers {
    inner: Vec<Box<dyn FnOnce(&web_sys::EventTarget) -> EventHandle + 'static>>,
}

impl Handlers {
    fn add_listener<Ev, State, Updater>(&mut self, key: Key<State>, updater: Updater)
    where
        Ev: Event,
        State: 'static,
        Updater: 'static + FnMut(&State, Ev) -> Option<State>,
    {
        self.inner.push(Box::new(move |target| -> EventHandle {
            EventHandle::new(target.to_owned(), key, updater)
        }));
    }

    pub(crate) fn apply(self, target: &web_sys::EventTarget) -> Vec<EventHandle> {
        self.inner
            .into_iter()
            .map(|handler| handler(target))
            .collect()
    }
}

impl Debug for Handlers {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.debug_struct("Handlers").finish()
    }
}

pub struct ClickEvent(web_sys::MouseEvent);

impl AsRef<web_sys::Event> for ClickEvent {
    fn as_ref(&self) -> &web_sys::Event {
        self.0.as_ref()
    }
}

impl Event for ClickEvent {
    const NAME: &'static str = "click";
}
