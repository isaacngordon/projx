use crate::io;


// trait Mutation<T: Input + Display, V> {
//     fn mutate(input: io::T) -> V {
//         println!("{}", input.to_string());
//         todo!()
//     }
// }


// impl<T: Input, V> Mutation<T, V> for WaveAppMutation {
//     fn mutate(input: io::T) -> V {
//         todo!()
//     }
// }

pub enum WaveAppMutation {
    /// Create a customer.
    customerCreate {
        input: io::CustomerCreateInput,
        output: io::CustomerCreateOutput, 
    },
    // /// Patch a customer.
    // customerPatch {
    //     input: io::CustomerPatchInput,
    //     output: io::CustomerPatchOutput,
    // },
    // /// Delete customer.
    // customerDelete {
    //     input: io::CustomerDeleteInput,
    //     output: io::CustomerDeleteOutput,
    // },
    // /// Create an account.
    // accountCreate {
    //     input: io::AccountCreateInput,
    //     output: io::AccountCreateOutput,
    // },
    // /// Archive an account.
    // accountArchive {
    //     input: io::AccountArchiveInput,
    //     output: io::AccountArchiveOutput,
    // },
    // /// Patch an account.
    // accountPatch {
    //     input: io::AccountPatchInput,
    //     output: io::AccountPatchOutput,
    // },
    // /// Create a sales tax.
    // salesTaxCreate {
    //     input: io::SalesTaxCreateInput,
    //     output: io::SalesTaxCreateOutput,
    // },
    // /// Update a sales tax.
    // salesTaxPatch {
    //     input: io::SalesTaxPatchInput,
    //     output: io::SalesTaxPatchOutput,
    // },
    // /// Archive a sales tax.
    // salesTaxArchive {
    //     input: io::SalesTaxArchiveInput,
    //     output: io::SalesTaxArchiveOutput,
    // },
    // /// Create an invoice.
    // invoiceCreate {
    //     input: io::InvoiceCreateInput,
    //     output: io::InvoiceCreateOutput,
    // },
    // /// Patch an invoice.
    // invoicePatch {
    //     input: io::InvoicePatchInput,
    //     output: io::InvoicePatchOutput,
    // },
    // /// Clones an invoice.
    // invoiceClone {
    //     input: io::InvoiceCloneInput,
    //     output: io::InvoiceCloneOutput,
    // },
    // /// Delete an invoice.
    // invoiceDelete {
    //     input: io::InvoiceDeleteInput,
    //     output: io::InvoiceDeleteOutput,
    // },
    // /// Send an invoice. Requires `Business.emailSendEnabled` to be true.
    // invoiceSend {
    //     input: io::InvoiceSendInput,
    //     output: io::InvoiceSendOutput,
    // },
    // /// Approve an invoice.
    // invoiceApprove {
    //     input: io::InvoiceApproveInput,
    //     output: io::InvoiceApproveOutput,
    // },
    // /// Mark the invoice as sent.
    // invoiceMarkSent {
    //     input: io::InvoiceMarkSentInput,
    //     output: io::InvoiceMarkSentOutput,
    // },
    // /// Create a product.
    // productCreate {
    //     input: io::ProductCreateInput,
    //     output: io::ProductCreateOutput,
    // },
    // /// Patch a product.
    // productPatch {
    //     input: io::ProductPatchInput,
    //     output: io::ProductPatchOutput,
    // },
    // /// Archive a product.
    // productArchive {
    //     input: io::ProductArchiveInput,
    //     output: io::ProductArchiveOutput,
    // },
    // /// **BETA**: Create money transaction. Requires `isClassicAccounting` to be `false`.
    // moneyTransactionCreate {
    //     input: io::MoneyTransactionCreateInput,
    //     output: io::MoneyTransactionCreateOutput,
    // },
    // /// Create a money transaction. ⚠️ DEPRECATED
    // moneyDepositTransactionCreate {
    //     input: io::MoneyDepositTransactionCreateInput,
    //     output: io::MoneyDepositTransactionCreateOutput,
    // },
}


// TODO: Implement new and default for WaveAppMutation
// impl WaveAppMutation {
//     fn to_payload(self) -> String {
        
//         let operation_variables = 
//         r#"
//         {{
//             businessId: "{}",
//             name: "{} {}",
//             firstName: "{}",
//             lastName: "{}",
//             email: "{}",
//             phone: "{}",
//             address: "{}"
//             currency {{
//                 code: "{}"
//             }}
//         }}"#;

//         let mutation_spec = 
//         r#"customer {{
//                 id
//                 name
//                 email
//                 phone
//                 address
//             }}"#;

//         let mutation = format!(
//             r#"
//             mutation(input: {}) {{
//                 {}(input: $input) {{
//                     {}
//                 }}
//             }}
//             "#,
//             input_type, 
//             mutation_name,
//             mutation_spec
//         );


//         payload
//    }
// }