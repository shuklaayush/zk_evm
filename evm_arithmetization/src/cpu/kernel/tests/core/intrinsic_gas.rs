use anyhow::Result;
use ethereum_types::U256;
use plonky2::field::goldilocks_field::GoldilocksField as F;

use crate::cpu::kernel::aggregator::KERNEL;
use crate::cpu::kernel::constants::global_metadata::GlobalMetadata;
use crate::cpu::kernel::constants::txn_fields::NormalizedTxnField;
use crate::cpu::kernel::interpreter::Interpreter;

const GAS_TX: u32 = 21_000;
const GAS_TXCREATE: u32 = 32_000;

#[test]
fn test_intrinsic_gas() -> Result<()> {
    let intrinsic_gas = KERNEL.global_labels["intrinsic_gas"];

    // Contract creation transaction.
    let initial_stack = vec![0xdeadbeefu32.into()];
    let mut interpreter: Interpreter<F> =
        Interpreter::new_with_kernel(intrinsic_gas, initial_stack.clone());
    interpreter.set_global_metadata_field(GlobalMetadata::ContractCreation, U256::one());
    interpreter.run()?;
    assert_eq!(interpreter.stack(), vec![(GAS_TX + GAS_TXCREATE).into()]);

    // Message transaction.
    let mut interpreter: Interpreter<F> =
        Interpreter::new_with_kernel(intrinsic_gas, initial_stack);
    interpreter.set_txn_field(NormalizedTxnField::To, 123.into());
    interpreter.run()?;
    assert_eq!(interpreter.stack(), vec![GAS_TX.into()]);

    Ok(())
}
