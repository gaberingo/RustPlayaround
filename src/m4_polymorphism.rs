use ethers::types::Address;
use std::{ops::Add, str::FromStr};

trait EthereumAddress{
    fn convert_address(&self) -> Result<Address, &str>;
}

impl EthereumAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(&self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid Ethereum Address"),
        }
    }
}

impl EthereumAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}

fn get_ethereum_address<T: EthereumAddress>(address: T) -> Address{
    address.convert_address().unwrap()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_polymorphism(){
        let conv_address1 = get_ethereum_address("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5");
        println!("{}", conv_address1);

        let conv_address2 = get_ethereum_address(conv_address1);
        println!("{}", conv_address2);

        let conv_address3 = Address::from_str("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5");
        println!("{}", conv_address3.unwrap());

        let address_4 = "0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5".convert_address();
        println!("{}", address_4.unwrap());
    }
}