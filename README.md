All turbin3 assignments and projects are posted here.
For year 2025 Q1 COHORT.

# Milestone Program Documentation

This document provides a comprehensive overview of the Milestone program built on Solana using the Anchor framework.

## Program Overview

The Milestone program facilitates funding transactions between companies and NGOs. Companies can create funding projects with specific requirements, NGOs can apply to these projects, and upon approval, funds are disbursed to the NGOs.

## Program Flow

1. Admin initializes program parameters
2. Companies and NGOs register their accounts
3. Companies create projects with requirements and deposit funds
4. NGOs apply for projects by submitting requirement hashes
5. Companies review applications and accept or reject them
6. For accepted applications, funds are transferred to NGOs
7. Relevant accounts are closed when completed

## Core Components

### Accounts

#### Admin Account

- Manages program parameters
- Sets max projects allowed
- Configures fee structure (in basis points)
- Receives fees from successful transactions

#### Company Account

- Represents companies that create and fund projects
- Tracks company information and total projects created

#### NGO Account

- Represents NGO organizations that apply for projects
- Tracks completed projects and verification data

#### Project Account

- Contains project details created by companies
- Tracks status, requirements, and application counts

#### Vault Account

- Holds funds for projects
- Controls token accounts for USDC transfers

#### Temporary Transaction Account

- Tracks NGO applications to projects
- Holds status of application (Processing/Accepted/Rejected)

#### Project Completion Details

- Created when a project is accepted
- Contains verification data including merkle root

### Key Operations

#### Creating Projects

Companies can create projects by:

- Specifying project requirements (stored as a hash)
- Setting maximum number of NGO applications allowed
- Depositing USDC funds into the project vault

#### Applying for Projects

NGOs can apply to projects by:

- Submitting requirement hashes that match company's criteria
- Creating temporary transaction accounts to track applications

#### Processing Applications

Companies can:

- Accept or reject NGO applications
- If accepted, create completion details with merkle roots for verification

#### Disbursing Funds

For accepted projects:

- Funds are transferred from vault to NGO's token account
- An admin fee is deducted based on fee_basis_points
- Relevant accounts are closed

### Status Management

The program uses several status indicators:

#### Project Status:

- `OpenForApplication`: NGOs can apply
- `NotOpenForApplication`: Temporarily closed for applications
- `Funded`: Project accepted and funded
- `InProgress`: Work is being done (tracked off-chain)
- `Closed`: Project is completed and accounts closed

#### Temp Transaction Status:

- `Processing`: Application submitted but not yet reviewed
- `Accepted`: Application approved
- `Rejected`: Application denied

## Error Handling

The program includes comprehensive error handling for scenarios like:

- Maximum project limits reached
- Invalid fund amounts
- Projects being closed for applications
- Insufficient balances for transfers
- Status inconsistencies

## Events

The program emits events at critical points:

- Project creation
- NGO applications
- Application acceptance/rejection
- Payment processing

## Security Considerations

The program uses PDAs (Program Derived Addresses) with appropriate seed derivation to ensure security for all accounts.

Fee calculation is performed carefully to avoid rounding errors, using basis points (1/100th of a percent) for precision.

## Account Structure and Seeds

- Admin: `["admin"]`
- Company: `["company", signer_pubkey]`
- NGO: `["ngo", signer_pubkey]`
- Project: `["project", company_pubkey, project_name]`
- Vault: `["vault", project_pubkey]`
- TempTransaction: `["temp_tx", project_pubkey, ngo_pubkey]`
- ProjectCompletionDetails: `["project_completion_details", project_pubkey, ngo_pubkey]`

The appropriate use of these seed derivations ensures that accounts are properly associated with the entities that create them, providing a secure permission model.

## Test Results

The program has undergone comprehensive testing with the following test cases all passing successfully:

### Account Initialization Tests

✔ should fail to initialize admin twice
✔ intialize admin
✔ intialize company
✔ intialize ngo account

### Project Creation and Management Tests

✔ should fail to create project with invalid max submissions
✔ should fail to create project with too long name
✔ create project account

### Project Application Tests

✔ should fail to apply for project with wrong requirements hash
✔ apply project
✔ process project

### Payment Processing Tests

✔ should fail to process project with invalid status
✔ should fail to process payment for already processed project
✔ should fail to process payment with insufficient vault balance
✔ should fail to access closed accounts after payment processing
✔ should fail to process payment with already closed accounts
✔ process payment and close accounts

### Account Management Tests

✔ should fail to initialize company with empty name
✔ should fail to close temp account with wrong signer
✔ should fail to edit NGO requirements with wrong signer
✔ should fail to edit project with wrong signer
✔ should fail to edit project with invalid parameters

All 21 tests passed successfully, demonstrating the robustness of the program's error handling and core functionality. The tests cover various scenarios including:

- Account initialization validation
- Project creation constraints
- Application processing rules
- Payment processing security
- Account closure conditions
- Permission checks
- Parameter validation

Test execution time: 5.50 seconds
