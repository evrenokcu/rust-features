use crate::print_header;

mod arc_mutex;
mod cell;
mod life_time;
mod linked_list;
mod rc;
mod ref_cell;
mod rw_lock;
mod send_sync;
mod struct_with_str_ref;
mod thread_channel_mutex;
pub(crate) fn concurrency() {
    print_header("concurrency");
    struct_with_str_ref::generics();
    arc_mutex::arc_mutex();
    send_sync::send_sync();
    life_time::life_time();
    thread_channel_mutex::thread_channel_mutex();
    rc::execute();
    ref_cell::execute();
    linked_list::execute();
    cell::execute();
    rw_lock::execute();
}
