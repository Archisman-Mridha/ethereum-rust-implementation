use tokio::sync::mpsc::UnboundedSender;

pub struct EventEmitters<T> {
  emitters: Vec<UnboundedSender<T>>
}

impl<T: Clone> EventEmitters<T> {
  // Makes all the emitters emit the given event.
  pub fn emit(&mut self, event: T) {
    self.emitters.retain(|emitter| emitter.send(event.clone( )).is_ok( ))
  }
}