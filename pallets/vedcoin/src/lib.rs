#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_ok, assert_noop, parameter_types, traits::{OnFinalize, OnInitialize}};
    use sp_core::H256;
    use frame_system as system;
    use sp_runtime::{testing::Header, traits::{BlakeTwo256, IdentityLookup}, Perbill};

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test where
            Block = Block,
            NodeBlock = Block,
            UncheckedExtrinsic = UncheckedExtrinsic,
        {
            System: frame_system,
            VedCoin: crate,
        }
    );

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const MaxSymbolLength: u32 = 8;
        pub const ExistentialDeposit: u128 = 1;
        pub const PalletId: frame_support::PalletId = frame_support::PalletId(*b"ved/coin");
        pub const FeeBurnPercentage: Perbill = Perbill::from_percent(10);
    }

    impl system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type Origin = Origin;
        type Call = Call;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<u64>;
        type Header = Header;
        type Event = Event;
        type BlockHashCount = BlockHashCount;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = pallet_balances::AccountData<u128>;
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
        type OnSetCode = ();
        type MaxConsumers = frame_support::traits::ConstU32<16>;
    }

    impl Config for Test {
        type RuntimeEvent = Event;
        type Currency = pallet_balances::Pallet<Test>;
        type MaxSymbolLength = MaxSymbolLength;
        type PalletId = PalletId;
        type FeeBurnPercentage = FeeBurnPercentage;
    }

    parameter_types! {
        pub const ExistentialDeposit2: u128 = 1;
    }
    impl pallet_balances::Config for Test {
        type Balance = u128;
        type DustRemoval = ();
        type RuntimeEvent = Event;
        type ExistentialDeposit = ExistentialDeposit2;
        type AccountStore = System;
        type WeightInfo = ();
        type MaxLocks = (); type MaxReserves = (); type ReserveIdentifier = [u8; 8];
        type FreezeIdentifier = (); type MaxFreezes = (); type MaxHolds = (); type HoldIdentifier = (); 
    }

    pub fn new_test_ext() -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
        pallet_balances::GenesisConfig::<Test> {
            balances: vec![(1, 1_000_000), (2, 1_000_000), (3, 1_000_000)],
        }.assimilate_storage(&mut t).unwrap();
        t.into()
    }

    #[test]
    fn initialize_token_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(VedCoin::initialize_token(
                Origin::root(),
                b"VED".to_vec(),
                b"VedCoin".to_vec(),
                18,
                1_000_000_000_000_000_000u128
            ));
            let info = VedCoin::token_info().unwrap();
            assert_eq!(info.symbol, b"VED".to_vec());
            assert_eq!(info.name, b"VedCoin".to_vec());
            assert_eq!(info.decimals, 18);
            assert_eq!(info.total_supply, 1_000_000_000_000_000_000u128);
        });
    }

    #[test]
    fn transfer_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(VedCoin::initialize_token(Origin::root(), b"VED".to_vec(), b"VedCoin".to_vec(), 18, 1_000_000_000_000_000_000u128));
            assert_ok!(VedCoin::transfer(Origin::signed(1), 2, 100_000));
        });
    }

    #[test]
    fn mint_tokens_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(VedCoin::initialize_token(Origin::root(), b"VED".to_vec(), b"VedCoin".to_vec(), 18, 1_000_000_000_000_000_000u128));
            assert_ok!(VedCoin::mint_tokens(Origin::root(), 1, 500_000));
        });
    }

    #[test]
    fn burn_tokens_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(VedCoin::initialize_token(Origin::root(), b"VED".to_vec(), b"VedCoin".to_vec(), 18, 1_000_000_000_000_000_000u128));
            assert_ok!(VedCoin::burn_tokens(Origin::signed(1), 100_000));
        });
    }

    #[test]
    fn register_and_stake_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(VedCoin::initialize_token(Origin::root(), b"VED".to_vec(), b"VedCoin".to_vec(), 18, 1_000_000_000_000_000_000u128));
            assert_ok!(VedCoin::register_validator(Origin::signed(1), 10));
            assert_ok!(VedCoin::stake(Origin::signed(1), 1, 100_000));
            assert_ok!(VedCoin::stake(Origin::signed(2), 1, 50_000));
        });
    }

    #[test]
    fn unstake_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(VedCoin::initialize_token(Origin::root(), b"VED".to_vec(), b"VedCoin".to_vec(), 18, 1_000_000_000_000_000_000u128));
            assert_ok!(VedCoin::register_validator(Origin::signed(1), 10));
            assert_ok!(VedCoin::stake(Origin::signed(1), 1, 100_000));
            assert_ok!(VedCoin::unstake(Origin::signed(1), 1, 50_000));
        });
    }

}
#![cfg_attr(not(feature = "std"), no_std)]

//! # VedCoin Pallet
//! 
//! The VedCoin pallet implements the core VED token functionality including:
//! - Token transfers and balances
//! - Staking and delegation
//! - Fee payment and burning
//! - Cross-chain bridge support

use frame_support::{
    codec::{Decode, Encode},
    dispatch::{DispatchError, DispatchResult},
    traits::{Currency, Get, ReservableCurrency},
    PalletId, RuntimeDebug,
};
use frame_system::ensure_signed;
use scale_info::TypeInfo;
use sp_runtime::{
    traits::{AccountIdConversion, Saturating, Zero},
    Perbill,
};
use sp_std::vec::Vec;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The currency used for fee payment.
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        /// The maximum length of a token symbol.
        #[pallet::constant]
        type MaxSymbolLength: Get<u32>;

        /// The pallet id, used for deriving sovereign account IDs.
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        /// Percentage of fees that should be burned (vs sent to treasury).
        #[pallet::constant]
        type FeeBurnPercentage: Get<Perbill>;
    }

    /// Token metadata
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct TokenInfo {
        /// Token symbol (e.g., "VED")
        pub symbol: Vec<u8>,
        /// Token name (e.g., "VedCoin")
        pub name: Vec<u8>,
        /// Number of decimal places
        pub decimals: u8,
        /// Total supply
        pub total_supply: u128,
        /// Circulating supply (total - burned)
        pub circulating_supply: u128,
    }

    /// Staking information for an account
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct StakingInfo<AccountId, Balance> {
        /// Amount staked by this account
        pub staked: Balance,
        /// Accounts that have delegated to this validator
        pub delegators: Vec<(AccountId, Balance)>,
        /// Commission rate for this validator (0-100%)
        pub commission: u8,
        /// Whether this account is an active validator
        pub is_validator: bool,
    }

    #[pallet::storage]
    #[pallet::getter(fn token_info)]
    /// Token metadata storage
    pub type TokenInfoStorage<T: Config> = StorageValue<_, TokenInfo>;

    #[pallet::storage]
    #[pallet::getter(fn staking_info)]
    /// Staking information for accounts
    pub type StakingInfoStorage<T: Config> = 
        StorageMap<_, Blake2_128Concat, T::AccountId, StakingInfo<T::AccountId, u128>>;

    #[pallet::storage]
    #[pallet::getter(fn total_staked)]
    /// Total amount staked in the network
    pub type TotalStaked<T: Config> = StorageValue<_, u128, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn burned_tokens)]
    /// Total amount of tokens burned
    pub type BurnedTokens<T: Config> = StorageValue<_, u128, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Tokens were burned [amount]
        TokensBurned { amount: u128 },
        
        /// Tokens were staked [validator, staker, amount]
        TokensStaked {
            validator: T::AccountId,
            staker: T::AccountId,
            amount: u128,
        },
        
        /// Tokens were unstaked [validator, staker, amount]
        TokensUnstaked {
            validator: T::AccountId,
            staker: T::AccountId,
            amount: u128,
        },
        
        /// Validator was registered [account, commission]
        ValidatorRegistered {
            account: T::AccountId,
            commission: u8,
        },
        
        /// Staking rewards were distributed [validator, total_rewards]
        RewardsDistributed {
            validator: T::AccountId,
            total_rewards: u128,
        },

        /// Tokens transferred [from, to, amount]
        TokensTransferred {
            from: T::AccountId,
            to: T::AccountId,
            amount: u128,
        },

        /// Tokens minted [to, amount]
        TokensMinted {
            to: T::AccountId,
            amount: u128,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Token info already initialized
        TokenAlreadyInitialized,
        /// Token info not found
        TokenNotInitialized,
        /// Invalid commission rate (must be 0-100)
        InvalidCommission,
        /// Insufficient staked balance
        InsufficientStake,
        /// Account is not a validator
        NotValidator,
        /// Maximum symbol length exceeded
        SymbolTooLong,
        /// Cannot stake zero amount
        ZeroStake,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Initialize token information (can only be called once)
        #[pallet::weight(10_000)]
        #[pallet::call_index(0)]
        pub fn initialize_token(
            origin: OriginFor<T>,
            symbol: Vec<u8>,
            name: Vec<u8>,
            decimals: u8,
            total_supply: u128,
        ) -> DispatchResult {
            ensure_root(origin)?;

            ensure!(
                symbol.len() <= T::MaxSymbolLength::get() as usize,
                Error::<T>::SymbolTooLong
            );

            ensure!(
                !TokenInfoStorage::<T>::exists(),
                Error::<T>::TokenAlreadyInitialized
            );

            let token_info = TokenInfo {
                symbol,
                name,
                decimals,
                total_supply,
                circulating_supply: total_supply,
            };

            TokenInfoStorage::<T>::put(&token_info);

            Ok(())
        }

        /// Transfer tokens between accounts
        #[pallet::weight(10_000)]
        #[pallet::call_index(99)]
        pub fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: u128,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;
            ensure!(amount > 0, Error::<T>::ZeroStake);
            T::Currency::transfer(&from, &to, amount, frame_support::traits::ExistenceRequirement::AllowDeath)?;
            Self::deposit_event(Event::TokensTransferred { from: from.clone(), to: to.clone(), amount });
            Ok(())
        }

        /// Mint new tokens (root only, for controlled supply increases)
        #[pallet::weight(10_000)]
        #[pallet::call_index(98)]
        pub fn mint_tokens(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: u128,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(amount > 0, Error::<T>::ZeroStake);
            T::Currency::deposit_creating(&to, amount);
            // Update token info
            if let Some(mut token_info) = Self::token_info() {
                token_info.circulating_supply = token_info.circulating_supply.saturating_add(amount);
                token_info.total_supply = token_info.total_supply.saturating_add(amount);
                TokenInfoStorage::<T>::put(&token_info);
            }
            Self::deposit_event(Event::TokensMinted { to: to.clone(), amount });
            Ok(())
        }

        /// Register as a validator with specified commission rate
        #[pallet::weight(10_000)]
        #[pallet::call_index(1)]
        pub fn register_validator(
            origin: OriginFor<T>,
            commission: u8,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(commission <= 100, Error::<T>::InvalidCommission);

            let staking_info = StakingInfo {
                staked: 0u128,
                delegators: Vec::new(),
                commission,
                is_validator: true,
            };

            StakingInfoStorage::<T>::insert(&who, &staking_info);

            Self::deposit_event(Event::ValidatorRegistered {
                account: who,
                commission,
            });

            Ok(())
        }

        /// Stake tokens to a validator
        #[pallet::weight(10_000)]
        #[pallet::call_index(2)]
        pub fn stake(
            origin: OriginFor<T>,
            validator: T::AccountId,
            amount: u128,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(amount > 0, Error::<T>::ZeroStake);

            // Ensure validator exists
            let mut validator_info = Self::staking_info(&validator)
                .ok_or(Error::<T>::NotValidator)?;

            ensure!(validator_info.is_validator, Error::<T>::NotValidator);

            // Reserve the tokens (this checks balance)
            let amount_to_reserve = amount.saturated_into();
            T::Currency::reserve(&who, amount_to_reserve)?;

            // Update staking info
            if who == validator {
                // Self-staking
                validator_info.staked = validator_info.staked.saturating_add(amount);
            } else {
                // Delegation
                if let Some(pos) = validator_info.delegators.iter().position(|(acc, _)| acc == &who) {
                    validator_info.delegators[pos].1 = 
                        validator_info.delegators[pos].1.saturating_add(amount);
                } else {
                    validator_info.delegators.push((who.clone(), amount));
                }
            }

            StakingInfoStorage::<T>::insert(&validator, &validator_info);
            TotalStaked::<T>::put(Self::total_staked().saturating_add(amount));

            Self::deposit_event(Event::TokensStaked {
                validator,
                staker: who,
                amount,
            });

            Ok(())
        }

        /// Unstake tokens from a validator
        #[pallet::weight(10_000)]
        #[pallet::call_index(3)]
        pub fn unstake(
            origin: OriginFor<T>,
            validator: T::AccountId,
            amount: u128,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(amount > 0, Error::<T>::ZeroStake);

            let mut validator_info = Self::staking_info(&validator)
                .ok_or(Error::<T>::NotValidator)?;

            let amount_to_unreserve = amount.saturated_into();

            if who == validator {
                // Self-unstaking
                ensure!(
                    validator_info.staked >= amount,
                    Error::<T>::InsufficientStake
                );
                validator_info.staked = validator_info.staked.saturating_sub(amount);
            } else {
                // Undelegation
                if let Some(pos) = validator_info.delegators.iter().position(|(acc, _)| acc == &who) {
                    ensure!(
                        validator_info.delegators[pos].1 >= amount,
                        Error::<T>::InsufficientStake
                    );
                    validator_info.delegators[pos].1 = 
                        validator_info.delegators[pos].1.saturating_sub(amount);
                    
                    // Remove delegator if stake becomes zero
                    if validator_info.delegators[pos].1 == 0 {
                        validator_info.delegators.remove(pos);
                    }
                } else {
                    return Err(Error::<T>::InsufficientStake.into());
                }
            }

            // Unreserve the tokens
            T::Currency::unreserve(&who, amount_to_unreserve);

            StakingInfoStorage::<T>::insert(&validator, &validator_info);
            TotalStaked::<T>::put(Self::total_staked().saturating_sub(amount));

            Self::deposit_event(Event::TokensUnstaked {
                validator,
                staker: who,
                amount,
            });

            Ok(())
        }

        /// Burn tokens (reduce circulating supply)
        #[pallet::weight(10_000)]
        #[pallet::call_index(4)]
        pub fn burn_tokens(origin: OriginFor<T>, amount: u128) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Burn tokens from the caller's account
            let amount_to_burn = amount.saturated_into();
            T::Currency::withdraw(
                &who,
                amount_to_burn,
                frame_support::traits::WithdrawReasons::all(),
                frame_support::traits::ExistenceRequirement::AllowDeath,
            )?;

            // Update token info
            if let Some(mut token_info) = Self::token_info() {
                token_info.circulating_supply = 
                    token_info.circulating_supply.saturating_sub(amount);
                TokenInfoStorage::<T>::put(&token_info);
            }

            BurnedTokens::<T>::put(Self::burned_tokens().saturating_add(amount));

            Self::deposit_event(Event::TokensBurned { amount });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get the account ID of the pallet
        pub fn account_id() -> T::AccountId {
            T::PalletId::get().into_account_truncating()
        }

        /// Calculate and distribute staking rewards
        pub fn distribute_rewards(validator: &T::AccountId, total_rewards: u128) -> DispatchResult {
            if let Some(validator_info) = Self::staking_info(validator) {
                let commission_amount = total_rewards
                    .saturating_mul(validator_info.commission as u128)
                    .saturating_div(100);
                
                let delegator_rewards = total_rewards.saturating_sub(commission_amount);
                let total_delegated: u128 = validator_info.delegators
                    .iter()
                    .map(|(_, amount)| *amount)
                    .sum();

                // Distribute to delegators proportionally
                for (delegator, stake) in &validator_info.delegators {
                    if total_delegated > 0 {
                        let delegator_reward = delegator_rewards
                            .saturating_mul(*stake)
                            .saturating_div(total_delegated);
                        
                        // Mint rewards to delegator
                        let reward_amount = delegator_reward.saturated_into();
                        let _ = T::Currency::deposit_creating(delegator, reward_amount);
                    }
                }

                // Give commission to validator
                let commission_balance = commission_amount.saturated_into();
                let _ = T::Currency::deposit_creating(validator, commission_balance);

                Self::deposit_event(Event::RewardsDistributed {
                    validator: validator.clone(),
                    total_rewards,
                });
            }

            Ok(())
        }
    }
}

// Runtime API for external queries
sp_api::decl_runtime_apis! {
    pub trait VedCoinApi<AccountId> {
        fn get_token_info() -> Option<TokenInfo>;
        fn get_staking_info(account: AccountId) -> Option<StakingInfo<AccountId, u128>>;
        fn get_total_staked() -> u128;
        fn get_burned_tokens() -> u128;
    }
}
