use duckdb::{
    core::{DataChunkHandle, LogicalTypeHandle, LogicalTypeId},
    duckdb_entrypoint_c_api,
    vtab::{BindInfo, InitInfo, TableFunctionInfo, VTab},
    Connection, Result,
};
use std::{
    error::Error,
    f64,
    sync::atomic::{AtomicBool, Ordering},
};

use mortgage_sim::mortgage::Mortgage;
use mortgage_sim::paymentschemes::{MortgagePayments, PaymentScheme};

#[repr(C)]
struct MortgageBindData {
    principal: f64,
    nperiods: i64,
    year_interest_rate: Vec<f64>,
}

#[repr(C)]
struct MortgageInitData {
    done: AtomicBool,
}

struct MortgageVTab;

impl VTab for MortgageVTab {
    type InitData = MortgageInitData;
    type BindData = MortgageBindData;

    fn bind(bind: &BindInfo) -> Result<Self::BindData, Box<dyn std::error::Error>> {
        bind.add_result_column("month", LogicalTypeHandle::from(LogicalTypeId::Bigint));
        bind.add_result_column("payments", LogicalTypeHandle::from(LogicalTypeId::Double));
        bind.add_result_column("capital", LogicalTypeHandle::from(LogicalTypeId::Double));
        bind.add_result_column("interest", LogicalTypeHandle::from(LogicalTypeId::Double));
        let principal: f64 = bind.get_parameter(0).to_int64() as f64;
        let nperiods: i64 = bind.get_parameter(1).to_int64();
        let year_interest_rate: f64 = bind.get_parameter(2).to_int64() as f64;
        Ok(MortgageBindData {
            principal,
            nperiods,
            year_interest_rate: vec![year_interest_rate / 100.0; nperiods as usize],
        })
    }

    fn init(_: &InitInfo) -> Result<Self::InitData, Box<dyn std::error::Error>> {
        Ok(MortgageInitData {
            done: AtomicBool::new(false),
        })
    }

    fn func(
        func: &TableFunctionInfo<Self>,
        output: &mut DataChunkHandle,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let init_data = func.get_init_data();
        let bind_data = func.get_bind_data();
        if init_data.done.swap(true, Ordering::Relaxed) {
            output.set_len(0);
        } else {
            let mort: Mortgage = Mortgage::new(
                bind_data.principal,
                bind_data.nperiods,
                bind_data.year_interest_rate.clone(),
            );
            let pay: MortgagePayments = MortgagePayments::new(mort, PaymentScheme::FixedCapital);

            output
                .flat_vector(0)
                .copy((1..=bind_data.nperiods).collect::<Vec<i64>>().as_slice());

            output.flat_vector(1).copy(pay.payments().as_slice());

            output.flat_vector(2).copy(pay.capital_paid().as_slice());

            output.flat_vector(3).copy(pay.interest_paid().as_slice());

            output.set_len(bind_data.nperiods as usize);
        }
        Ok(())
    }

    fn parameters() -> Option<Vec<LogicalTypeHandle>> {
        Some(vec![
            LogicalTypeHandle::from(LogicalTypeId::Float),
            LogicalTypeHandle::from(LogicalTypeId::Integer),
            LogicalTypeHandle::from(LogicalTypeId::Float),
        ])
    }

    fn named_parameters() -> Option<Vec<(String, LogicalTypeHandle)>> {
        Some(vec![
            (
                "principal".to_string(),
                LogicalTypeHandle::from(LogicalTypeId::Float),
            ),
            (
                "number_months".to_string(),
                LogicalTypeHandle::from(LogicalTypeId::Integer),
            ),
            (
                "year_interest_rate".to_string(),
                LogicalTypeHandle::from(LogicalTypeId::Float),
            ),
        ])
    }
}

const EXTENSION_NAME: &str = env!("CARGO_PKG_NAME");

#[duckdb_entrypoint_c_api()]
pub unsafe fn extension_entrypoint(con: Connection) -> Result<(), Box<dyn Error>> {
    con.register_table_function::<MortgageVTab>("mortgage_table")
        .expect("Failed to register hello table function");
    Ok(())
}
