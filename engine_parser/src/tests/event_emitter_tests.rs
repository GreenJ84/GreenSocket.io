use std::sync::{Arc};

use crate::{
    EventEmitter,
    EventHandler,
    EventError,
    Listener,
};

fn sample_listener(message: &str) {
    println!("Received: {}", message);
}

#[cfg(test)]
mod listener_enrollment_management {
    use super::*;


    #[test]
    fn add_listener_functions() {
        let mut event_manager = EventEmitter::new();
        let callback: Listener = Arc::new(sample_listener);

        assert!(event_manager.add_listener("test_event", callback).is_ok());
        assert_eq!(event_manager.listener_count("test_event").unwrap_or(0), 1);
    }

    #[test]
    fn on_function_correctly_relays() {
        let mut event_manager = EventEmitter::new();
        let callback: Listener = Arc::new(sample_listener);

        assert!(event_manager.on("test_event", callback).is_ok());
        assert_eq!(event_manager.listener_count("test_event").unwrap_or(0), 1);
    }

    #[test]
    fn remove_listener_functions() {
        let mut event_manager = EventEmitter::new();
        let callback: Listener = Arc::new(sample_listener);
        assert!(event_manager.add_listener("test_event", Arc::clone(&callback)).is_ok(), "Adding new listener failed");

        assert!(event_manager.remove_listener("test_event", callback).is_ok());
        assert_eq!(event_manager.listener_count("test_event").unwrap_or(12), 0);
    }

    #[test]
    fn off_function_correctly_relays() {
        let mut event_manager = EventEmitter::new();
        let callback: Listener = Arc::new(sample_listener);
        assert!(event_manager.on("test_event", Arc::clone(&callback)).is_ok(), "Adding new listener failed");

        assert!(event_manager.off("test_event", callback).is_ok());
        assert_eq!(event_manager.listener_count("test_event").unwrap_or(12), 0);
    }


    #[test]
    fn remove_all_listeners_functions() {
        let mut event_manager = EventEmitter::new();
        let callback: Listener = Arc::new(sample_listener);
        assert!(event_manager.add_listener("test_event", Arc::clone(&callback)).is_ok(), "Adding listener 1 failed");
        assert!(event_manager.add_listener("test_event", Arc::clone(&callback)).is_ok(), "Adding listener 2 failed");
        assert!(event_manager.add_listener("test_event", Arc::clone(&callback)).is_ok(), "Adding listener 3 failed");

        assert!(event_manager.remove_all_listeners("test_event").is_ok());
        assert_eq!(event_manager.listener_count("test_event").unwrap_or(12), 0);
    }

    #[test]
    fn remove_listener_from_nonexistent_event() {
        let mut event_manager = EventEmitter::new();
        let callback: Listener = Arc::new(sample_listener);

        assert!(matches!(
            event_manager.remove_listener("test_event", callback),
            Err(EventError::EventNotFound)
        ));
    }

    #[test]
    fn remove_all_listeners_from_nonexistent_event() {
        let mut event_manager = EventEmitter::new();

        assert!(matches!(
            event_manager.remove_all_listeners("test_event"),
            Err(EventError::EventNotFound)
        ));
    }

    #[test]
    fn remove_nonexistent_listener_from_event() {
        let mut event_manager = EventEmitter::new();
        let callback: Listener = Arc::new(sample_listener);
        assert!(event_manager.add_listener("test_event", Arc::clone(&callback)).is_ok(), "Adding listener 1 failed");
        assert!(event_manager.add_listener("test_event", Arc::clone(&callback)).is_ok(), "Adding listener 2 failed");

        assert!(matches!(
            event_manager.remove_listener("test_event", Arc::new( |_s: &str|{} )),
            Err(EventError::ListenerNotFound)
        ));
    }
}

#[cfg(test)]
mod listener_maximum_management {
    use super::*;

    #[test]
    fn listener_max_updates() {
        let mut event_manager = EventEmitter::new();
        event_manager.set_max_listeners(2);
        assert_eq!(event_manager.max_listeners(), 2);
        event_manager.set_max_listeners(0);
        assert_eq!(event_manager.max_listeners(), 0);
        event_manager.set_max_listeners(100usize);
        assert_eq!(event_manager.max_listeners(), 100usize);
        event_manager.set_max_listeners(1_000_000usize);
        assert_eq!(event_manager.max_listeners(), 1_000_000usize);
    }

    #[test]
    fn listener_max_is_enforced() {
        let mut event_manager = EventEmitter::new();
        let callback: Listener = Arc::new(sample_listener);

        let max = event_manager.max_listeners();
        for _ in 0..max{
            assert!(event_manager.add_listener("test_event", Arc::clone(&callback)).is_ok());
        }
        assert_eq!(event_manager.listener_count("test_event").unwrap_or(0), max);
        assert!(matches!(
            event_manager.add_listener("test_event", Arc::clone(&callback)),
            Err(EventError::MaxedEventListeners)
        ));
    }
}

#[cfg(test)]
mod emitting_events {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn emitting_events_works() {
        let mut event_manager = EventEmitter::new();
        let received = Arc::new(Mutex::new(String::new()));
        let received_clone = Arc::clone(&received);

        let callback: Listener = Arc::new(move |msg| {
            let mut r = received_clone.lock().unwrap();
            *r = msg.to_string();
        });

        assert!(event_manager.add_listener("test_event", Arc::clone(&callback)).is_ok());
        event_manager.emit("test_event", "Hello, world!");

        let result = received.lock().unwrap().clone();
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn events_emit_to_all_listeners() {
        let mut event_manager = EventEmitter::new();
        let received = Arc::new(Mutex::new(0));
        let received_clone = Arc::clone(&received);

        let callback: Listener = Arc::new(move |msg| {
            assert_eq!(msg, "Hello, world!");
            let mut r = received_clone.lock().unwrap();
            *r += 1;
        });

        for _ in 0..10{
            assert!(event_manager.add_listener("test_event", Arc::clone(&callback)).is_ok());
        }
        event_manager.emit("test_event", "Hello, world!");

        let result = received.lock().unwrap().clone();
        assert_eq!(result, 10, "Not all listeners received the event");
    }
}


#[test]
fn test_event_names() {
    let mut event_manager = EventEmitter::new();
    let callback: Listener = Arc::new(sample_listener);

    let mut registered = Vec::<String>::new();
    for i in 1..=10{
        let name = format!("event{}", i);
        registered.push(name.clone());
        assert!(event_manager.add_listener(&name , Arc::clone(&callback)).is_ok());
    }

    let mut names = event_manager.event_names();
    names.sort();
    registered.sort();
    assert_eq!(names, registered);
}
