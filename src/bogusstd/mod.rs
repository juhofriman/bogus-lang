use crate::ast::scope::Scope;
use crate::bogusstd::io::io_functions;

mod io;

pub fn prepare_scope(scope: &mut Scope) {
    for f in io_functions() {
        scope.store(f.name, f.value);
    }
}