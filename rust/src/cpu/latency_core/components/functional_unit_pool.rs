// use std::{cell::Cell, rc::Rc};

use crate::cpu::{latency_core::latency_args::FunctionalUnitPoolConfig, stats::Tick};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ExecutionClass {
    IntALU,
    IntMult,
    IntDiv,
    Load,
    Store,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[flutter_rust_bridge::frb(dart_metadata=("freezed"))]
pub struct FunctionalUnitGroupCfg {
    pub operation_latency: Tick,
    pub issue_latency: Tick,
    pub num_units: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FunctionalUnit {
    class: ExecutionClass,
    operation_latency: Tick,
    issue_latency: Tick,
    // busy: Cell<Tick>,
    busy: Tick,
}

impl FunctionalUnit {
    pub fn set_busy(&mut self) {
        self.busy = self.issue_latency;
    }

    pub fn op_lat(&self) -> Tick {
        self.operation_latency
    }
}

// pub struct FunctionalUnitGroup {
//     units: Vec<Rc<FunctionalUnit>>,
// }

// impl FunctionalUnitGroup {
//     pub fn new_from_cfg(cfg: Vec<u64>) -> Self {
//         assert!(cfg.len() == 3);
//         Self {
//             units: {
//                 let mut v = Vec::with_capacity(cfg[2] as usize);
//                 (0..cfg[2] as usize).for_each(|_| {
//                     v.push(Rc::new(FunctionalUnit {
//                         operation_latency: cfg[0],
//                         issue_latency: cfg[1],
//                         busy: Cell::new(0),
//                     }))
//                 });
//                 v
//             },
//         }
//     }

//     pub fn get_exec_unit(&mut self) -> Option<Rc<FunctionalUnit>> {
//         for unit in self.units.iter() {
//             if unit.busy.get() == 0 {
//                 unit.set_busy();
//                 return Some(unit.clone());
//             }
//         }
//         None
//     }
// }

pub struct FunctionalUnitPool {
    // int_alu: FunctionalUnitGroup,
    // int_mult: FunctionalUnitGroup,
    // int_div: FunctionalUnitGroup,
    // load: FunctionalUnitGroup,
    // store: FunctionalUnitGroup,
    // busy: Vec<Rc<FunctionalUnit>>,
    units: Vec<FunctionalUnit>,
}

impl FunctionalUnitPool {
    // pub fn new_from_cfg(args: FunctionalUnitPoolConfig) -> Self {
    //     Self {
    //         int_alu: FunctionalUnitGroup::new_from_cfg(args.ialu.unwrap_or(vec![1, 1, 4])),
    //         int_mult: FunctionalUnitGroup::new_from_cfg(args.imult.unwrap_or(vec![1, 3, 1])),
    //         int_div: FunctionalUnitGroup::new_from_cfg(args.idiv.unwrap_or(vec![19, 20, 1])),
    //         load: FunctionalUnitGroup::new_from_cfg(args.load.unwrap_or(vec![1, 20, 2])),
    //         store: FunctionalUnitGroup::new_from_cfg(args.store.unwrap_or(vec![1, 20, 2])),
    //         busy: Vec::new(),
    //     }
    // }

    // pub fn get_exec_unit(&mut self, class: ExecutionClass) -> Option<u64> {
    //     let op_lat;
    //     if let Some(unit) = match class {
    //         ExecutionClass::IntALU => self.int_alu.get_exec_unit(),
    //         ExecutionClass::IntMult => self.int_mult.get_exec_unit(),
    //         ExecutionClass::IntDiv => self.int_div.get_exec_unit(),
    //         ExecutionClass::Load => self.load.get_exec_unit(),
    //         ExecutionClass::Store => self.store.get_exec_unit(),
    //     } {
    //         op_lat = unit.op_lat();
    //         self.busy.push(unit);
    //         Some(op_lat)
    //     } else {
    //         None
    //     }
    // }

    // pub fn execute_cycle(&mut self) {
    //     for unit in self.busy.iter() {
    //         unit.exec_cycle()
    //     }
    //     self.busy.retain(|unit| unit.busy.get() != 0)
    // }

    pub fn new() -> Self {
        Self { units: Vec::new() }
    }

    pub fn new_from_cfg(args: FunctionalUnitPoolConfig) -> Self {
        let mut pool = FunctionalUnitPool::new();
        pool.add_exec_unit_group_cfg(ExecutionClass::IntALU, args.ialu);
        pool.add_exec_unit_group_cfg(ExecutionClass::IntMult, args.imult);
        pool.add_exec_unit_group_cfg(ExecutionClass::IntDiv, args.idiv);
        pool.add_exec_unit_group_cfg(ExecutionClass::Load, args.load);
        pool.add_exec_unit_group_cfg(ExecutionClass::Store, args.store);
        pool
    }

    // pub fn default() -> Self {
    //     let mut func_pool = FunctionalUnitPool::new();
    //     func_pool.add_exec_unit_group(ExecutionClass::IntALU, 1, 1, 4);
    //     func_pool.add_exec_unit_group(ExecutionClass::IntMult, 1, 3, 1);
    //     func_pool.add_exec_unit_group(ExecutionClass::IntDiv, 19, 20, 1);
    //     func_pool.add_exec_unit_group(ExecutionClass::Load, 1, 20, 2);
    //     func_pool.add_exec_unit_group(ExecutionClass::Store, 1, 20, 2);
    //     func_pool
    // }

    // pub fn add_exec_unit_group(
    //     &mut self,
    //     class: ExecutionClass,
    //     issue_latency: u8,
    //     operation_latency: u8,
    //     num_units: u8,
    // ) {
    //     self.units.append(&mut vec![
    //         FunctionalUnit {
    //             class,
    //             issue_latency,
    //             operation_latency,
    //             busy: 0,
    //         };
    //         num_units as usize
    //     ]);
    // }

    fn add_exec_unit_group_cfg(&mut self, class: ExecutionClass, cfg: FunctionalUnitGroupCfg) {
        self.units.append(&mut vec![
            FunctionalUnit {
                class,
                issue_latency: cfg.issue_latency,
                operation_latency: cfg.operation_latency,
                busy: 0,
            };
            cfg.num_units as usize
        ]);
    }

    pub fn get_exec_unit(&mut self, class: ExecutionClass) -> Option<&mut FunctionalUnit> {
        // let exec_units = self.map_enum_to_vec(class);
        self.units
            .iter_mut()
            .find(|exec_unit| exec_unit.busy == 0 && exec_unit.class == class)
    }

    pub fn execute_cycle(&mut self) {
        for unit in self.units.iter_mut() {
            unit.busy = unit.busy.saturating_sub(1);
        }
    }
}
