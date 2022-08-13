use anyhow::{anyhow, Result};

pub(crate) fn fill_array_unchecked<const N: usize, T>(
    mut hydrate: [T; N],
    it: impl Iterator<Item = T>,
) -> [T; N] {
    hydrate.iter_mut().zip(it).for_each(|(i, v)| *i = v);
    hydrate
}

pub(crate) fn new_arr<const N: usize, T>(mut it: impl Iterator<Item = T>) -> Result<[T; N]> {
    // Branch here depending on whether T needs to be dropped.
    // If it does, we track the amount of cells we've initialised,
    // and impl Drop on this, to ensure that our progress is rolled back
    let mut array: std::mem::MaybeUninit<[T; N]> = std::mem::MaybeUninit::uninit();

    let mut ptr_i = array.as_mut_ptr() as *mut T;

    if std::mem::needs_drop::<T>() {
        //  Safety: The array length is known at compile time, it suffices to
        //      track counts and assert the pointer arithmetic stays between
        //      0 and N - 1
        unsafe {
            // Count as we go along;
            // if the iterator stops before N or finishes after we'll return an Err.
            let mut c = 0;
            loop {
                match it.next() {
                    Some(v) if c <= N - 1 => {
                        ptr_i.write(v);
                        ptr_i = ptr_i.add(1);
                    }
                    None if c <= N - 1 => {
                        return Err(anyhow!(
                            "Specified constructor terminated in {} steps, expected {}",
                            c,
                            N
                        ));
                    }
                    Some(_) => {
                        return Err(anyhow!("Specified constructor is longer than {} steps", N));
                    }
                    _ => break,
                }
                c += 1;
            }
            Ok(array.assume_init())
        }
    } else {
        struct RollbackGuard<T> {
            base_ptr: *mut T,
            init_count: usize,
        }

        impl<T> Drop for RollbackGuard<T> {
            fn drop(self: &'_ mut Self) {
                // Safety: The init_count is only incremented after each successful
                // pointer write.
                unsafe {
                    std::ptr::drop_in_place(std::slice::from_raw_parts_mut(
                        self.base_ptr,
                        self.init_count,
                    ));
                }
            }
        }

        unsafe {
            let mut panic_guard = RollbackGuard {
                base_ptr: ptr_i,
                init_count: 0,
            };

            let mut c = 0;
            loop {
                match it.next() {
                    Some(v) if c <= N - 1 => {
                        ptr_i.write(v);
                        ptr_i = ptr_i.add(1);
                        panic_guard.init_count += 1;
                    }
                    None if c <= N - 1 => {
                        return Err(anyhow!(
                            "Specified constructor terminated after {} elements , expected {}",
                            c,
                            N
                        ));
                    }
                    Some(_) => {
                        return Err(anyhow!(
                            "Specified constructor is longer than {} elements",
                            N
                        ));
                    }
                    _ => {
                        // leak the panic guard; the array can own the initialised memory
                        std::mem::forget(panic_guard);
                        break;
                    }
                }
                c += 1;
            }

            Ok(array.assume_init())
        }
    }
}
