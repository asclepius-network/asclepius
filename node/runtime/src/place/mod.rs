use crate::identifier;
use codec::{Decode, Encode};
use rstd::prelude::*;
use sr_primitives::weights::SimpleDispatchInfo;
/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs
use support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure};
use system::ensure_signed;

/// Data storage type for each account
#[derive(Encode, Decode, Default, Clone, Eq, PartialEq)]
pub struct Place<AccountId> {
    /// Owner of the place
    pub owner: AccountId,
    /// Visited people
    pub visited: Vec<AccountId>,
}

impl<AccountId> Place<AccountId> {
    pub fn new(owner: AccountId, visited: Vec<AccountId>) -> Self {
        Place { owner, visited }
    }
}

// Module's function and Methods of custom struct to be placed here
impl<T: Trait> Module<T> {}

/// The module's configuration trait.
pub trait Trait: system::Trait + timestamp::Trait + identifier::Trait {
    // TODO: Add other types and constants required configure this module.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as DID {
        pub Places get(place): map u64 => Place<T::AccountId>;
        pub Sudoers get(sudoer): map u64 => T::AccountId;
        pub Visits get(visit): map (u64, T::AccountId) => T::Moment;
    }
}

// The module's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        // this is needed only if you are using events in your module
        fn deposit_event() = default;

        #[weight = SimpleDispatchInfo::FixedNormal(0)]
        pub fn register_place(origin, place_id: u64) -> Result {
            let owner = ensure_signed(origin)?;
            ensure!(!<Places<T>>::exists(place_id.clone()), "The place already exists");
            let visitors: Vec<T::AccountId> = vec![];
            let place = Place::new(owner.clone(), visitors);
            <Places<T>>::insert(place_id, place);
            Self::deposit_event(RawEvent::PlaceRegistered(owner, place_id));
            Ok(())
        }

        #[weight = SimpleDispatchInfo::FixedNormal(0)]
        pub fn set_sudo(origin, sudo_id: T::AccountId, place_id: u64) -> Result {
            let owner = ensure_signed(origin)?;
            ensure!(<Places<T>>::exists(place_id.clone()), "Place is not registered");
            ensure!(owner == Self::place(place_id.clone()).owner, "Requested place is not owned by the sender");
            if !<Sudoers<T>>::exists(place_id.clone()) {
                <Sudoers<T>>::insert(place_id.clone(), sudo_id );
                Self::deposit_event(RawEvent::PlaceSudoUpdated(place_id));
                return Ok(());
            }
            else {
                <Sudoers<T>>::mutate(place_id.clone(), |sudo| *sudo = sudo_id);
            }
            Self::deposit_event(RawEvent::PlaceSudoUpdated(place_id));
            Ok(())
        }

        pub fn set_infected(origin, place_id: u64) -> Result {
            let sender = ensure_signed(origin)?;
            let place: Place<T::AccountId> = Self::place(place_id.clone());
            // ensure place exists
            ensure!(<Places<T>>::exists(place_id.clone()), "Place is not registered");
            // ensure sender is owner or sudoer
            ensure!(sender == place.clone().owner, "Requested place is not owned by the sender");
            ensure!(sender == Self::sudoer(place_id.clone()), "You are not sudoer to determine infeciton of this place");
            let visitor_ids: Vec<T::AccountId> = place.clone().visited;
            // set all visitors as infected
            for visitor_id in visitor_ids {
                let mut visitor= identifier::Module::<T>::id(visitor_id.clone());
                visitor.infected = true;
                identifier::Identifiers::<T>::mutate(visitor_id, |v| *v = visitor);
            }
            Self::deposit_event(RawEvent::PlaceInfected(place_id));
            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        PlaceRegistered(AccountId, u64),
        PlaceSudoUpdated(u64),
        PlaceInfected(u64),
    }
);
