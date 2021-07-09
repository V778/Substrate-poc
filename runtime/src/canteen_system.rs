use support::{ decl_module, decl_storage, decl_event, dispatch::Result,
    StorageValue, StorageMap, ensure, traits::{ Currency, ReservableCurrency } };

  
   use rstd::prelude::*;

  
  pub type StdResult<T> = rstd::result::Result<T, &'static str>;
  
  pub trait Trait: timestamp::Trait + balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
  }
  
 
  const ORDER_FOOD: u32 = 3;
    
  #[derive(Encode, Decode)]
  pub enum CanteenStatus {
    On,
    Off,
  }
  
  impl Default for CanteenStatus {
    fn default() -> Self { CanteenStatus::On }
  }
  
  #[derive(Encode, Decode)]
  pub enum FoodStatus {
   FastFood,
   Meal,
  }

  impl Default for FoodStatus {
    fn default() -> Self { FoodStatus::Active }
  }
  
  
  #[derive(Encode, Decode)]
  pub struct Customer<Hash, AccountId> {
    id: Hash,
    name: Option<Vec<u8>>,
    Address: Option<AccountId>,
    m_no: Option<u64>,
    city: Option<u64>,
    in_canteen: bool,
  }
  
  #[derive(Encode, Decode)]
  pub struct Menu<Hash, MenuTx> {
  
    paneer_dish: Option<Vec<u8>>,
    bhendi_dish: Option<Vec<u8>>,
    palak_dish: Option<Vec<u8>>,
    soya_dish: Option<Vec<u8>>,
    gobi_dish: Option<Vec<u8>>,
    tea: Option<Vec<u8>>,
    coffee: Option<Vec<u8>>,

  

  }
  
  #[derive(Encode, Decode)]
  pub struct Food<Hash, AccountId> {
   
    canteen_id: Hash,

    status: FoodStatus,
  }
  
  
  decl_storage! {
    trait Store for Module<T: Trait> as CanteenSystem {
     
    }
  }
  
  decl_event!(
    pub enum Event<T> where
      <T as system::Trait>::AccountId,
      <T as system::Trait>::Hash,
      <T as balances::Trait>::Balance,
       {
  
      AccountCreated(AccountId, Hash, Vec<u8>),
      Ordering(AccountId, Hash, Hash, Balance, Moment),
      OrderCancelled(Hash),
      
      CanteenTx(Hash, Hash, AccountId, AccountId),
    }
  );
  
  decl_module! {
    
  
  }
        
       