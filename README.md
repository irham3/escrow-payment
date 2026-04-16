Here is the professional `README.md` for your Escrow Smart Contract in English.

---

# Soroban Escrow Smart Contract

This smart contract facilitates secure transactions between a **Buyer** and a **Seller** using a third-party **Arbiter** as a mediator in case of disputes.

**Contract ID:** `CABLDP7CLBSMDDXE5YKDTWOUWZKPDE2BUHYCQKCICQSTZYSGF2FPWHNO`
**Network:** Soroban (Stellar)

## 📌 Workflow

1.  **Initialize**: The contract is initialized by setting the Buyer, Seller, and Arbiter addresses, the Token type, and the target Amount.
2.  **Deposit**: The Buyer transfers the specified token amount into the contract. The status changes to `Funded`.
3.  **Release/Refund**: 
    * The Buyer can release the funds to the Seller.
    * The Seller can refund the funds back to the Buyer.
4.  **Dispute**: If an issue arises, either party can raise a dispute.
5.  **Arbiter Resolution**: Once in a `Disputed` state, only the Arbiter has the authority to resolve it by choosing to `Release` (to Seller) or `Refund` (to Buyer).

## 🛠 Core Functions

### Initialization & State
- `initialize(buyer: Address, seller: Address, arbiter: Address, token: Address, amount: i128)`
- `get_status() -> EscrowStatus` (Returns: `Pending`, `Funded`, `Released`, `Refunded`, `Disputed`)
- `get_amount() -> i128` (Returns the expected deposit amount)

### Transactional Actions
- `deposit(from: Address)`: Transfers funds from the Buyer into the contract (Requires Authorization).
- `release(caller: Address)`: Transfers funds from the contract to the Seller (Requires Authorization from Buyer or Arbiter).
- `refund(caller: Address)`: Transfers funds back to the Buyer (Requires Authorization from Seller or Arbiter).
- `dispute(caller: Address)`: Locks the funds in a dispute state (Requires Authorization from Buyer or Seller).

## 🚀 CLI Interaction Guide

Use the `stellar contract invoke` command to interact with the deployed contract on Testnet.

### 1. Initialize the Contract
Replace the wallet addresses with your specific accounts:
```bash
stellar contract invoke \
  --id CABLDP7CLBSMDDXE5YKDTWOUWZKPDE2BUHYCQKCICQSTZYSGF2FPWHNO \
  --source-account irham3 \
  --network testnet \
  -- \
  initialize \
  --buyer G... \
  --seller G... \
  --arbiter G... \
  --token CAS3J7... (Token Contract ID) \
  --amount 1000000000 (100 units if decimal is 7)
```

### 2. Deposit Funds (By Buyer)
**Note:** The Buyer must first grant `allowance` to this Contract ID before calling deposit.
```bash
stellar contract invoke \
  --id CABLDP7CLBSMDDXE5YKDTWOUWZKPDE2BUHYCQKCICQSTZYSGF2FPWHNO \
  --source-account buyer_alias \
  --network testnet \
  -- \
  deposit --from G_BUYER_ADDRESS
```

### 3. Check Current Status
```bash
stellar contract invoke \
  --id CABLDP7CLBSMDDXE5YKDTWOUWZKPDE2BUHYCQKCICQSTZYSGF2FPWHNO \
  --network testnet \
  -- \
  get_status
```

### 4. Release Funds (By Buyer or Arbiter)
```bash
stellar contract invoke \
  --id CABLDP7CLBSMDDXE5YKDTWOUWZKPDE2BUHYCQKCICQSTZYSGF2FPWHNO \
  --source-account irham3 \
  --network testnet \
  -- \
  release --caller G_BUYER_OR_ARBITER_ADDRESS
```

## 🔐 Security & Validation
- **require_auth**: All sensitive functions require a cryptographic signature (authorization) from the relevant party.
- **State Guarding**: Funds can only be released or refunded if the contract is in the `Funded` or `Disputed` state.
- **Atomicity**: Built using the official Soroban Token SDK to ensure token transfers are atomic and secure within the contract environment.

---
Developed for the Stellar Soroban ecosystem.