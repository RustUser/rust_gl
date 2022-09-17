use crate::{Slider, UIElement};

pub type OnSliderValueChanged = dyn FnMut(&mut Slider, f32, f32);

///Defined by the Component that is being dragged, the mouse_delta(the difference in mouse position since last position), the Offset(the distance from the position of the object at the start of the drag), and then the Reference Mouse position.
pub type OnDrag = dyn FnMut(&mut Box<dyn UIElement>, [f64; 2], [f64; 2], [f64; 2]);