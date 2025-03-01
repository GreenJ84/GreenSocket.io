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


mod removing_listeners {
    use crate::Listener;
    use super::*;

    #[test]
    fn remove_single_listener() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            let listener = emitter.add_listener("test", Arc::new(|_| {}));
            assert!(listener.is_ok());
            assert_eq!(emitter.listener_count("test"), 1);

            assert!(emitter.remove_listener("test", &listener.unwrap()).is_ok());
            assert_eq!(emitter.listener_count("test"), 0);
        }
    }

    #[test]
    fn remove_single_finite_listener() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            let listener = emitter.add_limited_listener("test", Arc::new(|_| {}), 5);
            assert!(listener.is_ok());
            assert_eq!(emitter.listener_count("test"), 1);

            assert!(emitter.remove_listener("test", &listener.unwrap()).is_ok());
            assert_eq!(emitter.listener_count("test"), 0);
        }
    }

    #[test]
    fn remove_single_once_listener() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            let listener = emitter.add_once("test", Arc::new(|_| {}));
            assert!(listener.is_ok());
            assert_eq!(emitter.listener_count("test"), 1);

            assert!(emitter.remove_listener("test", &listener.unwrap()).is_ok());
            assert_eq!(emitter.listener_count("test"), 0);
        }
    }

    #[test]
    fn remove_all_listeners() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            for _ in 0..10 {
                emitter.add_listener("test", Arc::new(|_| {})).unwrap();
            }
            assert_eq!(emitter.listener_count("test"), 10);

            assert!(emitter.remove_all_listeners("test").is_ok());
            assert_eq!(emitter.listener_count("test"), 0);
        }
    }

    #[test]
    fn remove_invalid_listener_throws_error() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            emitter.add_listener("test", Arc::new(|_| {})).unwrap();
            assert_eq!(emitter.remove_listener("test", &Listener::<TestStringPayload>::new(Arc::new(|_| {}), None)), Err(EventError::ListenerNotFound));
        }
    }

    #[test]
    fn remove_from_invalid_event_throws_error() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            emitter.add_listener("test", Arc::new(|_| {})).unwrap();
            assert_eq!(emitter.remove_listener("not_test",  &Listener::<TestStringPayload>::new(Arc::new(|_| {}), None)), Err(EventError::EventNotFound));
        }
    }
}


mod emitting_events {
    use super::*;

    #[test]
    fn emit_successful() {
        let mut emitter = EventManager::<TestStringPayload>::new(EventEmitter::default());
        let count = Arc::new(Mutex::new(0));
        let count_clone = Arc::clone(&count);
        {
            let emitter = Arc::get_mut(&mut emitter).unwrap();
            assert!(
                emitter.add_listener("count", Arc::new(move |payload| {
                    assert_eq!(payload.as_ref(), "Test");
                    *count_clone.lock().unwrap() += 1;
                })).is_ok(),
                "Failed to add event listener"
            );

            for _ in 0..10 {
                assert!(
                    emitter.emit("count", test_string_payload("Test")).is_ok(),
                    "Failed to emit event"
                );
            }
        }

        assert_eq!(*count.lock().unwrap(), 10, "Event callbacks unsuccessful");
    }

    #[test]
    fn limited_listener_emission_drop_off_successful() {
        let mut emitter = EventManager::new(EventEmitter::default());
        let count = Arc::new(Mutex::new(0));
        let count_clone = Arc::clone(&count);
        {
            let emitter = Arc::get_mut(&mut emitter).unwrap();
            assert!(
                emitter.add_limited_listener(
                    "count",
                    Arc::new(move |_| {
                        *count_clone.lock().unwrap() += 1;
                    }),
                    5
                ).is_ok(),
                "Failed to add event listener"
            );

            for _ in 0..5 {
                assert!(
                    emitter.emit("count", test_string_payload("Test")).is_ok(),
                    "Failed to emit event to limited listeners"
                );
            }
        }

        assert!(
            !emitter.has_listener("count"),
            "Failed to remove limited listener {}", emitter.listener_count("count")
        );
        assert_eq!(*count.lock().unwrap(), 5, "Event callbacks unsuccessful");
    }

    #[test]
    fn once_listener_emission_drop_off_successful() {
        let mut emitter = EventManager::new(EventEmitter::default());
        let count = Arc::new(Mutex::new(0));
        let count_clone = Arc::clone(&count);
        {
            let emitter = Arc::get_mut(&mut emitter).unwrap();
            assert!(
                emitter.add_once("count", Arc::new(move |_| {
                    *count_clone.lock().unwrap() += 1;
                })).is_ok(),
                "Failed to add once event listener"
            );

            assert!(
                emitter.emit("count", test_string_payload("Increment")).is_ok(),
                "Failed to emit event to once listeners"
            );
        }

        assert!(
            !emitter.has_listener("count"),
            "Failed to remove once listener: {}", emitter.listener_count("count")
        );
        assert_eq!(*count.lock().unwrap(), 1, "Event callbacks unsuccessful");
    }
}

