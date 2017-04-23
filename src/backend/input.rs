//! Common traits for input backends to receive input from.

use backend::{SeatInternal, TouchSlotInternal};

use std::error::Error;

/// A seat describes a group of input devices and at least one
/// graphics device belonging together.
///
/// By default only one seat exists for most systems and smithay backends
/// however multiseat configurations are possible and should be treated as
/// separated users, all with their own focus, input and cursor available.
///
/// Seats referring to the same internal id will always be equal and result in the same
/// hash, but capabilities of cloned and copied `Seat`s will not be updated by smithay.
/// Always referr to the `Seat` given by a callback for up-to-date information. You may
/// use this to calculate the differences since the last callback.
#[derive(Debug, Clone, Copy, Eq)]
pub struct Seat {
    id: u64,
    capabilities: SeatCapabilities,
}

impl SeatInternal for Seat {
    fn new(id: u64, capabilities: SeatCapabilities) -> Seat {
        Seat {
            id: id,
            capabilities: capabilities,
        }
    }

    fn capabilities_mut(&mut self) -> &mut SeatCapabilities {
        &mut self.capabilities
    }
}

impl Seat {
    /// Get the currently capabilities of this `Seat`
    pub fn capabilities(&self) -> &SeatCapabilities {
        &self.capabilities
    }
}

impl ::std::cmp::PartialEq for Seat {
    fn eq(&self, other: &Seat) -> bool {
        self.id == other.id
    }
}

impl ::std::hash::Hash for Seat {
    fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
        self.id.hash(state);
    }
}

/// Describes capabilities a `Seat` has.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SeatCapabilities {
    /// `Seat` has a pointer
    pub pointer: bool,
    /// `Seat` has a keyboard
    pub keyboard: bool,
    /// `Seat` has a touchscreen
    pub touch: bool,
}

/// Trait for generic functions every input event does provide/
pub trait Event {
    /// Returns an upward counting variable useful for event ordering.
    ///
    /// Makes no gurantees about actual time passed between events.
    // # TODO:
    // - check if events can even arrive out of order.
    // - Make stronger time guarantees, if possible
    fn time(&self) -> u32;
}

impl Event for () {
    fn time(&self) -> u32 {
        unreachable!()
    }
}

/// State of key on a keyboard. Either pressed or released
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KeyState {
    /// Key is released
    Released,
    /// Key is pressed
    Pressed,
}

/// Trait for keyboard event
pub trait KeyboardKeyEvent: Event {
    /// Code of the pressed key. See linux/input-event-codes.h
    fn key_code(&self) -> u32;
    /// State of the key
    fn state(&self) -> KeyState;
    /// Total number of keys pressed on all devices on the associated `Seat`
    fn count(&self) -> u32;
}

impl KeyboardKeyEvent for () {
    fn key_code(&self) -> u32 {
        unreachable!()
    }

    fn state(&self) -> KeyState {
        unreachable!()
    }

    fn count(&self) -> u32 {
        unreachable!()
    }
}

/// A particular mouse button
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Middle mouse button
    Middle,
    /// Right mouse button
    Right,
    /// Other mouse button with index
    Other(u8),
}

/// State of a button on a mouse. Either pressed or released
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MouseButtonState {
    /// Button is released
    Released,
    /// Button is pressed
    Pressed,
}

/// Common methods pointer event generated by pressed buttons do implement
pub trait PointerButtonEvent: Event {
    /// Pressed button of the event
    fn button(&self) -> MouseButton;
    /// State of the button
    fn state(&self) -> MouseButtonState;
}

impl PointerButtonEvent for () {
    fn button(&self) -> MouseButton {
        unreachable!()
    }

    fn state(&self) -> MouseButtonState {
        unreachable!()
    }
}

/// Axis when scrolling
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    /// Vertical axis
    Vertical,
    /// Horizonal axis
    Horizontal,
}

/// Source of an axis when scrolling
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AxisSource {
    /// Finger. Mostly used for trackpads.
    ///
    /// Guarantees that a scroll sequence is terminated with a scroll value of 0.
    /// A caller may use this information to decide on whether kinetic scrolling should
    /// be triggered on this scroll sequence.
    ///
    /// The coordinate system is identical to the
    /// cursor movement, i.e. a scroll value of 1 represents the equivalent relative
    /// motion of 1.
    Finger,
    /// Continous scrolling device. Almost identical to `Finger`
    ///
    /// No terminating event is guaranteed (though it may happen).
    ///
    /// The coordinate system is identical to
    /// the cursor movement, i.e. a scroll value of 1 represents the equivalent relative
    /// motion of 1.
    Continuous,
    /// Scroll wheel.
    ///
    /// No terminating event is guaranteed (though it may happen). Scrolling is in
    /// discrete steps. It is up to the caller how to interpret such different step sizes.
    Wheel,
    /// Scrolling through tilting the scroll wheel.
    ///
    /// No terminating event is guaranteed (though it may happen). Scrolling is in
    /// discrete steps. It is up to the caller how to interpret such different step sizes.
    WheelTilt,
}

/// Trait for pointer events generated by scrolling on an axis.
pub trait PointerAxisEvent: Event {
    /// `Axis` this event was generated for.
    fn axis(&self) -> Axis;
    /// Source of the scroll event. Important for interpretation of `amount`.
    fn source(&self) -> AxisSource;
    /// Amount of scrolling on the given `Axis`. See `source` for interpretation.
    fn amount(&self) -> f64;
}

impl PointerAxisEvent for () {
    fn axis(&self) -> Axis {
        unreachable!()
    }

    fn source(&self) -> AxisSource {
        unreachable!()
    }

    fn amount(&self) -> f64 {
        unreachable!()
    }
}

/// Trait for pointer events generated by relative device movement.
pub trait PointerMotionEvent: Event {
    /// Delta between the last and new pointer device position interpreted as pixel movement
    fn delta(&self) -> (u32, u32) {
        (self.delta_x(), self.delta_y())
    }

    /// Delta on the x axis between the last and new pointer device position interpreted as pixel movement
    fn delta_x(&self) -> u32;
    /// Delta on the y axis between the last and new pointer device position interpreted as pixel movement
    fn delta_y(&self) -> u32;
}

impl PointerMotionEvent for () {
    fn delta_x(&self) -> u32 {
        unreachable!()
    }

    fn delta_y(&self) -> u32 {
        unreachable!()
    }
}

/// Trait for pointer events generated by absolute device positioning.
pub trait PointerMotionAbsoluteEvent: Event {
    /// Device position in it's original coordinate space.
    ///
    /// The format is defined by the backend implementation.
    fn position(&self) -> (f64, f64) {
        (self.x(), self.y())
    }

    /// Device x position in it's original coordinate space.
    ///
    /// The format is defined by the backend implementation.
    fn x(&self) -> f64;

    /// Device y position in it's original coordinate space.
    ///
    /// The format is defined by the backend implementation.
    fn y(&self) -> f64;

    /// Device position converted to the targets coordinate space.
    /// E.g. the focused output's resolution.
    fn position_transformed(&self, coordinate_space: (u32, u32)) -> (u32, u32) {
        (self.x_transformed(coordinate_space.0), self.y_transformed(coordinate_space.1))
    }

    /// Device x position converted to the targets coordinate space's width.
    /// E.g. the focused output's width.
    fn x_transformed(&self, width: u32) -> u32;

    /// Device y position converted to the targets coordinate space's height.
    /// E.g. the focused output's height.
    fn y_transformed(&self, height: u32) -> u32;
}

impl PointerMotionAbsoluteEvent for () {
    fn x(&self) -> f64 {
        unreachable!()
    }

    fn y(&self) -> f64 {
        unreachable!()
    }

    fn x_transformed(&self, _width: u32) -> u32 {
        unreachable!()
    }

    fn y_transformed(&self, _height: u32) -> u32 {
        unreachable!()
    }
}

/// Slot of a different touch event.
///
/// Touch events are groubed by slots, usually to identify different
/// fingers on a multi-touch enabled input device. Events should only
/// be interpreted in the context of other events on the same slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TouchSlot {
    id: u64,
}

impl TouchSlotInternal for TouchSlot {
    fn new(id: u64) -> Self {
        TouchSlot { id: id }
    }
}

/// Trait for touch events starting at a given position.
pub trait TouchDownEvent: Event {
    /// `TouchSlot`, if the device has multi-touch capabilities
    fn slot(&self) -> Option<TouchSlot>;

    /// Touch position in the device's native coordinate space
    ///
    /// The actual format is defined by the implementation.
    fn position(&self) -> (f64, f64) {
        (self.x(), self.y())
    }

    /// Touch position converted into the target coordinate space.
    /// E.g. the focused output's resolution.
    fn position_transformed(&self, coordinate_space: (u32, u32)) -> (u32, u32) {
        (self.x_transformed(coordinate_space.0), self.y_transformed(coordinate_space.1))
    }

    /// Touch event's x-coordinate in the device's native coordinate space
    ///
    /// The actual format is defined by the implementation.
    fn x(&self) -> f64;

    /// Touch event's x-coordinate in the device's native coordinate space
    ///
    /// The actual format is defined by the implementation.
    fn y(&self) -> f64;

    /// Touch event's x position converted to the targets coordinate space's width.
    /// E.g. the focused output's width.
    fn x_transformed(&self, width: u32) -> u32;

    /// Touch event's y position converted to the targets coordinate space's width.
    /// E.g. the focused output's width.
    fn y_transformed(&self, height: u32) -> u32;
}

impl TouchDownEvent for () {
    fn slot(&self) -> Option<TouchSlot> {
        unreachable!()
    }

    fn x(&self) -> f64 {
        unreachable!()
    }

    fn y(&self) -> f64 {
        unreachable!()
    }

    fn x_transformed(&self, _width: u32) -> u32 {
        unreachable!()
    }

    fn y_transformed(&self, _height: u32) -> u32 {
        unreachable!()
    }
}

/// Trait for touch events regarding movement on the screen
pub trait TouchMotionEvent: Event {
    /// `TouchSlot`, if the device has multi-touch capabilities
    fn slot(&self) -> Option<TouchSlot>;

    /// Touch position in the device's native coordinate space
    ///
    /// The actual format is defined by the implementation.
    fn position(&self) -> (f64, f64) {
        (self.x(), self.y())
    }

    /// Touch position converted into the target coordinate space.
    /// E.g. the focused output's resolution.
    fn position_transformed(&self, coordinate_space: (u32, u32)) -> (u32, u32) {
        (self.x_transformed(coordinate_space.0), self.y_transformed(coordinate_space.1))
    }

    /// Touch event's x-coordinate in the device's native coordinate space
    ///
    /// The actual format is defined by the implementation.
    fn x(&self) -> f64;

    /// Touch event's x-coordinate in the device's native coordinate space
    ///
    /// The actual format is defined by the implementation.
    fn y(&self) -> f64;

    /// Touch event's x position converted to the targets coordinate space's width.
    /// E.g. the focused output's width.
    fn x_transformed(&self, width: u32) -> u32;

    /// Touch event's y position converted to the targets coordinate space's width.
    /// E.g. the focused output's width.
    fn y_transformed(&self, height: u32) -> u32;
}

impl TouchMotionEvent for () {
    fn slot(&self) -> Option<TouchSlot> {
        unreachable!()
    }

    fn x(&self) -> f64 {
        unreachable!()
    }

    fn y(&self) -> f64 {
        unreachable!()
    }

    fn x_transformed(&self, _width: u32) -> u32 {
        unreachable!()
    }

    fn y_transformed(&self, _height: u32) -> u32 {
        unreachable!()
    }
}

/// Trait for touch events finishing.
pub trait TouchUpEvent: Event {
    /// `TouchSlot`, if the device has multi-touch capabilities
    fn slot(&self) -> Option<TouchSlot>;
}

impl TouchUpEvent for () {
    fn slot(&self) -> Option<TouchSlot> {
        unreachable!()
    }
}

/// Trait for touch events cancelling the chain
pub trait TouchCancelEvent: Event {
    /// `TouchSlot`, if the device has multi-touch capabilities
    fn slot(&self) -> Option<TouchSlot>;
}

impl TouchCancelEvent for () {
    fn slot(&self) -> Option<TouchSlot> {
        unreachable!()
    }
}

/// Trait for touch frame events
pub trait TouchFrameEvent: Event {}

impl TouchFrameEvent for () {}

/// Trait that describes objects providing a source of input events. All input backends
/// need to implemenent this and provide the same base gurantees about the presicion of
/// given events.
pub trait InputBackend: Sized {
    /// Type of input device associated with the backend
    type InputConfig: ?Sized;

    /// Type representing errors that may be returned when processing events
    type EventError: Error;

    /// Type representing keyboard events
    type KeyboardKeyEvent: KeyboardKeyEvent;
    /// Type representing axis events on pointer devices
    type PointerAxisEvent: PointerAxisEvent;
    /// Type representing button events on pointer devices
    type PointerButtonEvent: PointerButtonEvent;
    /// Type representing motion events of pointer devices
    type PointerMotionEvent: PointerMotionEvent;
    /// Type representing motion events of pointer devices
    type PointerMotionAbsoluteEvent: PointerMotionAbsoluteEvent;
    /// Type representing touch events starting
    type TouchDownEvent: TouchDownEvent;
    /// Type representing touch events ending
    type TouchUpEvent: TouchUpEvent;
    /// Type representing touch events from moving
    type TouchMotionEvent: TouchMotionEvent;
    /// Type representing cancelling of touch events
    type TouchCancelEvent: TouchCancelEvent;
    /// Type representing touch frame events
    type TouchFrameEvent: TouchFrameEvent;

    /// Sets a new handler for this `InputBackend`
    fn set_handler<H: InputHandler<Self> + 'static>(&mut self, handler: H);
    /// Get a reference to the currently set handler, if any
    fn get_handler(&mut self) -> Option<&mut InputHandler<Self>>;
    /// Clears the currently handler, if one is set
    fn clear_handler(&mut self);

    /// Get current `InputConfig`
    fn input_config(&mut self) -> &mut Self::InputConfig;

    /// Processes new events of the underlying backend and drives the `InputHandler`.
    fn dispatch_new_events(&mut self) -> Result<(), Self::EventError>;
}

/// Implement to receive input events from any `InputBackend`.
pub trait InputHandler<B: InputBackend> {
    /// Called when a new `Seat` has been created
    fn on_seat_created(&mut self, seat: &Seat);
    /// Called when an existing `Seat` has been destroyed.
    fn on_seat_destroyed(&mut self, seat: &Seat);
    /// Called when a `Seat`'s properties have changed.
    ///
    /// ## Note:
    ///
    /// It is not guaranteed that any change has actually happened.
    fn on_seat_changed(&mut self, seat: &Seat);

    /// Called when a new keyboard event was received.
    ///
    /// # Arguments
    ///
    /// - `seat` - The `Seat` the event belongs to
    /// - `event` - The keyboard event
    ///
    fn on_keyboard_key(&mut self, seat: &Seat, event: B::KeyboardKeyEvent);

    /// Called when a new pointer movement event was received.
    ///
    /// # Arguments
    ///
    /// - `seat` - The `Seat` the event belongs to
    /// - `event` - The pointer movement event
    fn on_pointer_move(&mut self, seat: &Seat, event: B::PointerMotionEvent);
    /// Called when a new pointer absolute movement event was received.
    ///
    /// # Arguments
    ///
    /// - `seat` - The `Seat` the event belongs to
    /// - `event` - The pointer absolute movement event
    fn on_pointer_move_absolute(&mut self, seat: &Seat, event: B::PointerMotionAbsoluteEvent);
    /// Called when a new pointer button event was received.
    ///
    /// # Arguments
    ///
    /// - `seat` - The `Seat` the event belongs to
    /// - `event` - The pointer button event
    fn on_pointer_button(&mut self, seat: &Seat, event: B::PointerButtonEvent);
    /// Called when a new pointer scroll event was received.
    ///
    /// # Arguments
    ///
    /// - `seat` - The `Seat` the event belongs to
    /// - `event` - A upward counting variable useful for event ordering. Makes no gurantees about actual time passed between events.
    fn on_pointer_axis(&mut self, seat: &Seat, event: B::PointerAxisEvent);

    /// Called when a new touch down event was received.
    ///
    /// # Arguments
    ///
    /// - `seat` - The `Seat` the event belongs to
    /// - `event` - The touch down event
    fn on_touch_down(&mut self, seat: &Seat, event: B::TouchDownEvent);
    /// Called when a new touch motion event was received.
    ///
    /// # Arguments
    ///
    /// - `seat` - The `Seat` the event belongs to
    /// - `event` - The touch motion event.
    fn on_touch_motion(&mut self, seat: &Seat, event: B::TouchMotionEvent);
    /// Called when a new touch up event was received.
    ///
    /// # Arguments
    ///
    /// - `seat` - The `Seat` the event belongs to
    /// - `event` - The touch up event.
    fn on_touch_up(&mut self, seat: &Seat, event: B::TouchUpEvent);
    /// Called when a new touch cancel event was received.
    ///
    /// # Arguments
    ///
    /// - `seat` - The `Seat` the event belongs to
    /// - `event` - The touch cancel event.
    fn on_touch_cancel(&mut self, seat: &Seat, event: B::TouchCancelEvent);
    /// Called when a new touch frame event was received.
    ///
    /// # Arguments
    ///
    /// - `seat` - The `Seat` the event belongs to
    /// - `event` - The touch frame event.
    fn on_touch_frame(&mut self, seat: &Seat, event: B::TouchFrameEvent);

    /// Called when the `InputConfig` was changed through an external event.
    ///
    /// What kind of events can trigger this call is completely backend dependent.
    /// E.g. an input devices was attached/detached or changed it's own configuration.
    fn on_input_config_changed(&mut self, config: &mut B::InputConfig);
}

impl<B: InputBackend> InputHandler<B> for Box<InputHandler<B>> {
    fn on_seat_created(&mut self, seat: &Seat) {
        (**self).on_seat_created(seat)
    }

    fn on_seat_destroyed(&mut self, seat: &Seat) {
        (**self).on_seat_destroyed(seat)
    }

    fn on_seat_changed(&mut self, seat: &Seat) {
        (**self).on_seat_changed(seat)
    }

    fn on_keyboard_key(&mut self, seat: &Seat, event: B::KeyboardKeyEvent) {
        (**self).on_keyboard_key(seat, event)
    }

    fn on_pointer_move(&mut self, seat: &Seat, event: B::PointerMotionEvent) {
        (**self).on_pointer_move(seat, event)
    }

    fn on_pointer_move_absolute(&mut self, seat: &Seat, event: B::PointerMotionAbsoluteEvent) {
        (**self).on_pointer_move_absolute(seat, event)
    }

    fn on_pointer_button(&mut self, seat: &Seat, event: B::PointerButtonEvent) {
        (**self).on_pointer_button(seat, event)
    }

    fn on_pointer_axis(&mut self, seat: &Seat, event: B::PointerAxisEvent) {
        (**self).on_pointer_axis(seat, event)
    }

    fn on_touch_down(&mut self, seat: &Seat, event: B::TouchDownEvent) {
        (**self).on_touch_down(seat, event)
    }

    fn on_touch_motion(&mut self, seat: &Seat, event: B::TouchMotionEvent) {
        (**self).on_touch_motion(seat, event)
    }

    fn on_touch_up(&mut self, seat: &Seat, event: B::TouchUpEvent) {
        (**self).on_touch_up(seat, event)
    }

    fn on_touch_cancel(&mut self, seat: &Seat, event: B::TouchCancelEvent) {
        (**self).on_touch_cancel(seat, event)
    }

    fn on_touch_frame(&mut self, seat: &Seat, event: B::TouchFrameEvent) {
        (**self).on_touch_frame(seat, event)
    }

    fn on_input_config_changed(&mut self, config: &mut B::InputConfig) {
        (**self).on_input_config_changed(config)
    }
}
