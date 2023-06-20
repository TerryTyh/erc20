#![cfg_attr(not(feature = "std"), no_std,no_main)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;

    //Erc20 数据结构
    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }


    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        BalanceTooLow,
        AllowanceTooLow,
    }
    //声明方法所返回的结果
    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        // 构造函数，初始化时只运行一次
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::new();
            balances.insert(Self::env().caller(),&total_supply);
            Self {
                total_supply,
                balances,
                ..Default::default()
            }
        }
        // 查询total_supply
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }
        // 返回某账户的Balance
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }
        // 操作自身转账
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let sender = self.env().caller();
            return self.transfer_from_to(&sender, &to,value);
            
        }
        // 操作第三方账号转账
        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId,value: Balance)-> Result<()> {
            let sender = self.env().caller();
            let mut allowance = self.allowances.get(&(from,sender)).unwrap_or_default();
            
            if allowance < value {
                return Err(Error::AllowanceTooLow);
            }

            self.allowances.insert((from,sender),&(allowance - value));

            return self.transfer_from_to(&from, &to, value);
        }

        //授权第三方可操作的余额
        #[ink(message)]
        pub fn approve(&mut self,to: AccountId, value: Balance) -> Result<()> {
            let sender = self.env().caller();
            self.allowances.insert(&(sender,to),&value);
            self.env().emit_event(Approval{
                owner: sender,
                spender: to,
                value
            });

            Ok(())

        }

        // 转账方法
        pub fn transfer_from_to(&mut self, from: &AccountId, to: &AccountId, value: Balance) -> Result<()>{
            let transfer_from = self.balance_of(*from);
            let transfer_to = self.balance_of(*to);

            if value > transfer_from {
                return Err(Error::BalanceTooLow);
            }

            self.balances.insert(from,&(transfer_from - value));
            self.balances.insert(to, &(transfer_to + value));

            self.env().emit_event(
                Transfer {
                    from: *from,
                    to: *to,
                    value
                }
            );
            Ok(())
        }


    // #[cfg(test)]
    // mod tests {
    //     use super::*;
    //     use ink_lang as ink;
    
    //     #[ink::test]
    //     fn new_works() {
    //         let contract = Erc20::new(777);
    //         assert_eq!(contract.total_supply(), 777);
    //     }
    
    //     #[ink::test]
    //     fn balance_works() {
    //         let contract = Erc20::new(100);
    //         assert_eq!(contract.total_supply(), 100);
    //         assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
    //         assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
    //     }

    //     #[ink::test]
    //     fn transfer_works() {
    //         let mut erc20 = Erc20::new(100);
    //         assert_eq!(erc20.balance_of(AccountId::from([0x0; 32])), 0);
    //         assert_eq!(erc20.transfer(AccountId::from([0x0; 32]), 10), Ok(()));
    //         assert_eq!(erc20.balance_of(AccountId::from([0x0; 32])), 10);
    //     }

    //     #[ink::test]
    //     fn transfer_from_works() {
    //         let mut contract = Erc20::new(100);
    //         assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
    //         contract.approve(AccountId::from([0x1; 32]), 20);
    //         contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 10);
    //         assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
    //     }

    //     #[ink::test]
    //     fn allowances_works() {
    //         let mut contract = Erc20::new(100);
    //         assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
    //         contract.approve(AccountId::from([0x1; 32]), 200);
    //         assert_eq!(contract.allowance(AccountId::from([0x1; 32]), AccountId::from([0x1; 32])), 200);

    //         contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 50);
    //         assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 50);
    //         assert_eq!(contract.allowance(AccountId::from([0x1; 32]), AccountId::from([0x1; 32])), 150);

    //         contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 100);
    //         assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 50);
    //         assert_eq!(contract.allowance(AccountId::from([0x1; 32]), AccountId::from([0x1; 32])), 150);
    //     }

    }
}