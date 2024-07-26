
pub enum Mutation {
    /// No-op placeholder for code generation.
    _,

    /// Create a customer.
    customerCreate {
        input: CustomerCreateInput,
    },
    /// Patch a customer.
    customerPatch {
        input: CustomerPatchInput,
    },
    /// Delete customer.
    customerDelete {
        input: CustomerDeleteInput,
    },
    /// Create an account.
    accountCreate {
        input: AccountCreateInput,
    },
    /// Archive an account.
    accountArchive {
        input: AccountArchiveInput,
    },
    /// Patch an account.
    accountPatch {
        input: AccountPatchInput,
    },
    /// Create a sales tax.
    salesTaxCreate {
        input: SalesTaxCreateInput,
    },
    /// Update a sales tax.
    salesTaxPatch {
        input: SalesTaxPatchInput,
    },
    /// Archive a sales tax.
    salesTaxArchive {
        input: SalesTaxArchiveInput,
    },
    /// Create an invoice.
    invoiceCreate {
        input: InvoiceCreateInput,
    },
    /// Patch an invoice.
    invoicePatch {
        input: InvoicePatchInput,
    },
    /// Clones an invoice.
    invoiceClone {
        input: InvoiceCloneInput,
    },
    /// Delete an invoice.
    invoiceDelete {
        input: InvoiceDeleteInput,
    },
    /// Send an invoice. Requires `Business.emailSendEnabled` to be true.
    invoiceSend {
        input: InvoiceSendInput,
    },
    /// Approve an invoice.
    invoiceApprove {
        input: InvoiceApproveInput,
    },
    /// Mark the invoice as sent.
    invoiceMarkSent {
        input: InvoiceMarkSentInput,
    },
    /// Create a product.
    productCreate {
        input: ProductCreateInput,
    },
    /// Patch a product.
    productPatch {
        input: ProductPatchInput,
    },
    /// Archive a product.
    productArchive {
        input: ProductArchiveInput,
    },
    /// **BETA**: Create money transaction. Requires `isClassicAccounting` to be `false`.
    moneyTransactionCreate {
        input: MoneyTransactionCreateInput,
    },
    /// Create a money transaction. ⚠️ DEPRECATED
    moneyDepositTransactionCreate {
        input: MoneyDepositTransactionCreateInput,
    },
}
