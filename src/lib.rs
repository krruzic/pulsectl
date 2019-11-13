use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

extern crate libpulse_binding as pulse;

use pulse::{
    context::{introspect, Context},
    mainloop::standard::{IterateResult, Mainloop},
    operation::{Operation, State},
    proplist::Proplist,
};

pub mod controllers;
pub mod types;

pub struct Handler {
    pub mainloop: Rc<RefCell<Mainloop>>,
    pub context: Rc<RefCell<Context>>,
    pub introspect: introspect::Introspector,
}
impl Handler {
    pub fn connect(name: &str) -> Result<Handler, ()> {
        let mut proplist = Proplist::new().unwrap();
        proplist
            .set_str(pulse::proplist::properties::APPLICATION_NAME, name)
            .unwrap();

        let mainloop = Rc::new(RefCell::new(
            Mainloop::new().expect("Failed to create mainloop"),
        ));

        let context = Rc::new(RefCell::new(
            Context::new_with_proplist(mainloop.borrow().deref(), "MainConn", &proplist)
                .expect("Failed to create new context"),
        ));

        context
            .borrow_mut()
            .connect(None, pulse::context::flags::NOFLAGS, None)
            .expect("Failed to connect context");

        loop {
            match mainloop.borrow_mut().iterate(false) {
                IterateResult::Quit(_) | IterateResult::Err(_) => {
                    eprintln!("iterate state was not success, quitting...");
                    return Err(());
                }
                IterateResult::Success(_) => {}
            }

            match context.borrow().get_state() {
                pulse::context::State::Ready => break,
                pulse::context::State::Failed | pulse::context::State::Terminated => {
                    eprintln!("context state failed/terminated, quitting...");
                    return Err(());
                }
                _ => {}
            }
        }

        let introspect = context.borrow_mut().introspect();
        Ok(Handler {
            mainloop,
            context,
            introspect,
        })
    }

    // loop until the passed operation is completed
    pub fn wait_for_operation<G: ?Sized>(&mut self, op: Operation<G>) -> Result<(), ()> {
        loop {
            match self.mainloop.borrow_mut().iterate(false) {
                IterateResult::Quit(_) | IterateResult::Err(_) => return Err(()),
                IterateResult::Success(_) => {}
            }
            match op.get_state() {
                State::Done => {
                    break;
                }
                State::Running => {}
                State::Cancelled => return Err(()),
            }
        }
        Ok(())
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        self.context.borrow_mut().disconnect();
        self.mainloop.borrow_mut().quit(pulse::def::Retval(0));
    }
}
