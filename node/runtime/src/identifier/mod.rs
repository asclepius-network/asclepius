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
pub struct Identifier {
    pub infected: bool,
    /// hash from issuer identity public key
    pub visited: Vec<u64>,
}

impl Identifier {
    pub fn new(infected: bool, visited: Vec<u64>) -> Self {
        Identifier { infected, visited }
    }
}

/// Data storage type for each account
#[derive(Encode, Decode, Default, Clone, Eq, PartialEq)]
pub struct Log<Moment> {
    /// Visited Time
    pub when: Moment,
    /// Reference Identifier for place description
    pub place_id: u64,
}

impl<Moment> Log<Moment> {
    pub fn new(when: Moment, place_id: u64) -> Self {
        Log { when, place_id }
    }
}

// Module's function and Methods of custom struct to be placed here
impl<T: Trait> Module<T> {}

/// The module's configuration trait.
pub trait Trait: system::Trait + timestamp::Trait {
    // TODO: Add other types and constants required configure this module.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as DID {
        pub Identifiers get(id): map T::AccountId => Identifier;
        pub Logs get(log): map (T::AccountId, u64) => Log<T::Moment>;
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
        pub fn register_id(origin) -> Result {
            let registrar = ensure_signed(origin)?;
            ensure!(!<Identifiers<T>>::exists(registrar.clone()), "The identifier already exists");
            let places: Vec<u64> = vec![];
            let id = Identifier::new(false, places);
            <Identifiers<T>>::insert(registrar.clone(), id);
            Self::deposit_event(RawEvent::IdRegistered(registrar));
            Ok(())
        }

        #[weight = SimpleDispatchInfo::FixedNormal(0)]
        pub fn update(origin, id: T::AccountId, infected: bool) -> Result {
            let updater = ensure_signed(origin)?;
            ensure!(<Identifiers<T>>::exists(id.clone()), "Identifier is not registered");
            let mut identifier = Self::id(id.clone());
            // TODO: Ensure if the place is infected
            ///////////////
            identifier.infected = infected;
            // Update identifier
            <Identifiers<T>>::mutate(id.clone(), |id| *id = identifier);
            Self::deposit_event(RawEvent::IdUpdated(id, infected));
            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        IdRegistered(AccountId),
        IdUpdated(AccountId, bool),
    }
);
