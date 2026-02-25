use duckdb::{
    core::{DataChunkHandle, LogicalTypeHandle, LogicalTypeId},
    duckdb_entrypoint_c_api,
    vscalar::{ScalarFunctionSignature, VScalar},
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
    mortgage_type: PaymentScheme,
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
        let principal: f64 = bind.get_parameter(0).to_string().parse::<f64>().unwrap();
        let nperiods: i64 = bind.get_parameter(1).to_int64();
        let year_interest_rate: f64 = bind.get_parameter(2).to_string().parse::<f64>().unwrap();
        let mortgage_type_str: String = bind.get_parameter(3).to_string();
        let mortgage_type: PaymentScheme = match mortgage_type_str.parse::<PaymentScheme>() {
            Ok(payscheme) => payscheme,
            Err(error) => panic!("Problem parsing the mortgage type: {error:?}"),
        };

        Ok(MortgageBindData {
            principal,
            nperiods,
            year_interest_rate: vec![year_interest_rate / 100.0; nperiods as usize],
            mortgage_type: mortgage_type,
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

            let pay: MortgagePayments =
                MortgagePayments::new(mort, bind_data.mortgage_type.clone());

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
            LogicalTypeHandle::from(LogicalTypeId::Varchar),
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
            (
                "mortgage_type".to_string(),
                LogicalTypeHandle::from(LogicalTypeId::Varchar),
            ),
        ])
    }
}

struct PMTVScalar;
impl VScalar for PMTVScalar {
    type State = ();

    unsafe fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn duckdb::vtab::arrow::WritableVector,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let period_interest_rate: f64 = input.flat_vector(0).as_slice()[0];
        let nperiods: i64 = input.flat_vector(1).as_slice()[0];
        let principal: f64 = input.flat_vector(2).as_slice()[0];

        // Mortgage expects a yearly interest rate.
        let year_interest_rate: f64 = (1.0 + period_interest_rate).powf(12.0) - 1.0;

        let mort: Mortgage = Mortgage::new(
            -principal,
            nperiods,
            vec![year_interest_rate; nperiods as usize],
        );

        let pay: MortgagePayments = MortgagePayments::new(mort, PaymentScheme::FixedMensualities);

        let mut output_vector = output.flat_vector();
        output_vector.copy(&[pay.payments()[0]]);

        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![
                LogicalTypeId::Double.into(),  // Interest rate
                LogicalTypeId::Integer.into(), // Number of periods
                LogicalTypeId::Double.into(),  // Principal
            ],
            LogicalTypeId::Double.into(),
        )]
    }
}

const EXTENSION_NAME: &str = env!("CARGO_PKG_NAME");

#[duckdb_entrypoint_c_api()]
pub unsafe fn extension_entrypoint(con: Connection) -> Result<(), Box<dyn Error>> {
    con.register_table_function::<MortgageVTab>(EXTENSION_NAME)
        .expect("Failed to register mortgage_table table function");
    con.register_scalar_function::<PMTVScalar>("PMT")
        .expect("Failed to register PMT scalar function");
    Ok(())
}
