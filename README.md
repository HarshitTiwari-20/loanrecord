# Loan Record Smart Contract

## Project Description
A streamlined smart contract built on **Stellar** via **Soroban**. It is designed to manage all records of various loan transactions reliably on-chain. It establishes a verifiable, immutable history of financial agreements between lenders and borrowers.

## What it does
It can manage the entire lifecycle of a loan giving and repayment transaction. Specifically, it allows a lender to issue a new loan with a set amount and an assigned borrower. From there, the borrower can securely submit partial or full repayments against that loan, and the contract automatically tracks the amount repaid, ensuring that the loan state flips to "inactive" once the principal amount is settled.

## Features
- **On-chain Initialization:** Lenders can create an immutable loan record assigned to a borrower.
- **Secure Authentication:** `require_auth` ensures only the valid lender can initiate a loan, and only the specific borrower can make repayments.
- **Dynamic Repayment Tracking:** Incrementally add repayments through multiple installments.
- **Automatic Status Management:** Automatically flags a loan (`is_active: false`) when the `repaid` amount equals or exceeds the total loan `amount`.
- **Querying Capability:** Anyone can fetch the full state of a specific loan ID at any time.

## Deployed Smart Contract Link
<img width="1008" height="900" alt="Contract ScreenShort" src="https://github.com/HarshitTiwari-20/loanrecord/blob/main/stellar_contract.png" />



**Explorer Link:** [Stellar Expert Testnet](https://stellar.expert/explorer/testnet/tx/6c0aae8600dba0ceb2e2335d9f285494790622ede7433cf2e208b93d6f071190)
