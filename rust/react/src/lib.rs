#[allow(unused_variables)]

// Because these are passed without & to some functions,
// it will probably be necessary for these two types to be Copy.
pub type CellID = (usize, usize);
pub type CallbackID = (usize, usize, usize);

enum Cell<'a,T> {
    Input {
        value: T,
    },
    Compute {
        value: T,
        dependencies: Vec<CellID>,
        compute_func: Box<Fn(&[T]) -> T + 'a>,
        callbacks: Vec<Option<Box<FnMut(T) -> () + 'a>>> // TODO: find out why Box is required
    }
}

static mut NEW_REACTOR_ID: usize = 0;

pub struct Reactor<'a, T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    reactor_id: usize,
    cells: Vec<Cell<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl <'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        unsafe {
            let id = NEW_REACTOR_ID;
            NEW_REACTOR_ID += 1;
            Self { reactor_id: id, cells: vec![] }
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        self.cells.push(Cell::Input { value: initial });
        (self.reactor_id, self.cells.len()-1)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // Return an Err (and you can change the error type) if any dependency doesn't exist.
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(&mut self, dependencies: &[CellID], compute_func: F) -> Result<CellID, ()> { // TODO: added where F: 'static, how to avoid this?
        if dependencies.iter().any(|d| !self.cell_exists(*d)) {
            Err(())
        } else {
            let value = compute_func(&self.get_args(dependencies));
            self.cells.push(Cell::Compute {
                value: value,
                dependencies: dependencies.iter().cloned().collect(),
                compute_func: Box::new(compute_func),
                callbacks: vec![]
            });
            Ok((self.reactor_id, self.cells.len()-1))
        }
    }

    fn get_args(&self, cell_ids: &[CellID]) -> Vec<T> {
        cell_ids.iter()
            .map(|&(_, cell_id)| {
                match self.cells[cell_id] {
                    Cell::Input { value } => value,
                    Cell::Compute { ref dependencies, ref compute_func, .. } => 
                        compute_func(&self.get_args(dependencies)),
                }
            }).collect()
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        if self.cell_exists(id) {
            Some(match self.cells[id.1] {
                Cell::Input { value, .. } => value,
                Cell::Compute { value, .. } => value
            })
        } else {
            None
        }
    }

    fn cell_exists(&self, id: CellID) -> bool {
        let (reactor_id, cell_id) = id;
        reactor_id == self.reactor_id || cell_id < self.cells.len()
    }

    // Sets the value of the specified input cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist, or the
    // specified cell is a compute cell, since compute cells cannot have their values directly set.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: CellID, new_value: T) -> Result<(), ()> {
        if self.cell_exists(id) {
            match self.cells[id.1] {
                Cell::Input { ref mut value, .. } => {
                    *value = new_value;
                },
                Cell::Compute { .. } => return Err(())
            }
        } else {
            return Err(())
        }
        Ok(self.update_compute_cells())
    }

    fn update_compute_cells(&mut self) {
        let mut new_values = vec![];
        for (i, cell) in self.cells.iter().enumerate() {
            if let &Cell::Compute { ref dependencies, ref compute_func, .. } = cell {
                new_values.push((i, compute_func(&self.get_args(&dependencies))));
            }
        }

        for (i, new_val) in new_values {
            if let Cell::Compute { ref mut value, ref mut callbacks, .. } = self.cells[i] {
                if value != &new_val {
                    for c in callbacks {
                        if let &mut Some(ref mut c) = c {
                            c(new_val);
                        }
                    }
                    *value = new_val;
                }
            }
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) -> () + 'a>(&mut self, id: CellID, callback: F) -> Result<CallbackID, ()> {
        if self.cell_exists(id) {
            match self.cells[id.1] {
                Cell::Compute { ref mut callbacks, .. } => {
                    callbacks.push(Some(Box::new(callback)));
                    Ok((id.0, id.1, callbacks.len()-1))
                },
                _ => Err(())
            }
        } else {
            Err(())
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Return an Err (and you can change the error type) if either the cell or callback
    // does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(&mut self, cell: CellID, callback: CallbackID) -> Result<(), ()> {
        if !self.cell_exists(cell) || (callback.0, callback.1) != cell {
            return Err(())
        }

        match self.cells[cell.1] {
            Cell::Compute { ref mut callbacks, .. } => {
                if callback.2 >= callbacks.len() || callbacks[callback.2].is_none() {
                    Err(())
                } else {
                    Ok(callbacks[callback.2] = None)
                }
            },
            _ => Err(())
        }
    }
}
