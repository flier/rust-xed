use crate::{enum_properties, Iclass};

impl Iclass {
    enum_properties! {
        /// Return the maximum number of iforms for a particular iclass.
        iform_max: u32 { xed_iform_max_per_iclass }

        /// Return the first of the iforms for a particular iclass.
        iform_first: u32 { xed_iform_first_per_iclass }

        /// Take an instruction with a REP/REPE/REPNE prefix and return the corresponding xed_iclass_enum_t without that prefix.
        ///
        /// The return value differs from the other functions in this group:
        /// If the input iclass does not have REP/REPNE/REPE prefix, the function returns the original instruction.
        rep_remove: Iclass { xed_rep_remove }

        /// Take an `Iclass` value without a REPE prefix and return the corresponding #xed_iclass_enum_t with a REPE prefix.
        ///
        /// If the input instruction cannot have have a REPE prefix, this function returns None.
        repe: Iclass? { xed_repe_map }

        /// Take an `Iclass` value without a REPNE prefix and return the corresponding `Iclass` with a REPNE prefix.
        ///
        /// If the input instruction cannot have a REPNE prefix, this function returns None.
        repne: Iclass? { xed_repne_map }

        /// Take an `Iclass` value without a REP prefix and return the corresponding `Iclass` with a REP prefix.
        ///
        /// If the input instruction cannot have a REP prefix, this function returns None.
        rep: Iclass? { xed_rep_map }

        /// Take an `Iclass` value for an instruction with a REP/REPNE/REPE prefix and return the corresponding `Iclass` without that prefix.
        ///
        /// If the input instruction does not have a REP/REPNE/REPE prefix, this function returns None.
        norep: Iclass? { xed_norep_map }
    }
}
