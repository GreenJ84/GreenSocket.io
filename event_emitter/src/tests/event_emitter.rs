use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

use crate::{EventManager, EventEmitter, EventHandler, EventPayload, EventError};

type TestStringPayload = String;
fn test_string_payload(data: &str) -> EventPayload<TestStringPayload> {
    Arc::new(data.to_string())
}

mod adding_listeners {
    use super::*;

    #[test]
    fn add_single_infinite_listener() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            emitter.add_listener("test", Arc::new(move |_| {})).unwrap();
        }

        assert_eq!(emitter.listener_count("test"), 1,"Listener was not added");
    }

    #[test]
    fn add_single_finite_listener() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            emitter.add_limited_listener("test", Arc::new(move |_| {}), 5).unwrap();
        }

        assert_eq!(emitter.listener_count("test"), 1,"Listener was not added");
    }

    #[test]
    fn add_single_once_listener() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            emitter.add_once("test", Arc::new(move |_| {})).unwrap();
        }

        assert_eq!(emitter.listener_count("test"), 1,"Listener was not added");
    }

    #[test]
    fn add_multiple_different_listeners() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());

        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            for idx in 0..5{
                match idx {
                    0 | 2 => {
                        emitter.add_once("once", Arc::new(move |_: &EventPayload<TestStringPayload>| {})).unwrap();
                    }
                    1 | 3=> {
                        emitter.add_limited_listener("limited", Arc::new(move |_: &EventPayload<TestStringPayload>| {}), 4).unwrap();
                    }
                    _ => {
                        emitter.add_listener("unlimited", Arc::new(move |_: &EventPayload<TestStringPayload>| {})).unwrap();
                    }
                }
            }
        }
        for event_idx in 0..3 {
            match event_idx {
                0 => {
                    assert_eq!(emitter.listener_count("once"), 2, "a ONCE Listener was not added")
                },
                1 => {
                    assert_eq!(emitter.listener_count("limited"), 2,"a LIMITED Listener was not added");
                },
                _ => {
                    assert_eq!(emitter.listener_count("unlimited"), 1,"a UNLIMITED Listener was not added")
                }
            }
        }
    }

    #[test]
    fn overloading_event_throws_error() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());

        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            for _ in 0..10{
                emitter.add_listener("test", Arc::new(move |_| {})).unwrap();
            }

            assert_eq!(
                emitter.add_listener("test", Arc::new(move |_| {})),
                Err(EventError::OverloadedEvent),
                "Emitter did not throw a Overload error"
            );
        }
    }
}