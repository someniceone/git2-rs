use crate::util::Binding;
use crate::{raw, IndexEntry, MergeFileResult};
use libc::size_t;

pub struct Conflicts {
    raw: *mut raw::git_merge_conflicts,
}

#[allow(missing_docs)]
pub struct MergeDiff {
    pub dtype: raw::git_merge_diff_t,
    pub ancestor_entry: Option<IndexEntry>,
    pub our_entry: Option<IndexEntry>,
    pub our_status: raw::git_delta_t,
    pub their_entry: Option<IndexEntry>,
    pub their_status: raw::git_delta_t,
    pub merge_result: MergeFileResult,
}

impl Conflicts {
    pub fn len(&self) -> usize {
        unsafe { raw::git_merge_conflicts_count(&*self.raw) as usize }
    }

    pub fn get(&self, n: usize) -> Option<MergeDiff> {
        unsafe {
            let ptr = raw::git_merge_diff_get_by_conflicts(self.raw, n as size_t);
            if ptr.is_null() {
                None
            } else {
                Some(Binding::from_raw(*ptr))
            }
        }
    }
}

impl Binding for Conflicts {
    type Raw = *mut raw::git_merge_conflicts;
    unsafe fn from_raw(raw: *mut raw::git_merge_conflicts) -> Conflicts {
        Conflicts { raw: raw }
    }
    fn raw(&self) -> *mut raw::git_merge_conflicts {
        self.raw
    }
}

impl Drop for Conflicts {
    fn drop(&mut self) {
        unsafe { raw::git_merge_conflicts_free(self.raw) }
    }
}

impl Binding for MergeDiff {
    type Raw = raw::git_merge_diff;

    unsafe fn from_raw(raw: raw::git_merge_diff) -> MergeDiff {
        let raw::git_merge_diff {
            dtype,
            ancestor_entry,
            our_entry,
            our_status,
            their_entry,
            their_status,
            merge_result,
        } = raw;

        let ancestor: Option<IndexEntry>;
        if ancestor_entry.path.is_null() {
            ancestor = None;
        } else {
            ancestor = Some(Binding::from_raw(ancestor_entry));
        }

        let our: Option<IndexEntry>;
        if our_entry.path.is_null() {
            our = None;
        } else {
            our = Some(Binding::from_raw(our_entry));
        }

        let their: Option<IndexEntry>;
        if their_entry.path.is_null() {
            their = None;
        } else {
            their = Some(Binding::from_raw(their_entry));
        }

        MergeDiff {
            dtype: dtype,
            ancestor_entry: ancestor,
            our_entry: our,
            our_status,
            their_entry: their,
            their_status,
            merge_result: MergeFileResult::from_raw(merge_result),
        }
    }

    fn raw(&self) -> raw::git_merge_diff {
        // not implemented, may require a CString in storage
        panic!()
    }
}
