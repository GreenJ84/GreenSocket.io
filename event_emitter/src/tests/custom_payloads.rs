use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

use crate::{EventManager, EventEmitter, EventHandler, EventPayload};

mod primitive_data_payloads {
    use super::*;

    ///! Base Event Emitter testing done with a String Payload

    mod int_payload {
        use super::*;

        type TestIntPayload = u32;
        fn test_int_payload(data: u32) -> EventPayload<TestIntPayload> {
            Arc::new(data)
        }

        #[test]
        fn int_emit_successful() {
            let mut emitter = EventManager::<TestIntPayload>::new(EventEmitter::default());
            let count = Arc::new(Mutex::new(0));
            {
                let emitter = Arc::get_mut(&mut emitter).unwrap();
                let count_clone = Arc::clone(&count);
                assert!(
                    emitter.add_listener("emit", Arc::new(move |payload| {
                        assert_eq!(*payload.as_ref(), 42);
                        *count_clone.lock().unwrap() += 1;
                    })).is_ok(),
                    "Failed to add event listener"
                );

                emitter.emit("emit", test_int_payload(42)).unwrap();
            }

            assert_eq!(*count.lock().unwrap(), 1, "Event callbacks unsuccessful");
        }

        #[tokio::test]
        async fn int_async_emission_successful() {
            let mut emitter = EventManager::<TestIntPayload>::new(EventEmitter::default());
            let count = Arc::new(Mutex::new(0));
            {
                let emitter = Arc::get_mut(&mut emitter).unwrap();
                let count_clone = Arc::clone(&count);
                assert!(
                    emitter.add_listener("async_event", Arc::new(move |payload| {
                        assert_eq!(*payload.as_ref(), 24);
                        *count_clone.lock().unwrap() += 1;
                    })).is_ok(),
                    "Failed to add event listener"
                );

                for _ in 0..10 {
                    assert!(
                        emitter.emit_async("async_event", test_int_payload(24), false).is_ok()
                    );
                    sleep(Duration::from_millis(100)).await;
                }
            }

            assert_eq!(*count.lock().unwrap(), 10, "Async event callbacks unsuccessful");
        }
    }

    mod bool_payload {
        use super::*;

        type TestBoolPayload = bool;
        fn test_bool_payload(data: bool) -> EventPayload<TestBoolPayload> {
            Arc::new(data)
        }

        #[test]
        fn bool_emit_successful() {
            let mut emitter = EventManager::<TestBoolPayload>::new(EventEmitter::default());
            let count = Arc::new(Mutex::new(0));
            {
                let emitter = Arc::get_mut(&mut emitter).unwrap();
                let count_clone = Arc::clone(&count);
                assert!(
                    emitter.add_listener("emit", Arc::new(move |payload| {
                        assert_eq!(*payload.as_ref(), true);
                        *count_clone.lock().unwrap() += 1;
                    })).is_ok(),
                    "Failed to add event listener"
                );

                emitter.emit("emit", test_bool_payload(true)).unwrap();
            }

            assert_eq!(*count.lock().unwrap(), 1, "Event callbacks unsuccessful");
        }

        #[tokio::test]
        async fn bool_async_emission_successful() {
            let mut emitter = EventManager::<TestBoolPayload>::new(EventEmitter::default());
            let count = Arc::new(Mutex::new(0));
            {
                let emitter = Arc::get_mut(&mut emitter).unwrap();
                let count_clone = Arc::clone(&count);
                assert!(
                    emitter.add_listener("async_event", Arc::new(move |payload| {
                        assert_eq!(*payload.as_ref(), false);
                        *count_clone.lock().unwrap() += 1;
                    })).is_ok(),
                    "Failed to add event listener"
                );

                for _ in 0..10 {
                    assert!(
                        emitter.emit_async("async_event", test_bool_payload(false), false).is_ok()
                    );
                    sleep(Duration::from_millis(100)).await;
                }
            }

            assert_eq!(*count.lock().unwrap(), 10, "Async event callbacks unsuccessful");
        }
    }
}
